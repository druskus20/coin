# Coin 

## Running 

```bash
# whole app
cargo tauri dev

# only tauri
cargo run --bin coin-tauri

# backend standalone
cargo run --bin coin-cli

# web ui (svelte)
npm --prefix frontend/coin-web-ui run dev
```

## Building 

The following command bundles the statically generated frontend (svelte), the
tauri wrapper and the coin backend libraries:

```bash
cargo tauri build
```

## Structure

- `lib` contains the core libraries for coin. (TODO: support both local calls and remote (REST) backend)
- `frontend` contains the different frontends `coin-tauri` and `coin-cli`.
- [TODO] `backend` contains the code for the remote REST-based backend


