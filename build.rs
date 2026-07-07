#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    
    // Establecer icono del ejecutable
    res.set_icon("res/app.ico");
    
    // Compilar recursos
    res.compile()
        .expect("Error compilando recursos de Windows");
}

#[cfg(not(windows))]
fn main() {
    // En macOS y Linux, no hacer nada
}
