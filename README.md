# Pong in Rust 🏓

A cross-platform implementation of classic Pong written in Rust (2024 Edition). This project is designed as a modular **Cargo workspace** featuring a decoupled game engine with multiple frontends and a dedicated multiplayer server:
- **`pong_core`**: Platform-agnostic game simulation and network protocol schemas.
- **`pong_desktop`**: Native desktop application powered by **SDL2**.
- **`pong_web`**: Browser-compatible WebAssembly port powered by **Macroquad**.
- **`pong_server`**: Real-time async WebSocket multiplayer server powered by **Tokio**.
- **`web-wrapper`**: Responsive web UI featuring an offline local mode and online multiplayer lobby.

---

## 🏗️ Workspace Architecture

The repository is structured into focused crates to maximize code reuse across platforms:

```text
pong-rust/
├── .cargo/
│   └── config.toml          # vcpkg settings and wasm32 linker flags
├── pong_core/               # Shared engine & network schemas
│   ├── Cargo.toml
│   └── src/lib.rs
├── pong_desktop/            # Native desktop frontend (SDL2)
│   ├── Cargo.toml
│   └── src/main.rs
├── pong_server/             # Real-time WebSocket game server (Tokio + Tungstenite)
│   ├── Cargo.toml
│   └── src/main.rs
├── pong_web/                # WebAssembly frontend (Macroquad)
│   ├── Cargo.toml
│   └── src/main.rs
├── web-wrapper/             # Web assets, HTML UI lobby & WASM runner
│   ├── index.html
│   └── pong_web.wasm        # Compiled WebAssembly binary
├── build.rs                 # Build script (automates SDL2.dll copying on Windows)
├── Cargo.toml               # Workspace root configuration
└── Cargo.lock
```

### 1. `pong_core`
* **Dependencies**: `serde`, `serde_json`.
* **Role**: Platform-agnostic game simulation and network protocol.
* **Key Features**:
  * Frame-rate independent physics driven by delta time (`dt`).
  * Bounding-box (AABB) paddle and ball collision detection.
  * Score tracking and game state transitions.
  * Shared message definitions (`ClientMessage`, `ServerMessage`).

### 2. `pong_server`
* **Dependencies**: `tokio` (async runtime), `tokio-tungstenite` (WebSockets), `futures-util`, `rand`, `serde`, `serde_json`, `pong_core`.
* **Role**: Authoritative game server running on `ws://127.0.0.1:8080`.
* **Key Features**:
  * 4-letter alphanumeric room creation and matchmaking.
  * 60 FPS tick-based authoritative game simulation with pause/serve pacing.
  * Player input synchronization and live state broadcasts.
  * Client disconnect detection and room cleanup.

### 3. `pong_desktop`
* **Dependencies**: `sdl2` (`v0.38.0` with `use-vcpkg`), `pong_core`.
* **Role**: High-performance native desktop client.
* **Key Features**:
  * Hardware-accelerated 2D canvas rendering with VSync support.
  * Keyboard scancodes polling enabling simultaneous multi-key presses.
  * Real-time score and status display in the window title bar.

### 4. `pong_web`
* **Dependencies**: `macroquad` (`v0.4`), `pong_core`.
* **Role**: WebAssembly client targeting browsers via WebGL and HTML5 Canvas.
* **Key Features**:
  * Supports both local offline play and live online multiplayer modes.
  * Smooth delta-time rendering with center court markings and active player indicators.
  * Bridged to the web wrapper via JavaScript FFI and miniquad plugins.

---

## 📦 Dependencies & Prerequisites

### General Requirements
- **Rust**: Version compatible with edition 2024 (`rustc` / `cargo`).

### For Desktop (`pong_desktop`)
- **SDL2 Development Libraries**:
  - **Windows (Recommended)**: Installed via [vcpkg](https://github.com/microsoft/vcpkg):
    ```bash
    vcpkg install sdl2:x64-windows
    ```
  - **Linux (Ubuntu/Debian)**: `sudo apt-get install libsdl2-dev`
  - **macOS**: `brew install sdl2`

### For Web & Multiplayer (`pong_web` & `pong_server`)
- **WebAssembly Target**:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- **Local Static HTTP Server**: e.g., Python `http.server`, Node `serve`, etc.

---

## 🎮 Controls

### Offline Mode
| Action | Player 1 (Left Paddle) | Player 2 (Right Paddle) |
| :--- | :---: | :---: |
| **Move Up** | <kbd>W</kbd> | <kbd>↑</kbd> |
| **Move Down** | <kbd>S</kbd> | <kbd>↓</kbd> |
| **Start / Serve Round** | <kbd>Space</kbd> | <kbd>Space</kbd> |

### Online Multiplayer
| Action | Controls (Either Player) |
| :--- | :---: |
| **Move Up** | <kbd>W</kbd> or <kbd>↑</kbd> |
| **Move Down** | <kbd>S</kbd> or <kbd>↓</kbd> |
| **Pacing / Serving** | Handled automatically by server with brief round pauses |

---

## 🚀 Running the Project

### 1. Multiplayer Server
To run the WebSocket server:
```bash
cargo run -p pong_server
```
The server listens on `ws://127.0.0.1:8080`.

---

### 2. Web Client (Offline & Online Multiplayer)

1. **Compile the WASM Binary**:
   ```bash
   cargo build -p pong_web --target wasm32-unknown-unknown --release
   ```

2. **Copy to Web Wrapper**:
   *PowerShell (Windows):*
   ```powershell
   Copy-Item "target/wasm32-unknown-unknown/release/pong_web.wasm" -Destination "web-wrapper/pong_web.wasm" -Force
   ```
   *Bash (Linux/macOS):*
   ```bash
   cp target/wasm32-unknown-unknown/release/pong_web.wasm web-wrapper/pong_web.wasm
   ```

3. **Serve Locally**:
   ```bash
   cd web-wrapper
   python -m http.server 8000
   ```

4. **Play**:
   - Open [`http://localhost:8000`](http://localhost:8000) in two browser windows/tabs.
   - Click **Online Multiplayer**.
   - Window 1: Click **Create Room** and note the 4-letter room code.
   - Window 2: Enter the room code and click **Join Room**.
   - Both screens will transition directly into the match!

---

### 3. Native Desktop Application

To run the local desktop build using SDL2:
```bash
cargo run -p pong_desktop
```

---

## 🛠️ Workspace Verification

```bash
# Check all workspace crates
cargo check --workspace

# Run all unit tests
cargo test --workspace
```
