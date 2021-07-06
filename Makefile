CARGO ?= cargo
RUSTUP ?= rustup


help: Makefile
	@echo
	@echo " Choose a command to run in Buzzard:"
	@echo
	@sed -n 's/^##//p' $< | awk -F ':' '{printf "  %-25s %s\n", $$1, $$2}'
	@echo


## config: Install dependencies.
config:
	$(RUSTUP) component add rustfmt


## build: Build binary
build:
	@echo "\n>> ============= Cargo Build ============= <<"
	rm -rf target
	$(CARGO) build --verbose --all


## version: Show version of cargo and rustup
version:
	@echo "\n>> ============= Cargo Version ============= <<"
	$(CARGO) --version
	@echo "\n>> ============= Rustup Version ============= <<"
	$(RUSTUP) --version


## release: Build releases
release:
	@echo "\n>> ============= Cargo Release ============= <<"
	rm -rf target
	$(CARGO) build --release --verbose


## test: Run test cases
test:
	@echo "\n>> ============= Cargo Test ============= <<"
	$(CARGO) test --verbose --all


## fmt: Format code
fmt:
	@echo "\n>> ============= Cargo Format ============= <<"
	$(CARGO) fmt


## fmt_check: Check format
fmt_check:
	@echo "\n>> ============= Cargo Format Check ============= <<"
	$(CARGO) fmt -- --check


## run: Run project
run:
	@echo "\n>> ============= Cargo Run ============= <<"
	$(CARGO) run


## ci: Run all CI tests.
ci: build test fmt_check
	@echo "\n>> ============= All quality checks passed ============= <<"


## clean: Clean build artifacts
clean:
	@echo "\n>> ============= Cleaning build artifacts ============= <<"
	rm -rf target


.PHONY: help