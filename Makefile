.phony: run build

run:
	cargo run

build:
	cargo build

rbuild:
	cargo build --release

rrun:
	cargo run --release