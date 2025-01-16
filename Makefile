.PHONY: help build build-release build-wasm test test-fast clean compile run fmt lint

# default target
.DEFAULT_GOAL := help

# build the layman compiler
build:
	@echo "building layman compiler..."
	cargo build

# build release version
build-release:
	@echo "building layman compiler (release)..."
	cargo build --release

# build wasm module for web
build-wasm:
	@echo "building layman wasm module..."
	@if ! command -v wasm-pack >/dev/null 2>&1; then \
		echo "installing wasm-pack..."; \
		cargo install wasm-pack --locked; \
	fi
	@cd wasm && wasm-pack build --target web --out-dir ../../layman-site/public/wasm
	@echo "wasm module built successfully in layman-site/public/wasm/"

# run all tests (compile + execute each test case)
# includes all .lay files recursively (positive and negative tests)
test:
	@echo "running all tests (including negative test cases)..."
	@cargo run -- test test-cases
	@echo "running rust integration tests..."
	@cargo test

# run tests with verbose output
test-verbose:
	@echo "running all tests (verbose)..."
	cargo run -- test test-cases --verbose

# run a specific test file
test-file:
	@if [ -z "$(FILE)" ]; then \
		echo "usage: make test-file FILE=test-cases/test_001.lay"; \
	else \
		cargo run -- $(FILE); \
	fi

# compile a specific file
compile:
	@if [ -z "$(FILE)" ]; then \
		echo "usage: make compile FILE=hello.lay"; \
	else \
		cargo run -- compile $(FILE); \
	fi

# run a compiled file
run:
	@if [ -z "$(FILE)" ]; then \
		echo "usage: make run FILE=hello.layc"; \
	else \
		cargo run -- run-compiled $(FILE); \
	fi

# format code
fmt:
	@echo "formatting code..."
	cargo fmt

# check linting
lint:
	@echo "checking linting..."
	cargo clippy -- -D warnings

# clean build artifacts
clean:
	@echo "cleaning build artifacts..."
	cargo clean
	find . -name "*.layc" -delete
	find . -name "hello" -delete
	find test-cases -type f -executable -delete

# clean test artifacts (compiled files)
clean-tests:
	@echo "cleaning test artifacts..."
	find test-cases -name "*.layc" -delete
	find test-cases -type f -perm +111 -name "test_*" -delete || true

# verify all tests compile and run successfully (with output comparison)
verify: build
	@echo "verifying all tests (compile + run + compare output)..."
	@cargo run -- verify test-cases || true

# generate expected output files for all tests
generate-expected: build
	@echo "generating expected output files..."
	@./generate-expected.sh test-cases

# show help
help:
	@echo "layman compiler makefile"
	@echo ""
	@echo "targets:"
	@echo "  build           - build the compiler"
	@echo "  build-release   - build release version"
	@echo "  build-wasm      - build wasm module for web playground"
	@echo "  test            - run all tests"
	@echo "  test-verbose    - run tests with verbose output"
	@echo "  test-file       - run a specific test (FILE=path)"
	@echo "  compile         - compile a file (FILE=path)"
	@echo "  run             - run a compiled file (FILE=path)"
	@echo "  fmt             - format code"
	@echo "  lint            - check linting"
	@echo "  clean           - clean all build artifacts"
	@echo "  clean-tests     - clean test artifacts only"
	@echo "  verify          - clean, build, and test"
	@echo "  help            - show this help"
	@echo ""
	@echo "examples:"
	@echo "  make test"
	@echo "  make test-file FILE=test-cases/test_001.lay"
	@echo "  make compile FILE=hello.lay"
	@echo "  make run FILE=hello.layc"

