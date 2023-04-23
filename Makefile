PACKAGE = sparklet

# vet
vet\:check: # Check error [synonym: check]
	@cargo check --workspace --verbose
.PHONY: vet\:check

check: vet\:check
.PHONY: check

vet\:format: # Show format diff [synonym: vet:fmt, format, fmt]
	@cargo fmt --all -- --check
.PHONY: vet\:format

vet\:fmt: vet\:format
.PHONY: vet\:fmt

format: vet\:format
.PHONY: format

fmt: vet\:format
.PHONY: fmt

vet\:lint: # Show suggestions relates to hygiene [synonym: lint]
	@cargo clippy --all-targets
.PHONY: vet\:lint

lint: vet\:lint
.PHONY: lint

vet\:all: check fmt lint # Run all vet targets [synonym: vet]
.PHONY: vet\:all

vet: vet\:all
.PHONY: vet

# test
test\:unit\:lib: # Run only unit tests for lib package
	@cargo test --lib -- --nocapture
.PHONY: test\:unit\:lib

test\:unit\:cli: # Run only unit tests for -cli package
	@cargo test --bin $(PACKAGE)-cli -- --nocapture
.PHONY: test\:unit\:lib

test\:unit\:tui: # Run only unit tests for -tui package
	@cargo test --bin $(PACKAGE)-tui -- --nocapture
.PHONY: test\:unit\:tui

test\:unit: # Run only unit tests for all packages
	@cargo test --lib --bins -- --nocapture
.PHONY: test\:unit

test\:integration: # Run only integration tests for all packages
	@cargo test --test integration -- --nocapture
.PHONY: test\:integration

test\:doc: # Run only doc tests
	@cargo test --doc
.PHONY: test\:doc

test\:all: test\:doc # Run all tests
	@cargo test --all -- --nocapture
.PHONY: test\:all

# build
build\:debug\:lib: # Build only lib package with debug mode
	cargo build --lib
.PHONY: build\:debug\:lib

build\:debug\:cli: # Build only -cli package with debug mode
	cargo build --bin $(PACKAGE)-cli
.PHONY: build\:debug\:cli

build\:debug\:tui: # Build only -tui package with debug mode
	cargo build --bin $(PACKAGE)-tui
.PHONY: build\:debug\:tui

build\:release\:lib: # Build only lib package with release mode
	cargo build --lib --release
.PHONY: build\:release\:lib

build\:release\:cli: # Build only -cli package with release mode
	cargo build --bin $(PACKAGE)-cli --release
.PHONY: build\:release\:cli

build\:release\:tui: # Build only -tui package with release mode
	cargo build --bin $(PACKAGE)-tui --release
.PHONY: build\:release\:tui

build\:debug: # Build all packages with debug mode [synonym: build]
	cargo build --workspace
.PHONY: build\:debug

build\:release: # Build all packages with release mode
	cargo build --workspace --release
.PHONY: build\:release

build: build\:debug
.PHONY: build

# util
watch\:lib: # Monitor build process for lib package (require cargo-watch)
	@cargo watch --exec 'build --lib' --delay 0.3
.PHONY: watch\:lib

watch\:cli: # Monitor build process for -cli package (require cargo-watch)
	@cargo watch --exec 'build --bin $(PACKAGE)-cli' --delay 0.3
.PHONY: watch\:cli

watch\:tui: # Monitor build process for -tui package (require cargo-watch)
	@cargo watch --exec 'build --bin $(PACKAGE)-tui' --delay 0.3
.PHONY: watch\:tui

clean: # Remove cache and built artifacts
	@cargo clean
.PHONY: clean

package\:%: # Create a package
	@cargo package --manifest-path src/$(subst package:,,$@)/Cargo.toml
.PHONY: package

help: # Display this message
	@set -uo pipefail; \
	grep --extended-regexp '^[0-9a-z\:\\\%]+: ' \
		$(firstword $(MAKEFILE_LIST)) | \
		grep --extended-regexp ' # ' | \
		sed --expression='s/\([a-z0-9\-\:\ ]*\): \([a-z0-9\-\:\ ]*\) #/\1: #/g' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ": # "}; \
			{printf "\033[38;05;222m%-18s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help

.DEFAULT_GOAL = build\:release
default: build\:release
