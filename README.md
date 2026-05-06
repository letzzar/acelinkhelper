# AcelinkHelper

**English** | [Español](#español)

---

A lightweight desktop tray app for macOS and Windows that intercepts `acestream://` protocol URLs and opens the resulting stream directly in VLC.

## Features

- Registers as the system handler for `acestream://` links
- Connects to a local or remote AceStream engine and retrieves the HTTP stream URL
- Launches VLC automatically with the converted URL
- System tray icon with quick settings access
- Persistent configuration (server address, VLC path)
- Auto-detects system language (Spanish / English)

## Prerequisites

| Requirement | macOS | Windows |
|---|---|---|
| Rust toolchain | [rustup.rs](https://rustup.rs) | [rustup.rs](https://rustup.rs) |
| C linker | Xcode Command Line Tools (`xcode-select --install`) | [MSVC Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) — select "Desktop development with C++" |
| AceStream engine | [acestream.org](https://www.acestream.org) | [acestream.org](https://www.acestream.org) |
| VLC media player | [videolan.org/vlc](https://www.videolan.org/vlc/) | [videolan.org/vlc](https://www.videolan.org/vlc/) |

No API keys required.

## Build

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # macOS/Linux
# On Windows: download and run rustup-init.exe from rustup.rs

# Clone and build
git clone https://github.com/letzzar/acelinkhelper.git
cd acelinkhelper
cargo build --release
```

The binary is placed in `target/release/acelinkhelper` (macOS/Linux) or `target\release\acelinkhelper.exe` (Windows).

## Usage

1. Launch **AcelinkHelper** — it sits in the system tray
2. Click the tray icon → Settings → set your AceStream engine address (default: `http://127.0.0.1:6878`)
3. Click any `acestream://` link in your browser — VLC opens automatically

## Configuration

Settings are stored automatically:
- **macOS**: `~/Library/Application Support/acelinkhelper/`
- **Windows**: `%APPDATA%\acelinkhelper\`

---

## Español

Aplicación ligera de bandeja del sistema para macOS y Windows que intercepta URLs con el protocolo `acestream://` y abre el stream resultante directamente en VLC.

## Características

- Se registra como manejador del sistema para enlaces `acestream://`
- Se conecta al motor AceStream local o remoto y obtiene la URL HTTP del stream
- Lanza VLC automáticamente con la URL convertida
- Icono en la bandeja del sistema con acceso rápido a ajustes
- Configuración persistente (dirección del servidor, ruta de VLC)
- Detección automática del idioma del sistema (español / inglés)

## Requisitos previos

| Requisito | macOS | Windows |
|---|---|---|
| Rust | [rustup.rs](https://rustup.rs) | [rustup.rs](https://rustup.rs) |
| Linker C | Xcode Command Line Tools (`xcode-select --install`) | [MSVC Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) — selecciona "Desarrollo de escritorio con C++" |
| Motor AceStream | [acestream.org](https://www.acestream.org) | [acestream.org](https://www.acestream.org) |
| VLC | [videolan.org/vlc](https://www.videolan.org/vlc/) | [videolan.org/vlc](https://www.videolan.org/vlc/) |

No se necesitan claves API.

## Compilar

```bash
# Instalar Rust (si no está instalado)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # macOS/Linux
# En Windows: descarga y ejecuta rustup-init.exe desde rustup.rs

# Clonar y compilar
git clone https://github.com/letzzar/acelinkhelper.git
cd acelinkhelper
cargo build --release
```

El binario queda en `target/release/acelinkhelper` (macOS/Linux) o `target\release\acelinkhelper.exe` (Windows).

## Uso

1. Lanza **AcelinkHelper** — se minimiza en la bandeja del sistema
2. Clic en el icono → Ajustes → configura la dirección del motor AceStream (por defecto: `http://127.0.0.1:6878`)
3. Haz clic en cualquier enlace `acestream://` en el navegador — VLC se abre automáticamente

## Licencia

MIT © 2026 letzzar
