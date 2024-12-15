test:
	cargo test && cargo test --all-features && cargo test --no-default-features
doc:
	cargo doc --all-features --open
fmt:
	cargo fmt