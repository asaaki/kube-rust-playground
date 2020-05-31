include shared.mk

K3D_CLUSTER_CREATE_FLAGS = \
	$(K3D_CLUSTER_FLAGS) \
	--image $(K3S_IMAGE) \
	--enable-registry \
	--registry-volume $(NAME)-registry \
	--registry-name $(K3D_REGISTRY_NAME) \
	--registry-port $(K3D_REGISTRY_PORT) \
	--auto-restart \
	--api-port=6550 \
	--publish $(K3D_PUBLIC_HTTP_PORT):80 \
	--workers 2

K3D_CLUSTER_DELETE_FLAGS = \
	$(K3D_CLUSTER_FLAGS) \
	--prune

get-kubeconfig:
	@$(K3D) get-kubeconfig --name=$(NAME)

cluster:
	$(K3D) create $(K3D_CLUSTER_CREATE_FLAGS)
	@echo "Waiting a bit (5sec) ..."
	@sleep 5
	@echo
	@$(KUBECTL) cluster-info

clean-cluster:
	$(K3D) delete $(K3D_CLUSTER_DELETE_FLAGS)

registry-test:
	$(DOCKER) pull $(REGISTRY_TEST_IMAGE):latest
	$(DOCKER) tag $(REGISTRY_TEST_IMAGE):latest $(K3D_REGISTRY)/$(REGISTRY_TEST_IMAGE):latest
	$(DOCKER) push $(K3D_REGISTRY)/$(REGISTRY_TEST_IMAGE):latest

create-psp:
	$(KUBECTL) apply -f k3d-data/psp.yml

build-image-$(SWS_IMAGE_NAME):
	$(MAKE) -C $(SWS_IMAGE_NAME) image
	$(DOCKER) images $(K3D_REGISTRY)/*

create-$(SWS_IMAGE_NAME):
	$(MAKE) -C $(SWS_IMAGE_NAME) image-push
	sed 's!__SWS_IMAGE_FULL__!$(SWS_IMAGE_FULL)!g' k3d-data/$(SWS_IMAGE_NAME).yml | $(KUBECTL) apply -f -
	$(KUBECTL) apply -f k3d-data/ingress.yml

delete-$(SWS_IMAGE_NAME):
	$(KUBECTL) delete -f k3d-data/ingress.yml
	$(KUBECTL) delete -f k3d-data/$(SWS_IMAGE_NAME).yml

# https://octant.dev/
# brew install octant
octant:
	KUBECONFIG=$(shell k3d get-kubeconfig --name=$(NAME)) octant

request:
	curl http://localhost:$(K3D_PUBLIC_HTTP_PORT)/hello

# wrk2
perftest:
	wrk $(WRK_SETTINGS) http://localhost:$(K3D_PUBLIC_HTTP_PORT)/hello
