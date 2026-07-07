use thiserror::Error;

#[derive(Error, Debug)]
pub enum AcelinkError {
    #[error("Error de configuración: {0}")]
    ConfigError(String),

    #[error("Error al cargar/guardar configuración: {0}")]
    ConfigyError(#[from] confy::ConfyError),

    #[error("Error al lanzar VLC: {0}")]
    VlcError(String),

    #[error("Error al registrar protocolo: {0}")]
    ProtocolRegistrationError(String),

    #[error("Error del sistema: {0}")]
    SystemError(String),

    #[cfg(windows)]
    #[error("Error del registro de Windows: {0}")]
    RegistryError(String),

    #[error("URL inválida: {0}")]
    InvalidUrl(String),

    #[error("Argumento inválido: {0}")]
    InvalidArgument(String),
}

pub type Result<T> = std::result::Result<T, AcelinkError>;
