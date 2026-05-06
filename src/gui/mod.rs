/// Módulo GUI - Interfaz de usuario multi-plataforma
/// Usa #[cfg(...)] para compilar solo el código apropiado según el SO

use acelinkhelper::config::Config;
use acelinkhelper::error::Result;
use std::sync::{Arc, Mutex};

// Importar implementación según el SO
#[cfg(any(target_os = "windows", target_os = "macos"))]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
pub use windows::ConfigApp;

#[cfg(target_os = "macos")]
pub use macos::ConfigApp;

/// Trait común que toda implementación de GUI debe cumplir
/// Permite en el futuro agregar diferentes implementaciones de UI
#[allow(dead_code)]
pub trait GuiApp {
    fn run(config: Arc<Mutex<Config>>) -> Result<()>;
}
