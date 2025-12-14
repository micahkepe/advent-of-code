.PHONY: test
test:
	for dir in crates/*; do \
		if [[ -f "$$dir/Cargo.toml" ]]; then \
			pushd "$$dir"; \
			cargo test --verbose; \
			popd; \
		fi; \
	done
	# Integration tests
	cargo test --verbose
