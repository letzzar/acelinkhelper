/// Implementación de GUI para Windows usando egui/eframe
/// #[cfg(target_os = "windows")] hace que esto solo compile en Windows

use acelinkhelper::config::{Config, Language};
use acelinkhelper::error::Result;
use acelinkhelper::icons;
use eframe::egui;
use std::sync::{Arc, Mutex};

pub struct ConfigApp {
    config: Arc<Mutex<Config>>,
    ip_input: String,
    selected_language: Language,
    show_message: Option<String>,
    i18n: I18n,
}

pub struct I18n {
    language: Language,
}

impl I18n {
    fn new(language: Language) -> Self {
        Self { language }
    }

    fn title(&self) -> &str {
        match self.language {
            Language::Spanish => "Configuración de AcelinkHelper",
            Language::English => "AcelinkHelper Settings",
        }
    }

    fn server_url(&self) -> &str {
        match self.language {
            Language::Spanish => "Dirección del Servidor NAS:",
            Language::English => "NAS Server Address:",
        }
    }

    fn language_label(&self) -> &str {
        match self.language {
            Language::Spanish => "Idioma:",
            Language::English => "Language:",
        }
    }

    fn save(&self) -> &str {
        match self.language {
            Language::Spanish => "Guardar",
            Language::English => "Save",
        }
    }

    fn close(&self) -> &str {
        match self.language {
            Language::Spanish => "Cerrar",
            Language::English => "Close",
        }
    }

    fn invalid_ip(&self) -> &str {
        match self.language {
            Language::Spanish => "Por favor ingresa una dirección IP válida",
            Language::English => "Please enter a valid IP address",
        }
    }

    fn saved(&self) -> &str {
        match self.language {
            Language::Spanish => "Configuración guardada",
            Language::English => "Settings saved",
        }
    }
}

impl ConfigApp {
    pub fn new(config: Arc<Mutex<Config>>) -> Self {
        let cfg = config.lock().unwrap();
        let selected_language = cfg.language;
        let ip_input = cfg.nas_ip.clone();
        drop(cfg);

        Self {
            config,
            ip_input,
            selected_language,
            show_message: None,
            i18n: I18n::new(selected_language),
        }
    }

    pub fn run(config: Arc<Mutex<Config>>) -> Result<()> {
        // Cargar icono para la ventana
        let icon_data = load_icon_data(icons::get_app_icon_bytes())?;
        
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([500.0, 300.0])
                .with_icon(std::sync::Arc::new(icon_data)),
            ..Default::default()
        };
        
        let _ = eframe::run_native(
            "AcelinkHelper",
            options,
            Box::new(move |_cc| {
                Box::new(ConfigApp::new(config))
            }),
        );
        Ok(())
    }
}

impl eframe::App for ConfigApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.i18n.title());
            ui.separator();

            ui.label(self.i18n.server_url());
            ui.text_edit_singleline(&mut self.ip_input);

            ui.separator();

            ui.label(self.i18n.language_label());
            ui.horizontal(|ui| {
                if ui.radio(self.selected_language == Language::Spanish, "Español").clicked() {
                    self.selected_language = Language::Spanish;
                    self.i18n = I18n::new(Language::Spanish);
                }
                if ui.radio(self.selected_language == Language::English, "English").clicked() {
                    self.selected_language = Language::English;
                    self.i18n = I18n::new(Language::English);
                }
            });

            ui.separator();

            if let Some(ref msg) = self.show_message {
                ui.colored_label(egui::Color32::GREEN, msg);
            }

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button(self.i18n.save()).clicked() {
                    if self.ip_input.is_empty() {
                        self.show_message = Some(self.i18n.invalid_ip().to_string());
                    } else {
                        let mut cfg = self.config.lock().unwrap();
                        cfg.nas_ip = self.ip_input.clone();
                        cfg.language = self.selected_language;

                        if cfg.save().is_ok() {
                            self.show_message = Some(self.i18n.saved().to_string());
                            log::info!(
                                "Configuración guardada: IP={}, Lang={:?}",
                                self.ip_input,
                                self.selected_language
                            );
                        }
                    }
                }

                if ui.button(self.i18n.close()).clicked() {
                    log::info!("Cerrando ventana de configuración");
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
}

/// Carga PNG bytes y los convierte a IconData para egui
fn load_icon_data(png_bytes: &[u8]) -> Result<egui::IconData> {
    use image::io::Reader;
    use std::io::Cursor;

    let reader = Reader::new(Cursor::new(png_bytes))
        .with_guessed_format()
        .map_err(|e| acelinkhelper::error::AcelinkError::SystemError(
            format!("Error detectando formato PNG: {}", e)
        ))?;

    let image = reader.decode()
        .map_err(|e| acelinkhelper::error::AcelinkError::SystemError(
            format!("Error decodificando PNG: {}", e)
        ))?;

    let rgba_image = image.to_rgba8();
    let (width, height) = rgba_image.dimensions();
    
    Ok(egui::IconData {
        rgba: rgba_image.into_raw(),
        width: width as u32,
        height: height as u32,
    })
}
