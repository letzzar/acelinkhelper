#![cfg_attr(windows, windows_subsystem = "windows")]

// Usar la librería acelinkhelper para toda la lógica de negocio
use acelinkhelper::{Config, Result, singleton, protocol, vlc, transform_acestream_url, detect_system_language};
use std::env;
use std::sync::{Arc, Mutex};

#[cfg(target_os = "macos")]
use std::ffi::CStr;

#[cfg(target_os = "macos")]
use objc::{class, declare::ClassDecl, msg_send, sel, sel_impl};
#[cfg(target_os = "macos")]
use objc::runtime::{Object, Sel};

// Importar GUI según la plataforma
#[path = "gui/mod.rs"]
mod gui;

#[cfg(target_os = "macos")]
fn register_macos_url_handler() -> Result<()> {
    unsafe {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("AcelinkHelperUrlHandler", superclass)
            .ok_or_else(|| acelinkhelper::error::AcelinkError::SystemError(
                "No se pudo crear la clase Objective-C para AppleEvents".to_string()
            ))?;

        extern "C" fn handle_get_url(_this: &Object, _cmd: Sel, event: *mut Object, _reply: *mut Object) {
            unsafe {
                let descriptor: *mut Object = msg_send![event, paramDescriptorForKeyword: 0x2D2D2D2D_u32];
                if descriptor.is_null() {
                    log::warn!("AppleEvent recibido sin descriptor directo");
                    return;
                }

                let url_nsstring: *mut Object = msg_send![descriptor, stringValue];
                if url_nsstring.is_null() {
                    log::warn!("AppleEvent URL sin cadena");
                    return;
                }

                let cstr: *const std::os::raw::c_char = msg_send![url_nsstring, UTF8String];
                if cstr.is_null() {
                    log::warn!("AppleEvent URL no pudo obtener UTF8String");
                    return;
                }

                let url = CStr::from_ptr(cstr).to_string_lossy().into_owned();
                log::info!("AppleEvent URL recibido: {}", url);

                if let Err(e) = handle_protocol_url(&url) {
                    log::error!("Error manejando AppleEvent URL: {}", e);
                }
            }
        }

        decl.add_method(
            sel!(handleGetURLEvent:withReplyEvent:),
            handle_get_url as extern "C" fn(&Object, Sel, *mut Object, *mut Object),
        );

        let cls = decl.register();
        let handler: *mut Object = msg_send![cls, new];
        let apple_event_manager: *mut Object = msg_send![class!(NSAppleEventManager), sharedAppleEventManager];
        let _ : () = msg_send![apple_event_manager,
            setEventHandler: handler
            andSelector: sel!(handleGetURLEvent:withReplyEvent:)
            forEventClass: 0x4755524C_u32
            andEventID: 0x4755524C_u32
        ];

        log::info!("AppleEvent handler de URL registrado en macOS");
    }

    Ok(())
}

fn main() {
    env_logger::init();

    // Registrar el handler de AppleEvents cuando se ejecute como bundle en macOS
    #[cfg(target_os = "macos")]
    if let Err(e) = register_macos_url_handler() {
        log::error!("Advertencia al registrar handler macOS: {}", e);
    }

    // Obtener el path de la aplicación actual
    let app_path = env::current_exe()
        .expect("No se pudo obtener la ruta de la aplicación");

    // Intentar registrar el protocolo en el sistema (multi-plataforma)
    if let Err(e) = protocol::register_protocol(app_path.clone()) {
        log::error!("Advertencia al registrar protocolo: {}", e);
    }

    // Procesar argumentos (URL acestream://)
    let args: Vec<String> = env::args().collect();
    if let Some(url) = args.iter().skip(1).find(|arg| arg.starts_with("acestream://")) {
        // Si viene con URL de protocolo, procesar SIN GUI y sin singleton
        log::info!("URL acestream recibida: {}", url);
        
        if let Err(e) = handle_protocol_url(url) {
            log::error!("Error procesar URL: {}", e);
        }
        // Salir después de procesar la URL
        return;
    }

    // Aquí solo llega si NO es un protocolo, sino ejecución normal de GUI
    // En este caso, aplicar singleton para evitar múltiples GUIs
    let _lock = match singleton::SingleInstanceLock::new() {
        Ok(lock) => {
            log::info!("Lock de instancia única obtenido");
            lock
        }
        Err(e) => {
            log::error!("Error: {}", e);
            return;
        }
    };

    // Cargar o crear configuración
    let config = match Config::load() {
        Ok(cfg) => {
            log::info!("Configuración cargada: IP={}", cfg.nas_ip);
            Arc::new(Mutex::new(cfg))
        }
        Err(e) => {
            log::warn!("Configuración no encontrada, creando por defecto: {}", e);
            // Detectar idioma del sistema
            let system_language = detect_system_language();
            log::info!("Idioma del sistema detectado: {:?}", system_language);
            
            // Crear configuración con el idioma del sistema
            let mut default_config = Config::default();
            default_config.language = system_language;
            Arc::new(Mutex::new(default_config))
        }
    };

    // Abrir GUI (automáticamente selecciona Windows o macOS)
    log::info!("Abriendo interfaz gráfica");
    if let Err(e) = gui::ConfigApp::run(config) {
        log::error!("Error en GUI: {}", e);
    }
}

/// Procesa una URL acestream:// desde el protocolo del SO
/// Esta función usa la lógica de la librería, no depende de la UI
fn handle_protocol_url(url: &str) -> Result<()> {
    // Cargar configuración
    let cfg = Config::load()?;
    
    // Transformar URL usando función de librería
    let final_url = transform_acestream_url(url, &cfg)?;
    
    // Lanzar VLC
    vlc::launch_vlc(&final_url)?;
    
    Ok(())
}
