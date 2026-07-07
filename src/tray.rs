use crate::error::Result;
use crate::icons;
use std::sync::mpsc::{channel, Receiver, Sender};
use tray_icon::menu::{Menu, MenuItem};
use tray_icon::TrayIconBuilder;

#[derive(Debug, Clone)]
pub enum TrayEvent {
    Exit,
    ConfigRequested,
}

pub struct TrayManager {
    tx: Sender<TrayEvent>,
    rx: Receiver<TrayEvent>,
}

impl TrayManager {
    /// Crea una nueva instancia del gestor de bandeja
    pub fn new() -> Result<Self> {
        let (tx, rx) = channel();
        Ok(Self { tx, rx })
    }

    /// Inicia la bandeja del sistema
    pub fn start(&self) -> Result<()> {
        // Cargar icono
        let tray_icon = icons::load_tray_icon()?;

        // Crear menú contextual simple
        let menu = Menu::new();
        
        let config_item = MenuItem::new("⚙️ Configuración", true, None);
        menu.append_items(&[&config_item]);

        let exit_item = MenuItem::new("❌ Cerrar", true, None);
        menu.append_items(&[&exit_item]);

        // Construir el icono de bandeja
        let _tray_icon_ref = TrayIconBuilder::new()
            .with_tooltip("AcelinkHelper")
            .with_icon(tray_icon)
            .with_menu(Box::new(menu))
            .build()
            .map_err(|e| crate::error::AcelinkError::SystemError(
                format!("No se pudo crear icono de bandeja: {}", e)
            ))?;

        log::info!("Bandeja del sistema iniciada");
        Ok(())
    }

    /// Obtiene el siguiente evento de la bandeja
    pub fn get_event(&self) -> Option<TrayEvent> {
        self.rx.try_recv().ok()
    }

    /// Espera a un evento de la bandeja (bloqueante)
    pub fn wait_event(&self) -> Result<TrayEvent> {
        self.rx.recv()
            .map_err(|e| crate::error::AcelinkError::SystemError(
                format!("Error recibiendo evento de bandeja: {}", e)
            ))
    }

    /// Envía un evento a la bandeja (usado internamente)
    pub fn send_event(&self, event: TrayEvent) -> Result<()> {
        self.tx.send(event)
            .map_err(|e| crate::error::AcelinkError::SystemError(
                format!("Error enviando evento a bandeja: {}", e)
            ))
    }
}

impl Default for TrayManager {
    fn default() -> Self {
        Self::new().expect("No se pudo crear TrayManager")
    }
}
