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

## Download

Download the latest version from the [releases page](https://github.com/letzzar/acelinkhelper/releases/latest):

| Platform | File |
|---|---|
| Windows (x86_64) | [acelinkhelper-windows-x86_64.zip](https://github.com/letzzar/acelinkhelper/releases/download/v1.0.0/acelinkhelper-windows-x86_64.zip) |
| macOS (Apple Silicon) | [acelinkhelper-macos-arm64.tar.gz](https://github.com/letzzar/acelinkhelper/releases/download/v1.0.0/acelinkhelper-macos-arm64.tar.gz) |
| macOS (Intel) | [acelinkhelper-macos-x86_64.tar.gz](https://github.com/letzzar/acelinkhelper/releases/download/v1.0.0/acelinkhelper-macos-x86_64.tar.gz) |
| Linux (x86_64) | [acelinkhelper-linux-x86_64.tar.gz](https://github.com/letzzar/acelinkhelper/releases/download/v1.0.0/acelinkhelper-linux-x86_64.tar.gz) |

## Server Setup (Docker)

AcelinkHelper is designed to work with the AceStream engine running in **Docker**. This is the recommended approach — no native AceStream installation needed, works on any machine with Docker, and optionally routes all AceStream traffic through a VPN for privacy.

**Requires:** [Docker + Docker Compose](https://docs.docker.com/get-docker/)

---

### Option 1 — Direct (no VPN)

Save as `docker-compose.yml` and run `docker compose up -d`:

```yaml
version: "3"
services:
  acelink:
    image: blaiseio/acelink
    container_name: acelink
    platform: linux/amd64
    ports:
      - 6878:6878   # AceStream engine port — AcelinkHelper connects here
    restart: always
```

Configure AcelinkHelper to connect to `http://<docker-host-ip>:6878`.

---

### Option 2 — Behind a WireGuard VPN (recommended)

All AceStream traffic is tunnelled through a VPN using [Gluetun](https://github.com/qdm12/gluetun). AceStream is invisible to your ISP.

```yaml
version: "3"
services:
  gluetun:
    image: qmcgaw/gluetun:latest
    container_name: gluetun
    cap_add:
      - NET_ADMIN
    devices:
      - /dev/net/tun:/dev/net/tun
    ports:
      - 6878:6878   # Port exposed on host — AcelinkHelper connects here
    environment:
      - VPN_SERVICE_PROVIDER=custom
      - VPN_TYPE=wireguard
      - WIREGUARD_PRIVATE_KEY=<your_private_key>      # From [Interface] PrivateKey
      - WIREGUARD_ADDRESSES=172.16.0.2/32             # From [Interface] Address
      - WIREGUARD_ENDPOINT_IP=162.159.192.1           # From [Peer] Endpoint (IP)
      - WIREGUARD_ENDPOINT_PORT=2408                  # From [Peer] Endpoint (port)
      - WIREGUARD_PUBLIC_KEY=<peer_public_key>        # From [Peer] PublicKey
      - TZ=Europe/Madrid
    restart: always

  acelink:
    image: blaiseio/acelink
    container_name: acelink
    platform: linux/amd64
    network_mode: "service:gluetun"   # AceLink hides behind Gluetun
    depends_on:
      - gluetun
    restart: always
```

#### How to get your WireGuard keys

**From a VPN provider** (Mullvad, ProtonVPN, IVPN, etc.):

1. Log in to your provider's dashboard and download a **WireGuard config file** (`.conf`)
2. Open it — it looks like this:

```ini
[Interface]
PrivateKey = ABC123...          ← WIREGUARD_PRIVATE_KEY
Address    = 172.16.0.2/32      ← WIREGUARD_ADDRESSES

[Peer]
PublicKey  = XYZ789...          ← WIREGUARD_PUBLIC_KEY
Endpoint   = 162.159.192.1:2408 ← IP → WIREGUARD_ENDPOINT_IP  /  port → WIREGUARD_ENDPOINT_PORT
```

**Generate your own keys** (self-hosted WireGuard server):

```bash
# Install wireguard-tools
sudo apt install wireguard-tools   # Ubuntu / Debian
brew install wireguard-tools       # macOS

# Generate private key
wg genkey > wg_private.key
cat wg_private.key            # → paste as WIREGUARD_PRIVATE_KEY

# Derive public key
cat wg_private.key | wg pubkey > wg_public.key
cat wg_public.key             # → register this on your WireGuard server
```

> **Security:** Never share your private key. Never generate WireGuard keys using online tools.

---

## Prerequisites

| Requirement | macOS | Windows |
|---|---|---|
| Rust toolchain | [rustup.rs](https://rustup.rs) | [rustup.rs](https://rustup.rs) |
| C linker | Xcode Command Line Tools (`xcode-select --install`) | [MSVC Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) — select "Desktop development with C++" |
| VLC media player | [videolan.org/vlc](https://www.videolan.org/vlc/) | [videolan.org/vlc](https://www.videolan.org/vlc/) |

No API keys required.

## Build

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # macOS/Linux
# On Windows: download and run rustup-init.exe from rustup.rs

git clone https://github.com/letzzar/acelinkhelper.git
cd acelinkhelper
cargo build --release
```

Binary: `target/release/acelinkhelper` (macOS/Linux) or `target\release\acelinkhelper.exe` (Windows).

## Usage

1. Start the AceStream Docker container (see Server Setup above)
2. Launch **AcelinkHelper** — it sits in the system tray
3. Click the tray icon → Settings → set the engine address to `http://<docker-host-ip>:6878`
4. Click any `acestream://` link in your browser — VLC opens automatically

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

## Descarga

Descarga la última versión desde la [página de releases](https://github.com/letzzar/acelinkhelper/releases/latest):

| Plataforma | Archivo |
|---|---|
| Windows (x86_64) | [acelinkhelper-windows-x86_64.zip](https://github.com/letzzar/acelinkhelper/releases/download/v1.0.0/acelinkhelper-windows-x86_64.zip) |
| macOS (Apple Silicon) | [acelinkhelper-macos-arm64.tar.gz](https://github.com/letzzar/acelinkhelper/releases/download/v1.0.0/acelinkhelper-macos-arm64.tar.gz) |
| macOS (Intel) | [acelinkhelper-macos-x86_64.tar.gz](https://github.com/letzzar/acelinkhelper/releases/download/v1.0.0/acelinkhelper-macos-x86_64.tar.gz) |
| Linux (x86_64) | [acelinkhelper-linux-x86_64.tar.gz](https://github.com/letzzar/acelinkhelper/releases/download/v1.0.0/acelinkhelper-linux-x86_64.tar.gz) |

## Configuración del Servidor (Docker)

AcelinkHelper está diseñado para funcionar con el motor AceStream ejecutándose en **Docker**. Es el enfoque recomendado — sin instalación nativa de AceStream, funciona en cualquier máquina con Docker y opcionalmente enruta todo el tráfico de AceStream a través de una VPN.

**Requisito:** [Docker + Docker Compose](https://docs.docker.com/get-docker/)

---

### Opción 1 — Directo (sin VPN)

Guarda como `docker-compose.yml` y ejecuta `docker compose up -d`:

```yaml
version: "3"
services:
  acelink:
    image: blaiseio/acelink
    container_name: acelink
    platform: linux/amd64
    ports:
      - 6878:6878   # Puerto del motor AceStream — al que se conecta AcelinkHelper
    restart: always
```

Configura AcelinkHelper para conectarse a `http://<ip-del-host-docker>:6878`.

---

### Opción 2 — Detrás de una VPN WireGuard (recomendado)

Todo el tráfico de AceStream se tuneliza a través de una VPN con [Gluetun](https://github.com/qdm12/gluetun). AceStream es invisible para tu ISP.

```yaml
version: "3"
services:
  gluetun:
    image: qmcgaw/gluetun:latest
    container_name: gluetun
    cap_add:
      - NET_ADMIN
    devices:
      - /dev/net/tun:/dev/net/tun
    ports:
      - 6878:6878   # Puerto expuesto en el host — AcelinkHelper se conecta aquí
    environment:
      - VPN_SERVICE_PROVIDER=custom
      - VPN_TYPE=wireguard
      - WIREGUARD_PRIVATE_KEY=<tu_clave_privada>      # De [Interface] PrivateKey
      - WIREGUARD_ADDRESSES=172.16.0.2/32             # De [Interface] Address
      - WIREGUARD_ENDPOINT_IP=162.159.192.1           # De [Peer] Endpoint (IP)
      - WIREGUARD_ENDPOINT_PORT=2408                  # De [Peer] Endpoint (puerto)
      - WIREGUARD_PUBLIC_KEY=<clave_publica_peer>     # De [Peer] PublicKey
      - TZ=Europe/Madrid
    restart: always

  acelink:
    image: blaiseio/acelink
    container_name: acelink
    platform: linux/amd64
    network_mode: "service:gluetun"   # AceLink se oculta tras Gluetun
    depends_on:
      - gluetun
    restart: always
```

#### Cómo obtener tus claves WireGuard

**Desde un proveedor de VPN** (Mullvad, ProtonVPN, IVPN, etc.):

1. Inicia sesión en el panel de tu proveedor y descarga un **archivo de configuración WireGuard** (`.conf`)
2. Ábrelo — tiene este aspecto:

```ini
[Interface]
PrivateKey = ABC123...           ← WIREGUARD_PRIVATE_KEY
Address    = 172.16.0.2/32       ← WIREGUARD_ADDRESSES

[Peer]
PublicKey  = XYZ789...           ← WIREGUARD_PUBLIC_KEY
Endpoint   = 162.159.192.1:2408  ← IP → WIREGUARD_ENDPOINT_IP  /  puerto → WIREGUARD_ENDPOINT_PORT
```

**Generar tus propias claves** (servidor WireGuard propio):

```bash
# Instalar wireguard-tools
sudo apt install wireguard-tools   # Ubuntu / Debian
brew install wireguard-tools       # macOS

# Generar clave privada
wg genkey > wg_private.key
cat wg_private.key            # → pega esto como WIREGUARD_PRIVATE_KEY

# Derivar clave pública
cat wg_private.key | wg pubkey > wg_public.key
cat wg_public.key             # → registra esto en tu servidor WireGuard
```

> **Seguridad:** Nunca compartas tu clave privada. Nunca generes claves WireGuard con herramientas online.

---

## Requisitos previos

| Requisito | macOS | Windows |
|---|---|---|
| Rust | [rustup.rs](https://rustup.rs) | [rustup.rs](https://rustup.rs) |
| Linker C | Xcode Command Line Tools (`xcode-select --install`) | [MSVC Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) — selecciona "Desarrollo de escritorio con C++" |
| VLC | [videolan.org/vlc](https://www.videolan.org/vlc/) | [videolan.org/vlc](https://www.videolan.org/vlc/) |

No se necesitan claves API.

## Compilar

```bash
# Instalar Rust (si no está instalado)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # macOS/Linux
# En Windows: descarga y ejecuta rustup-init.exe desde rustup.rs

git clone https://github.com/letzzar/acelinkhelper.git
cd acelinkhelper
cargo build --release
```

Binario en: `target/release/acelinkhelper` (macOS/Linux) o `target\release\acelinkhelper.exe` (Windows).

## Uso

1. Arranca el contenedor Docker de AceStream (ver Configuración del Servidor arriba)
2. Lanza **AcelinkHelper** — se minimiza en la bandeja del sistema
3. Clic en el icono → Ajustes → configura la dirección del motor a `http://<ip-del-host-docker>:6878`
4. Haz clic en cualquier enlace `acestream://` en el navegador — VLC se abre automáticamente

## Licencia

GNU General Public License v3.0 — ver [LICENSE](LICENSE)
