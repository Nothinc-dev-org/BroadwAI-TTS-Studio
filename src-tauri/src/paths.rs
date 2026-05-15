//! Rutas estándar de un proyecto BroadwAI.
//!
//! ```text
//! <root>/
//!   database/project.sqlite
//!   audio/
//!     generated/
//!     exports/
//!   assets/
//!     sfx/
//!     music/
//!     ambience/
//!   imports/
//!   cache/
//! ```

use std::path::{Path, PathBuf};

use crate::error::{AppError, AppResult};

pub struct ProjectPaths {
    pub root: PathBuf,
}

impl ProjectPaths {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn database_dir(&self) -> PathBuf {
        self.root.join("database")
    }

    pub fn database_file(&self) -> PathBuf {
        self.database_dir().join("project.sqlite")
    }

    pub fn audio_dir(&self) -> PathBuf {
        self.root.join("audio")
    }

    pub fn generated_audio_dir(&self) -> PathBuf {
        self.audio_dir().join("generated")
    }

    pub fn exports_dir(&self) -> PathBuf {
        self.audio_dir().join("exports")
    }

    pub fn assets_dir(&self) -> PathBuf {
        self.root.join("assets")
    }

    pub fn sfx_dir(&self) -> PathBuf {
        self.assets_dir().join("sfx")
    }

    pub fn music_dir(&self) -> PathBuf {
        self.assets_dir().join("music")
    }

    pub fn ambience_dir(&self) -> PathBuf {
        self.assets_dir().join("ambience")
    }

    pub fn imports_dir(&self) -> PathBuf {
        self.root.join("imports")
    }

    pub fn cache_dir(&self) -> PathBuf {
        self.root.join("cache")
    }

    /// Crea todas las carpetas necesarias para un proyecto nuevo.
    pub fn create_all(&self) -> AppResult<()> {
        for dir in [
            self.database_dir(),
            self.generated_audio_dir(),
            self.exports_dir(),
            self.sfx_dir(),
            self.music_dir(),
            self.ambience_dir(),
            self.imports_dir(),
            self.cache_dir(),
        ] {
            std::fs::create_dir_all(&dir)?;
        }
        Ok(())
    }

    /// Verifica que la estructura básica exista (RF-02).
    pub fn validate(&self) -> AppResult<()> {
        for required in [self.database_file(), self.audio_dir(), self.assets_dir()] {
            if !required.exists() {
                return Err(AppError::invalid(format!(
                    "estructura de proyecto incompleta: falta {}",
                    required.display()
                )));
            }
        }
        Ok(())
    }
}

impl AsRef<Path> for ProjectPaths {
    fn as_ref(&self) -> &Path {
        &self.root
    }
}
