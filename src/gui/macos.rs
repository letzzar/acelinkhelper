/// Implementación de GUI para macOS
/// Por ahora usa la misma interfaz egui que Windows
/// En el futuro podría usar Cocoa o SwiftUI

use acelinkhelper::config::Config;
use acelinkhelper::error::Result;
use std::sync::{Arc, Mutex};

use super::windows::ConfigApp as WindowsConfigApp;

// Reutilizar la implementación de Windows para macOS (ambos usan egui)
pub struct ConfigApp;

impl ConfigApp {
    pub fn run(config: Arc<Mutex<Config>>) -> Result<()> {
        // Por ahora macOS usa la misma interfaz que Windows
        // En el futuro se puede reemplazar con una UI nativa de macOS
        WindowsConfigApp::run(config)
    }
}
