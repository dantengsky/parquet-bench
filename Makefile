lint:
	cargo fmt --all
	cargo clippy --workspace --all-targets -- -D warnings
	# Check unused deps(make setup to install)
	cargo -Z sparse-registry machete

build:
	cargo build --bin opendal --release
