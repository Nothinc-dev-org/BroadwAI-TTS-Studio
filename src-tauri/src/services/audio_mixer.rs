//! AudioMixer — mezcla de voces + assets para `Play global` y export
//! (RF-26, RF-27, RF-28, RF-34, RF-35, RF-36, RF-37).
//!
//! Pipeline:
//!   1. Decodifica cada fuente (WAV/MP3/OGG/FLAC) a `f32` mono @ 24 kHz
//!      usando `symphonia`. El resample es interpolación lineal — suficiente
//!      para mezcla MVP; sustituible por `rubato` cuando se requiera calidad.
//!   2. Compone la pista de voces concatenando clips con `default_gap_ms`
//!      y `before/after_delay_ms` por nodo (RF-35).
//!   3. Compone cada pista de assets sumando clips en su `start_ms` absoluto.
//!   4. Aplica volumen/mute/solo por pista y suma todo a un buffer master.
//!   5. Escribe WAV mono 16-bit con `hound`.

use std::fs::{self, File};
use std::path::{Path, PathBuf};

use hound::{SampleFormat, WavSpec, WavWriter};
use symphonia::core::audio::{AudioBufferRef, SampleBuffer};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::error::{AppError, AppResult};

pub const TARGET_SAMPLE_RATE: u32 = 24_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Wav,
    Mp3,
}

impl ExportFormat {
    pub fn parse(value: &str) -> AppResult<Self> {
        match value.to_ascii_lowercase().as_str() {
            "wav" => Ok(ExportFormat::Wav),
            "mp3" => Ok(ExportFormat::Mp3),
            other => Err(AppError::invalid(format!("formato no soportado: {other}"))),
        }
    }
}

pub struct DialogueClip<'a> {
    pub source_path: &'a Path,
    pub before_delay_ms: i32,
    pub after_delay_ms: i32,
    pub volume: f32,
    pub fade_in_ms: i32,
    pub fade_out_ms: i32,
}

pub struct AssetClip<'a> {
    pub source_path: &'a Path,
    pub start_ms: i32,
    pub volume: f32,
    pub fade_in_ms: i32,
    pub fade_out_ms: i32,
    pub looping: bool,
    pub duration_ms: Option<i32>,
}

pub struct VoiceTrack<'a> {
    pub clips: &'a [DialogueClip<'a>],
    pub default_gap_ms: i32,
    pub volume: f32,
    pub muted: bool,
    pub solo: bool,
}

pub struct AssetTrack<'a> {
    pub clips: &'a [AssetClip<'a>],
    pub volume: f32,
    pub muted: bool,
    pub solo: bool,
}

pub struct MixRequest<'a> {
    pub voice_track: Option<VoiceTrack<'a>>,
    pub asset_tracks: &'a [AssetTrack<'a>],
    pub output: &'a Path,
    pub format: ExportFormat,
}

pub struct MixSummary {
    pub output_path: PathBuf,
    pub duration_ms: u64,
}

pub fn render_mix(req: MixRequest<'_>) -> AppResult<MixSummary> {
    if matches!(req.format, ExportFormat::Mp3) {
        return Err(AppError::NotImplemented("export MP3"));
    }

    let any_solo = req.voice_track.as_ref().map(|v| v.solo).unwrap_or(false)
        || req.asset_tracks.iter().any(|t| t.solo);

    let voice_buffer = match &req.voice_track {
        Some(track) if include_track(track.muted, track.solo, any_solo) => {
            render_voice_track(track)?
        }
        _ => Vec::new(),
    };

    let mut asset_buffers: Vec<Vec<f32>> = Vec::with_capacity(req.asset_tracks.len());
    for track in req.asset_tracks {
        if !include_track(track.muted, track.solo, any_solo) {
            asset_buffers.push(Vec::new());
            continue;
        }
        asset_buffers.push(render_asset_track(track)?);
    }

    let mut total_frames = voice_buffer.len();
    for buf in &asset_buffers {
        total_frames = total_frames.max(buf.len());
    }
    if total_frames == 0 {
        return Err(AppError::invalid("la mezcla no contiene clips audibles"));
    }

    let mut master = vec![0.0_f32; total_frames];
    if !voice_buffer.is_empty() {
        let gain = req.voice_track.as_ref().map(|t| t.volume).unwrap_or(1.0);
        accumulate(&mut master, &voice_buffer, gain);
    }
    for (buf, track) in asset_buffers.iter().zip(req.asset_tracks.iter()) {
        if buf.is_empty() {
            continue;
        }
        accumulate(&mut master, buf, track.volume);
    }

    if let Some(parent) = req.output.parent() {
        fs::create_dir_all(parent)?;
    }
    write_mono_wav(req.output, TARGET_SAMPLE_RATE, &master)?;
    let duration_ms = master.len() as u64 * 1000 / TARGET_SAMPLE_RATE as u64;
    Ok(MixSummary {
        output_path: req.output.to_path_buf(),
        duration_ms,
    })
}

fn include_track(muted: bool, solo: bool, any_solo: bool) -> bool {
    if muted {
        return false;
    }
    if any_solo {
        solo
    } else {
        true
    }
}

fn render_voice_track(track: &VoiceTrack<'_>) -> AppResult<Vec<f32>> {
    let mut buffer: Vec<f32> = Vec::new();
    for (index, clip) in track.clips.iter().enumerate() {
        if index > 0 {
            append_silence(&mut buffer, track.default_gap_ms);
        }
        append_silence(&mut buffer, clip.before_delay_ms);

        let samples = decode_to_mono(clip.source_path)?;
        let samples = apply_fades(samples, clip.fade_in_ms, clip.fade_out_ms);
        let samples = apply_volume(samples, clip.volume);
        buffer.extend(samples);

        append_silence(&mut buffer, clip.after_delay_ms);
    }
    Ok(buffer)
}

fn render_asset_track(track: &AssetTrack<'_>) -> AppResult<Vec<f32>> {
    let mut buffer: Vec<f32> = Vec::new();
    for clip in track.clips {
        let raw = decode_to_mono(clip.source_path)?;
        let cropped = crop_or_loop(raw, clip.duration_ms, clip.looping);
        let with_fades = apply_fades(cropped, clip.fade_in_ms, clip.fade_out_ms);
        let with_volume = apply_volume(with_fades, clip.volume);
        let start_frame = ms_to_frames(clip.start_ms);
        let needed = start_frame + with_volume.len();
        if buffer.len() < needed {
            buffer.resize(needed, 0.0);
        }
        for (i, sample) in with_volume.into_iter().enumerate() {
            buffer[start_frame + i] += sample;
        }
    }
    Ok(buffer)
}

fn accumulate(master: &mut [f32], track: &[f32], gain: f32) {
    let limit = master.len().min(track.len());
    if (gain - 1.0).abs() < f32::EPSILON {
        for i in 0..limit {
            master[i] += track[i];
        }
    } else {
        for i in 0..limit {
            master[i] += track[i] * gain;
        }
    }
}

fn crop_or_loop(samples: Vec<f32>, duration_ms: Option<i32>, looping: bool) -> Vec<f32> {
    let Some(target_frames) = duration_ms.map(ms_to_frames) else {
        return samples;
    };
    if samples.is_empty() {
        return samples;
    }
    if looping {
        let mut out = Vec::with_capacity(target_frames);
        while out.len() < target_frames {
            let remaining = target_frames - out.len();
            if remaining >= samples.len() {
                out.extend_from_slice(&samples);
            } else {
                out.extend_from_slice(&samples[..remaining]);
            }
        }
        out
    } else if samples.len() > target_frames {
        samples[..target_frames].to_vec()
    } else {
        samples
    }
}

fn append_silence(buffer: &mut Vec<f32>, ms: i32) {
    if ms <= 0 {
        return;
    }
    buffer.extend(std::iter::repeat(0.0_f32).take(ms_to_frames(ms)));
}

fn ms_to_frames(ms: i32) -> usize {
    if ms <= 0 {
        0
    } else {
        (ms as u64 * TARGET_SAMPLE_RATE as u64 / 1000) as usize
    }
}

fn apply_volume(mut samples: Vec<f32>, volume: f32) -> Vec<f32> {
    if (volume - 1.0).abs() < f32::EPSILON {
        return samples;
    }
    for s in &mut samples {
        *s *= volume;
    }
    samples
}

fn apply_fades(mut samples: Vec<f32>, fade_in_ms: i32, fade_out_ms: i32) -> Vec<f32> {
    let total = samples.len();
    if total == 0 {
        return samples;
    }
    let fade_in = ms_to_frames(fade_in_ms).min(total);
    if fade_in > 0 {
        for i in 0..fade_in {
            let factor = i as f32 / fade_in as f32;
            samples[i] *= factor;
        }
    }
    let fade_out = ms_to_frames(fade_out_ms).min(total);
    if fade_out > 0 {
        for i in 0..fade_out {
            let idx = total - 1 - i;
            let factor = i as f32 / fade_out as f32;
            samples[idx] *= factor;
        }
    }
    samples
}

fn write_mono_wav(path: &Path, sample_rate: u32, samples: &[f32]) -> AppResult<()> {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut writer = WavWriter::create(path, spec)
        .map_err(|e| AppError::internal(format!("hound create: {e}")))?;
    for &s in samples {
        let clamped = s.clamp(-1.0, 1.0);
        let value = (clamped * i16::MAX as f32) as i16;
        writer
            .write_sample(value)
            .map_err(|e| AppError::internal(format!("hound write: {e}")))?;
    }
    writer
        .finalize()
        .map_err(|e| AppError::internal(format!("hound finalize: {e}")))?;
    Ok(())
}

pub fn decode_to_mono(path: &Path) -> AppResult<Vec<f32>> {
    let (samples, sample_rate, channels) = decode_with_symphonia(path)?;
    let mono = downmix_to_mono(samples, channels);
    if sample_rate == TARGET_SAMPLE_RATE {
        Ok(mono)
    } else {
        Ok(resample_linear(mono, sample_rate, TARGET_SAMPLE_RATE))
    }
}

pub fn probe_duration_ms(path: &Path) -> AppResult<Option<i32>> {
    let (samples, sample_rate, channels) = decode_with_symphonia(path)?;
    if sample_rate == 0 || channels == 0 {
        return Ok(None);
    }
    let frames = samples.len() as u64 / channels as u64;
    let ms = frames * 1000 / sample_rate as u64;
    Ok(Some(ms as i32))
}

fn decode_with_symphonia(path: &Path) -> AppResult<(Vec<f32>, u32, u16)> {
    let file = File::open(path)
        .map_err(|e| AppError::internal(format!("open {}: {e}", path.display())))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        hint.with_extension(ext);
    }
    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| AppError::internal(format!("probe {}: {e}", path.display())))?;
    let mut format = probed.format;
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| {
            AppError::internal(format!("sin pistas decodificables: {}", path.display()))
        })?;
    let track_id = track.id;
    let codec_params = track.codec_params.clone();
    let sample_rate = codec_params.sample_rate.unwrap_or(48_000);
    let channels = codec_params
        .channels
        .map(|c| c.count() as u16)
        .unwrap_or(1)
        .max(1);
    let mut decoder = symphonia::default::get_codecs()
        .make(&codec_params, &DecoderOptions::default())
        .map_err(|e| AppError::internal(format!("codec init: {e}")))?;

    let mut samples: Vec<f32> = Vec::new();
    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(SymphoniaError::IoError(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                break
            }
            Err(SymphoniaError::ResetRequired) => break,
            Err(e) => return Err(AppError::internal(format!("packet: {e}"))),
        };
        if packet.track_id() != track_id {
            continue;
        }
        match decoder.decode(&packet) {
            Ok(decoded) => append_decoded(&mut samples, decoded),
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(e) => return Err(AppError::internal(format!("decode: {e}"))),
        }
    }
    Ok((samples, sample_rate, channels))
}

fn append_decoded(out: &mut Vec<f32>, decoded: AudioBufferRef<'_>) {
    let spec = *decoded.spec();
    let frames = decoded.frames();
    let mut sample_buf = SampleBuffer::<f32>::new(frames as u64, spec);
    sample_buf.copy_interleaved_ref(decoded);
    out.extend_from_slice(sample_buf.samples());
}

fn downmix_to_mono(samples: Vec<f32>, channels: u16) -> Vec<f32> {
    if channels <= 1 {
        return samples;
    }
    let chan = channels as usize;
    samples
        .chunks(chan)
        .map(|frame| frame.iter().sum::<f32>() / chan as f32)
        .collect()
}

fn resample_linear(input: Vec<f32>, from_sr: u32, to_sr: u32) -> Vec<f32> {
    if input.is_empty() || from_sr == to_sr {
        return input;
    }
    let ratio = to_sr as f64 / from_sr as f64;
    let out_len = ((input.len() as f64) * ratio).round() as usize;
    let mut out = Vec::with_capacity(out_len);
    for i in 0..out_len {
        let src = i as f64 / ratio;
        let idx = src.floor() as usize;
        let frac = (src - idx as f64) as f32;
        let a = input.get(idx).copied().unwrap_or(0.0);
        let b = input.get(idx + 1).copied().unwrap_or(a);
        out.push(a + (b - a) * frac);
    }
    out
}
