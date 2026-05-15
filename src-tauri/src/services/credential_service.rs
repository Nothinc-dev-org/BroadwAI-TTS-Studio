//! Almacenamiento seguro de API keys vía keyring del SO.
//!
//! - Linux: libsecret (gnome-keyring / kwallet).
//! - macOS: Keychain.
//! - Windows: Credential Manager (DPAPI).
//!
//! Las API keys **nunca** se persisten en SQLite ni se envían al frontend.
//! El frontend solo recibe el estado ([`ApiKeyStatus`]). El backend Rust es el
//! único que lee la key para inyectarla en cabeceras HTTP cuando llama a
//! DeepSeek o Gemini.

use keyring::Entry;
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};
use crate::services::KEYRING_SERVICE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Provider {
    Deepseek,
    Gemini,
}

impl Provider {
    fn keyring_account(self) -> &'static str {
        match self {
            Provider::Deepseek => "deepseek",
            Provider::Gemini => "gemini",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiKeyStatus {
    NotConfigured,
    Configured,
    Valid,
    Invalid,
    ConnectionError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyState {
    pub provider: Provider,
    pub status: ApiKeyStatus,
}

#[derive(Default)]
pub struct CredentialService;

impl CredentialService {
    pub fn new() -> Self {
        Self
    }

    fn entry(provider: Provider) -> AppResult<Entry> {
        Entry::new(KEYRING_SERVICE, provider.keyring_account()).map_err(AppError::from_keyring)
    }

    /// Persiste una API key en el keyring del SO.
    ///
    /// El parámetro `key` se recibe por referencia y **no** se loggea bajo
    /// ningún concepto. La cadena debe sobreescribirse en el llamador si se
    /// quiere reducir su tiempo de vida en memoria.
    pub fn set(&self, provider: Provider, key: &str) -> AppResult<()> {
        if key.trim().is_empty() {
            return Err(AppError::invalid("la API key no puede estar vacía"));
        }
        let entry = Self::entry(provider)?;
        entry.set_password(key).map_err(AppError::from_keyring)?;
        Ok(())
    }

    /// Devuelve la key cruda. Solo debe llamarse desde servicios que vayan a
    /// usarla inmediatamente para construir una petición HTTP. **Nunca** debe
    /// devolverse al frontend.
    pub(crate) fn read(&self, provider: Provider) -> AppResult<String> {
        let entry = Self::entry(provider)?;
        match entry.get_password() {
            Ok(value) => Ok(value),
            Err(keyring::Error::NoEntry) => Err(AppError::invalid(
                "API key no configurada para el proveedor",
            )),
            Err(other) => Err(AppError::from_keyring(other)),
        }
    }

    pub fn delete(&self, provider: Provider) -> AppResult<()> {
        let entry = Self::entry(provider)?;
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(other) => Err(AppError::from_keyring(other)),
        }
    }

    pub fn has(&self, provider: Provider) -> AppResult<bool> {
        let entry = Self::entry(provider)?;
        match entry.get_password() {
            Ok(_) => Ok(true),
            Err(keyring::Error::NoEntry) => Ok(false),
            Err(other) => Err(AppError::from_keyring(other)),
        }
    }

    pub fn status(&self, provider: Provider) -> AppResult<ApiKeyStatus> {
        if self.has(provider)? {
            Ok(ApiKeyStatus::Configured)
        } else {
            Ok(ApiKeyStatus::NotConfigured)
        }
    }
}
