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
    return register_protocol_linux(app_path);
}

#[cfg(windows)]
fn register_protocol_windows(app_path: PathBuf) -> Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    // HKCU\Software\Classes en lugar de HKEY_CLASSES_ROOT: un usuario sin
    // administrador no puede crear claves en HKCR ("acceso denegado") y el
    // registro fallaba en silencio. Windows combina HKCU\Software\Classes en
    // HKCR, asi que el protocolo queda registrado igual, solo para este usuario.
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Crear la entrada para acestream://
    let (acestream_key, _) = hkcu
        .create_subkey("Software\\Classes\\acestream")
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

/// Contenido del .desktop que asocia acestream:// a esta aplicacion en Linux
#[cfg(not(any(windows, target_os = "macos")))]
fn linux_desktop_entry(app_path: &str) -> String {
    format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=AcelinkHelper\n\
         Comment=Abre los enlaces acestream:// en VLC\n\
         Exec=\"{}\" %u\n\
         Icon=vlc\n\
         Terminal=false\n\
         Categories=AudioVideo;\n\
         MimeType=x-scheme-handler/acestream;\n",
        app_path
    )
}

#[cfg(not(any(windows, target_os = "macos")))]
fn register_protocol_linux(app_path: PathBuf) -> Result<()> {
    use std::fs;
    use std::process::Command;

    let app_path_str = app_path
        .to_str()
        .ok_or_else(|| AcelinkError::ProtocolRegistrationError(
            "Ruta de aplicación contiene caracteres inválidos".to_string()
        ))?;

    // $XDG_DATA_HOME/applications, o ~/.local/share/applications
    let data_home = std::env::var("XDG_DATA_HOME")
        .ok()
        .filter(|d| !d.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var("HOME").ok().map(|h| PathBuf::from(h).join(".local/share")))
        .ok_or_else(|| AcelinkError::ProtocolRegistrationError(
            "No se pudo obtener HOME".to_string()
        ))?;
    let apps_dir = data_home.join("applications");
    let desktop_path = apps_dir.join("acelinkhelper.desktop");
    let entry = linux_desktop_entry(app_path_str);

    // Se llama en cada arranque: solo se reescribe si ha cambiado
    if fs::read_to_string(&desktop_path).ok().as_deref() == Some(entry.as_str()) {
        log::info!("Protocolo acestream:// ya registrado en Linux");
        return Ok(());
    }

    fs::create_dir_all(&apps_dir).map_err(|e| AcelinkError::ProtocolRegistrationError(
        format!("No se pudo crear {:?}: {}", apps_dir, e)
    ))?;
    fs::write(&desktop_path, &entry).map_err(|e| AcelinkError::ProtocolRegistrationError(
        format!("No se pudo escribir {:?}: {}", desktop_path, e)
    ))?;

    // Asociar el esquema a este .desktop (si no hay xdg-mime, el MimeType del
    // .desktop ya basta en muchos escritorios)
    match Command::new("xdg-mime")
        .args(["default", "acelinkhelper.desktop", "x-scheme-handler/acestream"])
        .status()
    {
        Ok(status) if status.success() => {}
        Ok(status) => log::warn!("xdg-mime terminó con {}", status),
        Err(e) => log::warn!("No se pudo ejecutar xdg-mime: {}", e),
    }
    let _ = Command::new("update-desktop-database").arg(&apps_dir).status();

    log::info!("Protocolo acestream:// registrado correctamente en Linux: {:?}", desktop_path);
    Ok(())
}

#[cfg(all(test, not(any(windows, target_os = "macos"))))]
mod tests {
    use super::linux_desktop_entry;

    #[test]
    fn desktop_entry_asocia_el_esquema_acestream() {
        let entry = linux_desktop_entry("/home/u/.local/bin/acelinkhelper");
        assert!(entry.starts_with("[Desktop Entry]\n"));
        assert!(entry.contains("Exec=\"/home/u/.local/bin/acelinkhelper\" %u\n"));
        assert!(entry.contains("MimeType=x-scheme-handler/acestream;\n"));
    }
}

