/// Librería AcelinkHelper - Lógica de negocio independiente de la UI
/// Esta librería NO depende de egui ni de ningún framework de UI
/// Solo contiene: config, protocolo, transformación de URLs, búsqueda de VLC, etc.

pub mod config;
pub mod error;
pub mod protocol;
pub mod icons;
pub mod singleton;
pub mod vlc;
pub mod language_detection;

pub use config::Config;
pub use error::{Result, AcelinkError};
pub use language_detection::detect_system_language;

/// Transforma una URL acestream:// a HTTP
pub fn transform_acestream_url(url: &str, config: &Config) -> Result<String> {
    if !url.starts_with("acestream://") {
        return Err(AcelinkError::InvalidUrl(
            "URL no válida. Debe comenzar con 'acestream://'".to_string(),
        ));
    }

    config.validate()?;

    // Extraer el ID del contenido
    let content_id = url
        .replace("acestream://", "")
        .trim()
        .trim_end_matches('/')
        .to_string();

    if content_id.is_empty() {
        return Err(AcelinkError::InvalidUrl(
            "ID de contenido no puede estar vacío".to_string(),
        ));
    }

    // Construir la URL final
    let final_url = format!(
        "http://{}:6878/ace/getstream?id={}",
        config.nas_ip, content_id
    );

    log::info!(
        "Transformando acestream://{} -> {}",
        content_id,
        final_url
    );

    Ok(final_url)
}
