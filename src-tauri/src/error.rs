use serde::{Serialize, Serializer};
use thiserror::Error;

/// Tipo de error unificado de la aplicación.
///
/// Implementa [`Serialize`] para poder devolverse desde un `#[tauri::command]`.
/// **Nunca** debe serializar contenido sensible (API keys, prompts crudos con
/// credenciales, etc.). Por ese motivo el variante `Credential` solo expone un
/// mensaje genérico.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("operación no implementada todavía: {0}")]
    NotImplemented(&'static str),

    #[error("entidad no encontrada: {0}")]
    NotFound(String),

    #[error("entrada inválida: {0}")]
    InvalidInput(String),

    #[error("error de base de datos: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("error de IO: {0}")]
    Io(#[from] std::io::Error),

    #[error("error de serialización JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("error HTTP: {0}")]
    Http(#[from] reqwest::Error),

    #[error("error del proveedor externo: {0}")]
    Provider(String),

    /// Mensaje genérico para no filtrar detalles del store de credenciales.
    #[error("error de credenciales")]
    Credential,

    #[error("error de configuración: {0}")]
    Config(String),

    #[error("error interno: {0}")]
    Internal(String),
}

impl AppError {
    pub fn from_keyring(err: keyring::Error) -> Self {
        // El error original solo se loggea internamente; al exterior viaja el
        // variante genérico para evitar fugas accidentales.
        tracing::warn!(target: "credential", "keyring error: {err}");
        AppError::Credential
    }

    pub fn internal(message: impl Into<String>) -> Self {
        AppError::Internal(message.into())
    }

    pub fn invalid(message: impl Into<String>) -> Self {
        AppError::InvalidInput(message.into())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
