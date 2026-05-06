use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub nas_ip: String,
    pub vlc_path: Option<String>,
    pub language: Language,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq)]
pub enum Language {
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "en")]
    English,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            nas_ip: "192.168.1.100".to_string(),
            vlc_path: None,
            language: Language::English,
        }
    }
}

impl Config {
    /// Obtiene la ruta de configuración según el SO
    #[allow(dead_code)]
    fn get_config_dir() -> Result<PathBuf> {
        #[cfg(windows)]
        {
            let app_data = std::env::var("APPDATA")
                .map_err(|e| crate::error::AcelinkError::ConfigError(
                    format!("No se pudo obtener APPDATA: {}", e)
                ))?;
            Ok(PathBuf::from(app_data).join("AcelinkHelper"))
        }

        #[cfg(target_os = "macos")]
        {
            let home = std::env::var("HOME")
                .map_err(|e| crate::error::AcelinkError::ConfigError(
                    format!("No se pudo obtener HOME: {}", e)
                ))?;
            Ok(PathBuf::from(home).join("Library/Preferences/AcelinkHelper"))
        }

        #[cfg(target_os = "linux")]
        {
            let home = std::env::var("HOME")
                .map_err(|e| crate::error::AcelinkError::ConfigError(
                    format!("No se pudo obtener HOME: {}", e)
                ))?;
            Ok(PathBuf::from(home).join(".config/acelinkhelper"))
        }
    }

    /// Carga la configuración desde el almacenamiento persistente
    pub fn load() -> Result<Self> {
        confy::load("acelinkhelper", None)
            .map_err(|e| crate::error::AcelinkError::ConfigyError(e))
    }

    /// Guarda la configuración actualizada
    pub fn save(&self) -> Result<()> {
        confy::store("acelinkhelper", None, self)
            .map_err(|e| crate::error::AcelinkError::ConfigyError(e))
    }

    /// Valida que la IP sea correcta
    pub fn validate(&self) -> Result<()> {
        if self.nas_ip.is_empty() {
            return Err(crate::error::AcelinkError::ConfigError(
                "IP del NAS no puede estar vacía".to_string(),
            ));
        }
        Ok(())
    }

    /// Actualiza la IP del NAS
    pub fn set_nas_ip(&mut self, ip: String) -> Result<()> {
        self.nas_ip = ip;
        self.validate()?;
        self.save()?;
        Ok(())
    }

    /// Actualiza el idioma
    pub fn set_language(&mut self, lang: Language) -> Result<()> {
        self.language = lang;
        self.save()?;
        Ok(())
    }
}
