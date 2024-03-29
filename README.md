# Coin 

## Running 

```bash
# whole app
cargo tauri dev

# only tauri
cargo run --bin coin-tauri

# backend standalone
cargo run --bin coin

# web ui (svelte)
npm --prefix frontend/coin-web-ui run dev
```

## Building 

The following command bundles the statically generated frontend (svelte), the
tauri wrapper and the coin backend libraries:

```bash
cargo tauri build
```


