all:
	cargo run --release --bin scrape
	cargo run --release --bin verify
	cargo run --release --bin normalize
