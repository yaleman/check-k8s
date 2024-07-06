GIT_COMMIT := $(shell git rev-parse HEAD)
IMAGE_BASE ?= check-k8s
IMAGE_EXT_VERSION ?= $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "check_k8s")  | .version')
IMAGE_ARCH ?= "linux/amd64,linux/arm64"
MARKDOWN_FORMAT_ARGS ?= --options-line-width=100

.DEFAULT: help
.PHONY: help
help:
	@grep -E -h '\s##\s' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: test
test:
	cargo test

.PHONY: precommit
precommit: ## all the usual test things
precommit: test codespell doc
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: codespell
codespell: ## spell-check things.
codespell:
	codespell -c \
		-D .codespell_dictionary \
		--ignore-words .codespell_ignore \
		--skip='./target' \
		--skip='./docs/*,./.git'

.PHONY: doc
doc: ## Build the Rust documentation locally
doc:
	cargo doc --document-private-items --no-deps

.PHONY: doc/format
doc/format: ## Format docs if they exist
	find . -type f  \
		-not -path './target/*' \
		-not -path './docs/*' \
		-not -path '*/.venv/*' -not -path './vendor/*'\
		-not -path '*/.*/*' \
		-name \*.md \
		-exec deno fmt --check $(MARKDOWN_FORMAT_ARGS) "{}" +

.PHONY: doc/format/fix
doc/format/fix: ## Fix docs if they exist
	find . -type f  -not -path './target/*' -not -path '*/.venv/*' -not -path './vendor/*'\
		-name \*.md \
		-exec deno fmt  $(MARKDOWN_FORMAT_ARGS) "{}" +

.PHONY: release/prep
prep:
	cargo outdated -R
	cargo audit


.PHONY: coverage
coverage: ## Run all the coverage tests
coverage:
	# rm -rf ./target/debug/deps/
	LLVM_PROFILE_FILE="$(PWD)/target/profile/coverage-%p-%m.profraw" RUSTFLAGS="-C instrument-coverage" cargo test $(TESTS)
	rm -rf ./target/coverage/html
	grcov . --binary-path ./target/debug/deps/ \
		-s . \
		--llvm \
		-t html \
		--branch \
		--ignore-not-existing \
		--ignore 'src/bin/*' \
		--ignore '../*' \
		--ignore "/*" \
		--ignore "target/*" \
		-o target/coverage/html
	echo "Coverage report is in ./target/coverage/html/index.html"
