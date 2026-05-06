/// Módulo para detectar el idioma del sistema operativo
/// Funciona en Windows y macOS

use crate::config::Language;

/// Detecta el idioma del sistema operativo
/// Retorna Spanish si el sistema está en español, English para otros idiomas
pub fn detect_system_language() -> Language {
    #[cfg(windows)]
    {
        detect_windows_language()
    }
    
    #[cfg(target_os = "macos")]
    {
        detect_macos_language()
    }
    
    #[cfg(not(any(windows, target_os = "macos")))]
    {
        // Para otras plataformas, por defecto inglés
        Language::English
    }
}

/// Detecta el idioma en Windows usando variables de entorno o valores predeterminados
#[cfg(windows)]
fn detect_windows_language() -> Language {
    // Intentar obtener el idioma del registro de Windows
    #[cfg(target_os = "windows")]
    {
        use std::env;
        
        // Intentar obtener del environment variable LANG
        if let Ok(lang) = env::var("LANG") {
            if lang.starts_with("es") {
                log::info!("Idioma del sistema detectado: Español (LANG)");
                return Language::Spanish;
            }
        }
        
        // Intentar obtener LANGUAGE
        if let Ok(lang) = env::var("LANGUAGE") {
            if lang.starts_with("es") {
                log::info!("Idioma del sistema detectado: Español (LANGUAGE)");
                return Language::Spanish;
            }
        }
        
        // Intentar obtener LC_ALL
        if let Ok(lang) = env::var("LC_ALL") {
            if lang.starts_with("es") {
                log::info!("Idioma del sistema detectado: Español (LC_ALL)");
                return Language::Spanish;
            }
        }
        
        // En Windows, intentar leer del registro
        if let Ok(lang_code) = get_windows_locale() {
            if lang_code.starts_with("es") {
                log::info!("Idioma del sistema detectado: Español (Registro Windows)");
                return Language::Spanish;
            }
        }
    }
    
    log::info!("Idioma del sistema detectado: English (por defecto)");
    Language::English
}

/// Detecta el idioma en macOS
#[cfg(target_os = "macos")]
fn detect_macos_language() -> Language {
    use std::process::Command;
    
    // En macOS, usar 'defaults read' para obtener AppleLanguages
    if let Ok(output) = Command::new("defaults")
        .args(&["read", "-g", "AppleLanguages"])
        .output()
    {
        if output.status.success() {
            let langs = String::from_utf8_lossy(&output.stdout);
            
            // Buscar "es" en la salida (por ejemplo "es_ES" o simplemente "es")
            if langs.contains("es") {
                log::info!("Idioma del sistema detectado: Español (macOS defaults)");
                return Language::Spanish;
            }
        }
    }
    
    // Intentar con variables de entorno como fallback
    if let Ok(lang) = std::env::var("LANG") {
        if lang.starts_with("es") {
            log::info!("Idioma del sistema detectado: Español (LANG env)");
            return Language::Spanish;
        }
    }
    
    log::info!("Idioma del sistema detectado: English (por defecto)");
    Language::English
}

/// Lee el código de idioma del registro de Windows
#[cfg(windows)]
fn get_windows_locale() -> std::result::Result<String, Box<dyn std::error::Error>> {
    use winreg::RegKey;
    use winreg::enums::HKEY_CURRENT_USER;
    
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let config = hkcu.open_subkey("Control Panel\\International")?;
    
    // Intentar obtener LocaleName (Win7+)
    if let Ok(locale) = config.get_value::<String, &str>("LocaleName") {
        log::debug!("LocaleName del registro: {}", locale);
        return Ok(locale);
    }
    
    // Fallback a iLanguage (más antiguo)
    if let Ok(lang_id) = config.get_value::<String, &str>("iLanguage") {
        log::debug!("iLanguage del registro: {}", lang_id);
        return Ok(lang_id);
    }
    
    Err("No se pudo obtener el idioma del registro".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_language() {
        let lang = detect_system_language();
        println!("Sistema en idioma: {:?}", lang);
        // Simplemente imprimir para verificación manual
    }
}
