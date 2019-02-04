all:
	# scraping API JSON
	cargo run --release --bin scrape
	# verifying scraped data matched expected API schema
	cargo run --release --bin verify
	# creating normalized simplified version of the data
	cargo run --release --bin normalize
	# serving the normalized data through a web interface
	cargo run --release --bin serve
