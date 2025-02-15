# Build natlint using `cargo build`
build:
	cargo build --bin natlint --release --locked

test testname:
	cargo test $(testname) --locked -- --nocapture
