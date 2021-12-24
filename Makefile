.DEFAULT_GOAL := help

export CONTAINER_TOOL ?= docker

# "One weird trick!" https://www.gnu.org/software/make/manual/make.html#Syntax-of-Functions
EMPTY:=
SPACE:= ${EMPTY} ${EMPTY}

##################################################
##################################################

.PHONY: help
help:
	@awk 'BEGIN {FS = ":.*##"; printf "Usage: make ${FORMATTING_BEGIN_BLUE}<target>${FORMATTING_END}\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  ${FORMATTING_BEGIN_BLUE}%-46s${FORMATTING_END} %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##################################################
##################################################
# Cross compilations

.PHONY: build-x86_64-unknown-linux-gnu
build-x86_64-unknown-linux-gnu: target/x86_64-unknown-linux-gnu/release/mqrt ## Build a release binary for the x86_64-unknown-linux-gnu triple.
	@echo "Output to ${<}"

.PHONY: build-aarch64-unknown-linux-gnu
build-aarch64-unknown-linux-gnu: target/aarch64-unknown-linux-gnu/release/mqrt ## Build a release binary for the aarch64-unknown-linux-gnu triple.
	@echo "Output to ${<}"

.PHONY: build-x86_64-unknown-linux-musl
build-x86_64-unknown-linux-musl: target/x86_64-unknown-linux-musl/release/mqrt ## Build a release binary for the x86_64-unknown-linux-musl triple.
	@echo "Output to ${<}"

.PHONY: build-aarch64-unknown-linux-musl
build-aarch64-unknown-linux-musl: target/aarch64-unknown-linux-musl/release/mqrt ## Build a release binary for the aarch64-unknown-linux-musl triple.
	@echo "Output to ${<}"

.PHONY: build-armv7-unknown-linux-gnueabihf
build-armv7-unknown-linux-gnueabihf: target/armv7-unknown-linux-gnueabihf/release/mqrt ## Build a release binary for the armv7-unknown-linux-gnueabihf triple.
	@echo "Output to ${<}"

.PHONY: build-armv7-unknown-linux-musleabihf
build-armv7-unknown-linux-musleabihf: target/armv7-unknown-linux-musleabihf/release/mqrt ## Build a release binary for the armv7-unknown-linux-musleabihf triple.
	@echo "Output to ${<}"

.PHONY: CARGO_HANDLES_FRESHNESS
CARGO_HANDLES_FRESHNESS:
	${EMPTY}

# GNU Make < 3.82 pattern matching priority depends on the definition order
# so cross-image-% must be defined before cross-%
.PHONY: cross-image-%
cross-image-%: export TRIPLE =$($(strip @):cross-image-%=%)
cross-image-%:
	$(CONTAINER_TOOL) build \
		--tag mqrt-build-cross-env:${TRIPLE} \
		--file scripts/cross/${TRIPLE}.dockerfile \
		scripts/cross

target/%/mqrt: export PAIR =$(subst /, ,$(@:target/%/mqrt=%))
target/%/mqrt: export TRIPLE ?=$(word 1,${PAIR})
target/%/mqrt: export PROFILE ?=$(word 2,${PAIR})
target/%/mqrt: export CFLAGS += -g0 -O3
target/%/mqrt: cargo-install-cross CARGO_HANDLES_FRESHNESS
	$(MAKE) -k cross-image-${TRIPLE}
	cross build \
		$(if $(findstring release,$(PROFILE)),--release,) \
		--target ${TRIPLE}

# Cargo
.PHONY: cargo-install-%
cargo-install-%: override TOOL = $(@:cargo-install-%=%)
cargo-install-%:
	cargo install ${TOOL} --quiet; cargo clean

##################################################
##################################################
# Other
.PHONY: version
version: ## Get the current MQRT version
	@scripts/version.sh
