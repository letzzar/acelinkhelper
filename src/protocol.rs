use crate::error::{AcelinkError, Result};
use std::path::PathBuf;

#[cfg(target_os = "macos")]
use std::fs;

/// Registra el protocolo `acestream://` en el sistema operativo
/// Permite que los enlaces acestream:// se abran automáticamente con esta aplicación
pub fn register_protocol(app_path: PathBuf) -> Result<()> {
    #[cfg(windows)]
    return register_protocol_windows(app_path);

    #[cfg(target_os = "macos")]
    return register_protocol_macos(app_path);

    #[cfg(not(any(windows, target_os = "macos")))]
    {
        log::warn!("Registro de protocolo no soportado en esta plataforma");
        Ok(())
    }
}

#[cfg(windows)]
fn register_protocol_windows(app_path: PathBuf) -> Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

    // Crear la entrada para acestream://
    let (acestream_key, _) = hkcr
        .create_subkey("acestream")
        .map_err(|e| AcelinkError::ProtocolRegistrationError(
            format!("No se pudo crear clave 'acestream': {}", e)
        ))?;

    acestream_key
        .set_value("", &"URL:acestream Protocol")
        .map_err(|e| AcelinkError::ProtocolRegistrationError(
            format!("No se pudo establecer valor por defecto: {}", e)
        ))?;

    acestream_key
        .set_value("URL Protocol", &"")
        .map_err(|e| AcelinkError::ProtocolRegistrationError(
            format!("No se pudo establecer URL Protocol: {}", e)
        ))?;

    // Crear la entrada shell\open\command
    let (cmd_key, _) = acestream_key
        .create_subkey("shell\\open\\command")
        .map_err(|e| AcelinkError::ProtocolRegistrationError(
            format!("No se pudo crear clave shell\\open\\command: {}", e)
        ))?;

    // Establecer el comando que se ejecutará
    let app_path_str = app_path
        .to_str()
        .ok_or_else(|| AcelinkError::ProtocolRegistrationError(
            "Ruta de aplicación contiene caracteres inválidos".to_string()
        ))?;

    let command = format!("\"{}\" \"%1\"", app_path_str);
    cmd_key
        .set_value("", &command)
        .map_err(|e| AcelinkError::ProtocolRegistrationError(
            format!("No se pudo registrar comando: {}", e)
        ))?;

    log::info!("Protocolo acestream:// registrado correctamente en Windows");
    Ok(())
}

#[cfg(target_os = "macos")]
fn register_protocol_macos(app_path: PathBuf) -> Result<()> {
    // En macOS, el registro del protocolo se hace a través del Info.plist
    // Este es el código para documentar el proceso
    // En una app empaquetada como .app, el plist está en:
    // MyApp.app/Contents/Info.plist

    let bundle_path = app_path
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| AcelinkError::ProtocolRegistrationError(
            "No se pudo determinar la ruta del bundle".to_string()
        ))?;

    let plist_path = bundle_path.join("Contents/Info.plist");

    if !plist_path.exists() {
        log::warn!(
            "Info.plist no encontrado en: {:?}. Se requiere registro manual del protocolo.",
            plist_path
        );
        return Ok(());
    }

    // Leer el archivo plist actual
    let plist_content = fs::read_to_string(&plist_path)
        .map_err(|e| AcelinkError::ProtocolRegistrationError(
            format!("No se pudo leer Info.plist: {}", e)
        ))?;

    // Verificar si ya existe la entrada del protocolo
    if plist_content.contains("acestream") {
        log::info!("Protocolo acestream:// ya está registrado en macOS");
        return Ok(());
    }

    // Crear la entrada del protocolo en XML (formato plist)
    let protocol_entry = r#"	<key>CFBundleURLTypes</key>
	<array>
		<dict>
			<key>CFBundleURLName</key>
			<string>Acestream Protocol Handler</string>
			<key>CFBundleURLSchemes</key>
			<array>
				<string>acestream</string>
			</array>
		</dict>
	</array>"#;

    // Insertar antes de </dict> final
    let new_content = if let Some(pos) = plist_content.rfind("</dict>") {
        let (before, after) = plist_content.split_at(pos);
        format!("{}\n{}\n{}", before, protocol_entry, after)
    } else {
        log::error!("Formato de plist no reconocido");
        return Err(AcelinkError::ProtocolRegistrationError(
            "El archivo plist no tiene el formato esperado".to_string(),
        ));
    };

    // Escribir el plist actualizado
    fs::write(&plist_path, new_content).map_err(|e| {
        AcelinkError::ProtocolRegistrationError(format!("No se pudo escribir Info.plist: {}", e))
    })?;

    log::info!("Protocolo acestream:// registrado correctamente en macOS");
    Ok(())
}

#[cfg(not(any(windows, target_os = "macos")))]
fn register_protocol_linux(_app_path: PathBuf) -> Result<()> {
    log::warn!("Registro de protocolo no implementado para Linux");
    Ok(())
}

