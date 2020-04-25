include ../shared.mk

build:
	cargo build

image:
	$(BK_BUILD) -t $(SWS_IMAGE_FULL) .
	docker push $(SWS_IMAGE_FULL)
