include ../shared.mk

build:
	cargo build

# debian: needs gcc-multilib, musl-tools (and maybe other deps)
# rustup target add x86_64-unknown-linux-musl
build.musl:
	cargo build --target=x86_64-unknown-linux-musl

release:
	cargo build --release
	
release.musl:
	cargo build --release --target=x86_64-unknown-linux-musl

image:
	$(BK_BUILD) -t $(SWS_IMAGE_FULL) --squash .

image.builder:
	$(BK_BUILD) -t $(SWS_IMAGE_FULL)-builder .

image-push: image
	docker push $(SWS_IMAGE_FULL)
