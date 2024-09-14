# Blorf

## Running
Serve static resources
```bash
light-server -c .lightrc
```

Build + watch
```bash
cargo watch -i .gitignore -i "dist/*" -s "wasm-pack build --target web --out-dir="dist" --no-pack --no-typescript --release"
```