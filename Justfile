default: dev

dev:
    pnpm tauri dev

clean:
    cd src-tauri && cargo clean