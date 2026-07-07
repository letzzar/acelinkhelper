/// Módulo para garantizar que solo una instancia de la app puede ejecutarse
/// En Windows, usa un archivo de lock global

use crate::error::Result;
use std::path::PathBuf;
use std::fs;
use std::io::{Write, Read};

pub struct SingleInstanceLock {
    lock_file: PathBuf,
}

impl SingleInstanceLock {
    /// Intenta crear un lock. Si uno ya existe y la instancia anterior sigue viva, devuelve error
    pub fn new() -> Result<Self> {
        let lock_file = get_lock_file()?;
        
        // Si el archivo existe, verificar si el PID anterior sigue vivo
        if lock_file.exists() {
            if let Ok(mut file) = fs::File::open(&lock_file) {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    if let Ok(old_pid) = contents.trim().parse::<u32>() {
                        // Verificar si el proceso anterior sigue ejecutándose
                        if is_process_alive(old_pid) {
                            log::warn!("AcelinkHelper ya está en ejecución (PID: {})", old_pid);
                            return Err(crate::error::AcelinkError::SystemError(
                                "AcelinkHelper ya está en ejecución".to_string(),
                            ));
                        } else {
                            // El PID anterior no existe, limpiar el lock obsoleto
                            log::info!("Lock obsoleto encontrado (PID {} ya no existe), limpiando...", old_pid);
                            let _ = fs::remove_file(&lock_file);
                        }
                    }
                }
            }
        }

        // Crear el archivo de lock
        fs::create_dir_all(lock_file.parent().unwrap())
            .map_err(|e| crate::error::AcelinkError::SystemError(
                format!("Error creando directorio de lock: {}", e)
            ))?;

        let mut file = fs::File::create(&lock_file)
            .map_err(|e| crate::error::AcelinkError::SystemError(
                format!("Error creando archivo de lock: {}", e)
            ))?;

        let pid = std::process::id();
        writeln!(file, "{}", pid)
            .map_err(|e| crate::error::AcelinkError::SystemError(
                format!("Error escribiendo PID en lock: {}", e)
            ))?;

        log::info!("Lock creado en: {:?} (PID: {})", lock_file, pid);

        Ok(Self { lock_file })
    }
}

/// Verificar si un proceso con un PID específico está vivo
#[cfg(windows)]
fn is_process_alive(pid: u32) -> bool {
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;

    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if handle.is_null() {
            return false;
        }
        let result = CloseHandle(handle) != 0;
        result
    }
}

#[cfg(not(windows))]
fn is_process_alive(pid: u32) -> bool {
    // En Unix, enviar señal 0 (sin enviar nada) retorna 0 si el proceso existe
    unsafe {
        libc::kill(pid as libc::pid_t, 0) == 0
    }
}

fn get_lock_file() -> Result<PathBuf> {
    #[cfg(windows)]
    {
        let app_data = std::env::var("APPDATA")
            .map_err(|e| crate::error::AcelinkError::ConfigError(
                format!("No se pudo obtener APPDATA: {}", e)
            ))?;
        Ok(PathBuf::from(app_data).join("AcelinkHelper").join(".lock"))
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME")
            .map_err(|e| crate::error::AcelinkError::ConfigError(
                format!("No se pudo obtener HOME: {}", e)
            ))?;
        Ok(PathBuf::from(home).join("Library/Preferences/AcelinkHelper").join(".lock"))
    }

    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME")
            .map_err(|e| crate::error::AcelinkError::ConfigError(
                format!("No se pudo obtener HOME: {}", e)
            ))?;
        Ok(PathBuf::from(home).join(".config/acelinkhelper").join(".lock"))
    }
}

impl Drop for SingleInstanceLock {
    fn drop(&mut self) {
        // Limpiar el archivo de lock al salir
        if let Err(e) = fs::remove_file(&self.lock_file) {
            log::warn!("Error limpiando lock file: {}", e);
        } else {
            log::info!("Lock file limpiado correctamente");
        }
    }
}

