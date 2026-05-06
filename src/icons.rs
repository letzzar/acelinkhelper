use crate::error::{AcelinkError, Result};
use std::io::Cursor;
use tray_icon::icon::Icon;

/// Carga el icono de la bandeja del sistema desde PNG compilado
pub fn load_tray_icon() -> Result<Icon> {
    let png_bytes = include_bytes!("../res/tray.png");
    decode_png_to_icon(png_bytes)
}

/// Carga el icono de la aplicación desde PNG compilado
pub fn load_app_icon() -> Result<Icon> {
    let png_bytes = include_bytes!("../res/app.png");
    decode_png_to_icon(png_bytes)
}

/// Obtiene los bytes PNG del icono de la aplicación (para egui)
pub fn get_app_icon_bytes() -> &'static [u8] {
    include_bytes!("../res/app.png")
}

/// Obtiene los bytes PNG del icono pequeño de la bandeja (para egui)
pub fn get_tray_icon_bytes() -> &'static [u8] {
    include_bytes!("../res/tray.png")
}

/// Decodifica un PNG a datos RGBA válidos para el icono
fn decode_png_to_icon(png_bytes: &[u8]) -> Result<Icon> {
    use image::io::Reader;
    
    // Decodificar PNG desde bytes
    let reader = Reader::new(Cursor::new(png_bytes))
        .with_guessed_format()
        .map_err(|e| AcelinkError::SystemError(
            format!("Error detectando formato PNG: {}", e)
        ))?;

    let image = reader.decode()
        .map_err(|e| AcelinkError::SystemError(
            format!("Error decodificando PNG: {}", e)
        ))?;

    // Convertir a RGBA8
    let rgba_image = image.to_rgba8();
    let (width, height) = rgba_image.dimensions();
    let rgba_data = rgba_image.into_raw();

    log::debug!("Icono PNG cargado: {}x{} ({} bytes RGBA)", width, height, rgba_data.len());

    // Crear icono a partir de datos RGBA
    Icon::from_rgba(rgba_data, width, height)
        .map_err(|e| AcelinkError::SystemError(
            format!("Error creando icono: {}", e)
        ))
}
