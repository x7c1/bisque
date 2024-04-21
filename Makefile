help: ## docs : Display tasks
	@cat Makefile |\
	egrep '^[A-Z0-9a-z-]+:' |\
	sed -e 's/:[ ]*##[ ]*/:/' |\
	column -t -s :

cargo-clippy: ## lint
	cargo clippy -- \
	    --no-deps \
	    --deny warnings

cargo-clippy-fix: ## lint
	cargo clippy --fix -- \
	    --no-deps \
	    --deny warnings

cargo-fmt: ## format
	cargo fmt

cargo-fmt-check: ## format
	cargo fmt -- --check

format: ## format
	make cargo-fmt

test: ## test
	cargo test
