build:
	cargo watch -q -c -w src/ -x "build -q"

run:
	cargo watch -q -c -w src/ -x "run -q"

clean:
	cargo clean

cache:
	cargo-cache --remove-dir all

test:
	cargo test 
		
install:
	cargo build --release
	cargo install --path .

layout:
	zellij --layout rust-layout.kdl
