# Build natlint using `cargo build`
build:
	cargo build --bin natlint --release --locked

# Install natlint using `cargo install`
install:
	cargo install --path . --locked

# Run tests using `cargo test`
test testname="--all":
	cargo test {{testname}} --locked -- --nocapture
