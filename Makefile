.DEFAULT_GOAL := help

export VERSION ?= $(shell scripts/version.sh)
export CONTAINER_TOOL ?= docker


define relocate_artifact
	@echo "Built to ${1}, relocating to ${2}"
	@mkdir -p target/artifacts/
	cp -v ${1} ${2}
endef


################################################################################
################################################################################
################################################################################

.PHONY: help
help:
	@awk 'BEGIN {FS = ":.*##"; printf "Usage: make ${FORMATTING_BEGIN_BLUE}<target>${FORMATTING_END}\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  ${FORMATTING_BEGIN_BLUE}%-46s${FORMATTING_END} %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

.PHONY: version
version: ## Get the current MQRT version
	@scripts/version.sh

.PHONY: build
build: ## Build the project in dev mode
	cargo build

.PHONY: build-release
build-release: export CFLAGS += -g0 -O3
build-release: ## Build the project in release mode
	cargo build --release

.PHONY: clean
clean: ## Clean everything
	cargo clean



################################################################################
################################################################################
################################################################################
## build


.PHONY: build-x86_64-unknown-linux-gnu
build-x86_64-unknown-linux-gnu: target/x86_64-unknown-linux-gnu/release/mqrt ## Build a release binary for the x86_64-unknown-linux-gnu triple.
	@echo "Output to ${<}"

.PHONY: build-x86_64-unknown-linux-musl
build-x86_64-unknown-linux-musl: target/x86_64-unknown-linux-musl/release/mqrt ## Build a release binary for the x86_64-unknown-linux-musl triple.
	@echo "Output to ${<}"

.PHONY: build-aarch64-unknown-linux-gnu
build-aarch64-unknown-linux-gnu: target/aarch64-unknown-linux-gnu/release/mqrt ## Build a release binary for the aarch64-unknown-linux-gnu triple.
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


target/%/mqrt: export PAIR =$(subst /, ,$(@:target/%/mqrt=%))
target/%/mqrt: export TRIPLE ?=$(word 1,${PAIR})
target/%/mqrt: export PROFILE ?=$(word 2,${PAIR})
target/%/mqrt: export CFLAGS += -g0 -O3
target/%/mqrt:
	$(MAKE) -k cross-image-${TRIPLE}
	cross build \
		$(if $(findstring release,$(PROFILE)),--release,) \
		--target ${TRIPLE}



################################################################################
################################################################################
################################################################################
# package-all-%

.PHONY: package-all-x86_64-unknown-linux-gnu
package-all-x86_64-unknown-linux-gnu: package-tar-x86_64-unknown-linux-gnu package-deb-amd64 ## Build all for `x86_64-unknown-linux-gnu`

.PHONY: package-all-x86_64-unknown-linux-musl
package-all-x86_64-unknown-linux-musl: package-tar-x86_64-unknown-linux-musl ## Build all for `x86_64-unknown-linux-musl`

.PHONY: package-all-aarch64-unknown-linux-gnu
package-all-aarch64-unknown-linux-gnu: package-tar-aarch64-unknown-linux-gnu package-deb-arm64 ## Build all for `aarch64-unknown-linux-gnu`

.PHONY: package-all-aarch64-unknown-linux-musl
package-all-aarch64-unknown-linux-musl: package-tar-aarch64-unknown-linux-musl ## Build all for `aarch64-unknown-linux-musl`

.PHONY: package-all-armv7-unknown-linux-gnueabihf
package-all-armv7-unknown-linux-gnueabihf: package-tar-armv7-unknown-linux-gnueabihf package-deb-armhf ## Build all for `armv7-unknown-linux-gnueabihf`

.PHONY: package-all-armv7-unknown-linux-musleabihf
package-all-armv7-unknown-linux-musleabihf: package-tar-armv7-unknown-linux-musleabihf ## Build all for `armv7-unknown-linux-musleabihf`



################################################################################
################################################################################
################################################################################
# package-tar-%

.PHONY: package-tar-x86_64-unknown-linux-gnu
package-tar-x86_64-unknown-linux-gnu: target/artifacts/mqrt-${VERSION}-x86_64-unknown-linux-gnu.tar.gz ## Build and create an archive suitable for the `x86_64-unknown-linux-gnu` triple.
	@echo "Output to ${<}."

.PHONY: package-tar-x86_64-unknown-linux-musl
package-tar-x86_64-unknown-linux-musl: target/artifacts/mqrt-${VERSION}-x86_64-unknown-linux-musl.tar.gz ## Build and create an archive suitable for the `x86_64-unknown-linux-musl` triple.
	@echo "Output to ${<}."

.PHONY: package-tar-aarch64-unknown-linux-musl
package-tar-aarch64-unknown-linux-musl: target/artifacts/mqrt-${VERSION}-aarch64-unknown-linux-musl.tar.gz ## Build and create an archive suitable for the `aarch64-unknown-linux-musl` triple.
	@echo "Output to ${<}."

.PHONY: package-tar-aarch64-unknown-linux-gnu
package-tar-aarch64-unknown-linux-gnu: target/artifacts/mqrt-${VERSION}-aarch64-unknown-linux-gnu.tar.gz ## Build and create an archive suitable for the `aarch64-unknown-linux-gnu` triple.
	@echo "Output to ${<}."

.PHONY: package-tar-armv7-unknown-linux-gnueabihf
package-tar-armv7-unknown-linux-gnueabihf: target/artifacts/mqrt-${VERSION}-armv7-unknown-linux-gnueabihf.tar.gz ## Build and create an archive suitable for the `armv7-unknown-linux-gnueabihf` triple.
	@echo "Output to ${<}."

.PHONY: package-tar-armv7-unknown-linux-musleabihf
package-tar-armv7-unknown-linux-musleabihf: target/artifacts/mqrt-${VERSION}-armv7-unknown-linux-musleabihf.tar.gz ## Build and create an archive suitable for the `armv7-unknown-linux-musleabihf triple.
	@echo "Output to ${<}."


target/artifacts/mqrt-${VERSION}-%.tar.gz: export TRIPLE=$(@:target/artifacts/mqrt-${VERSION}-%.tar.gz=%)
target/artifacts/mqrt-${VERSION}-%.tar.gz: override PROFILE =release
target/artifacts/mqrt-${VERSION}-%.tar.gz: target/%/release/mqrt.tar.gz
	$(call relocate_artifact,${<},${@})

target/%/mqrt.tar.gz: export PAIR =$(subst /, ,$(@:target/%/mqrt.tar.gz=%))
target/%/mqrt.tar.gz: export TRIPLE ?=$(word 1,${PAIR})
target/%/mqrt.tar.gz: export PROFILE ?=$(word 2,${PAIR})
target/%/mqrt.tar.gz: target/%/mqrt
	rm -rf target/scratch/mqrt-${TRIPLE} || true
	mkdir -p target/scratch/mqrt-${TRIPLE}/bin target/scratch/mqrt-${TRIPLE}/etc/mqrt
	cp --recursive --force --verbose \
		target/${TRIPLE}/${PROFILE}/mqrt \
		target/scratch/mqrt-${TRIPLE}/bin/mqrt
	cp --recursive --force --verbose \
    		distribution/config/* \
    		target/scratch/mqrt-${TRIPLE}/etc/mqrt
	cp --recursive --force --verbose \
		README.md \
		LICENSE \
		target/scratch/mqrt-${TRIPLE}/
	cp --recursive --force --verbose \
		distribution/systemd \
		target/scratch/mqrt-${TRIPLE}/etc/
	tar --create \
		--gzip \
		--verbose \
		--file target/${TRIPLE}/${PROFILE}/mqrt.tar.gz \
		--directory target/scratch/ \
		./mqrt-${TRIPLE}
	rm -rf target/scratch/



################################################################################
################################################################################
################################################################################
# package-deb-%

.PHONY: package-deb-x86_64-unknown-linux-gnu
package-deb-amd64: target/artifacts/mqrt-${VERSION}-amd64.deb ## Build and create the deb package for `x86_64-unknown-linux-gnu`
	@echo "Output to ${<}."

.PHONY: package-deb-aarch64-unknown-linux-gnu
package-deb-arm64: target/artifacts/mqrt-${VERSION}-arm64.deb ## Build and create the deb package for `aarch64-unknown-linux-gnu`.
	@echo "Output to ${<}."

.PHONY: package-deb-armv7-unknown-linux-gnueabihf
package-deb-armhf: target/artifacts/mqrt-${VERSION}-armhf.deb ## Build and create the deb package for `armv7-unknown-linux-gnueabihf`.
	@echo "Output to ${<}."


target/artifacts/mqrt-${VERSION}-amd64.deb: target/x86_64-unknown-linux-gnu/debian/mqrt-${VERSION}.deb
	$(call relocate_artifact,${<},${@})

target/artifacts/mqrt-${VERSION}-arm64.deb: target/aarch64-unknown-linux-gnu/debian/mqrt-${VERSION}.deb
	$(call relocate_artifact,${<},${@})

target/artifacts/mqrt-${VERSION}-armhf.deb: target/armv7-unknown-linux-gnueabihf/debian/mqrt-${VERSION}.deb
	$(call relocate_artifact,${<},${@})


target/%/debian/mqrt-${VERSION}.deb: export TARGET=$(@:target/%/debian/mqrt-${VERSION}.deb=%)
target/%/debian/mqrt-${VERSION}.deb: override PROFILE =release
target/%/debian/mqrt-${VERSION}.deb: target/%/mqrt
	@cargo deb --target "${TARGET}" --deb-version "${VERSION}" --variant "${TARGET}" --no-build --no-strip --output "${@}"



################################################################################
################################################################################
################################################################################

.PHONY: cross-image-%
cross-image-%: export TRIPLE =$($(strip @):cross-image-%=%)
cross-image-%:
	$(CONTAINER_TOOL) build \
		--tag mqrt-build-cross-env:${TRIPLE} \
		--file scripts/cross/${TRIPLE}.dockerfile \
		scripts/cross

# Cargo
.PHONY: cargo-install-%
cargo-install-%: override TOOL = $(@:cargo-install-%=%)
cargo-install-%:
	cargo install ${TOOL} --quiet; cargo clean



################################################################################
################################################################################
################################################################################

.PHONY: docker-release
docker-release: export PLATFORM=linux/amd64,linux/arm64,linux/arm/v7
docker-release: ## Build docker containers and push. Supports DOCKER_PUSH and DOCKER_REGISTRY
	@scripts/docker-build.sh



################################################################################
################################################################################
################################################################################

.PHONY: ci-prepare
ci-prepare: cargo-install-cross cargo-install-cargo-deb
