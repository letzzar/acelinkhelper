# AcelinkHelper

**English** | [Español](#español)

---

A lightweight desktop tray app for macOS and Windows that intercepts `acestream://` protocol URLs and opens the resulting stream directly in VLC.

## Features

- Registers as the system handler for `acestream://` links
- Connects to a local AceStream engine and retrieves the HTTP stream URL
- Launches VLC automatically with the converted URL
- System tray icon with quick settings access
- Persistent configuration (server address, VLC path)
- Auto-detects system language (Spanish / English)

## Requirements

- [AceStream Engine](https://www.acestream.org) running locally or on a remote host
- [VLC media player](https://www.videolan.org/vlc/)
- macOS 11+ or Windows 10+

## Build

```bash
cargo build --release
```

The binary is placed in `target/release/acelinkhelper`.

On macOS, the `.app` bundle and `.dmg` installer are built separately via the packaging script in the repo.

## Usage

1. Launch **AcelinkHelper** — it sits in the system tray
2. Click the tray icon to open settings and configure the AceStream engine address
3. Click any `acestream://` link in your browser — VLC opens automatically

## Configuration

Settings are stored automatically in the OS config directory:
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

## Requisitos

- [Motor AceStream](https://www.acestream.org) ejecutándose local o en remoto
- [VLC media player](https://www.videolan.org/vlc/)
- macOS 11+ o Windows 10+

## Compilar

```bash
cargo build --release
```

El binario queda en `target/release/acelinkhelper`.

## Uso

1. Lanza **AcelinkHelper** — se minimiza en la bandeja del sistema
2. Haz clic en el icono para abrir ajustes y configurar la dirección del motor AceStream
3. Haz clic en cualquier enlace `acestream://` en el navegador — VLC se abre automáticamente

## Licencia

MIT © 2026 letzzar
