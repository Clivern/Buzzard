CARGO ?= cargo
RUSTUP ?= rustup


help: Makefile
	@echo
	@echo " Choose a command run in Buzzard:"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
	@echo


## config: Install dependencies.
config:
	$(RUSTUP) component add rustfmt


## build: Build binary
build:
	@echo "\n>> ============= Cargo Build ============= <<"
	rm -rf target
	$(CARGO) build --verbose --all


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


.PHONY: help