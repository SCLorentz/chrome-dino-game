cargo build-web
wasm-bindgen \
	target/wasm32-unknown-unknown/release/chrome_dino_game.wasm \
	--out-dir static/pkg \
	--target web
