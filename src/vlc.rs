/// Módulo VLC - Busca y lanza VLC de forma multi-plataforma
use crate::error::{AcelinkError, Result};
use std::process::Command;

/// Lanza VLC con la URL especificada
pub fn launch_vlc(url: &str) -> Result<()> {
    log::info!("========== LANZANDO VLC ==========");
    log::info!("URL: {}", url);
    
    // Intentar rutas comunes según el SO
    let mut vlc_paths = vec![
        "vlc".to_string(),  // Si está en PATH
    ];

    // En Windows, agregar rutas estándar
    #[cfg(windows)]
    {
        vlc_paths.extend(vec![
            "C:\\Program Files\\VideoLAN\\VLC\\vlc.exe".to_string(),
            "C:\\Program Files (x86)\\VideoLAN\\VLC\\vlc.exe".to_string(),
        ]);
        
        // Intentar encontrar VLC usando 'where' en Windows
        if let Ok(output) = Command::new("where")
            .arg("vlc.exe")
            .output()
        {
            if output.status.success() {
                if let Ok(path) = String::from_utf8(output.stdout) {
                    let vlc_path = path.trim().to_string();
                    if !vlc_path.is_empty() {
                        log::info!("VLC encontrado en PATH: {}", vlc_path);
                        vlc_paths.insert(0, vlc_path);
                    }
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    let mut last_error_macos = String::new();

    #[cfg(target_os = "macos")]
    {
        // Usar Launch Services en macOS es más confiable para aplicaciones instaladas
        // como paquetes .app, incluso si no se tiene el binario en PATH.
        //
        // La URL se pasa como documento, NO detrás de --args: con --args solo
        // llega en el argv de main(), que ya se ejecutó si VLC estaba abierto,
        // así que el segundo enlace se perdía hasta cerrar VLC.
        //
        // Se comprueba el código de salida de 'open': spawn() tiene éxito
        // aunque VLC no esté instalado, y entonces nunca se probaba la ruta
        // directa de abajo.
        match Command::new("open")
            .arg("-a")
            .arg("VLC")
            .arg(url)
            .status()
        {
            Ok(status) if status.success() => {
                log::info!("✅ VLC lanzado exitosamente con 'open -a VLC'");
                log::info!("========== VLC EN EJECUCIÓN ==========");
                return Ok(());
            }
            Ok(status) => {
                log::warn!("'open -a VLC' terminó con {}. Intentando ruta directa...", status);
                last_error_macos = format!("'open -a VLC' terminó con {}", status);
            }
            Err(e) => {
                log::warn!("No se pudo ejecutar 'open': {}. Intentando ruta directa...", e);
                last_error_macos = format!("no se pudo ejecutar 'open': {}", e);
            }
        }

        vlc_paths.extend(vec![
            "/Applications/VLC.app/Contents/MacOS/VLC".to_string(),
        ]);
    }

    #[allow(unused_mut)]
    let mut last_error = String::from("VLC no encontrado en rutas estándar");
    #[cfg(target_os = "macos")]
    if !last_error_macos.is_empty() {
        last_error = last_error_macos;
    }

    for vlc_path in vlc_paths {
        log::info!("Intentando VLC en: {}", vlc_path);
        
        match Command::new(&vlc_path)
            .arg(url)
            .spawn()
        {
            Ok(child) => {
                log::info!("✅ VLC lanzado exitosamente desde: {}", vlc_path);
                log::info!("========== VLC EN EJECUCIÓN ==========");
                
                // Detach del proceso (no esperar)
                drop(child);
                return Ok(());
            }
            Err(e) => {
                last_error = format!("Fallo en {}: {}", vlc_path, e);
                log::warn!("{}", last_error);
                continue;
            }
        }
    }

    log::error!("❌ No se pudo lanzar VLC");
    log::error!("========== ERROR ==========");
    Err(AcelinkError::VlcError(last_error))
}
