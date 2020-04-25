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
	@k3d get-kubeconfig --name=$(NAME)

cluster:
	k3d create $(K3D_CLUSTER_CREATE_FLAGS)
	@echo "Waiting a bit (5sec) ..."
	@sleep 5
	@echo
	@kubectl cluster-info

clean-cluster:
	k3d delete $(K3D_CLUSTER_DELETE_FLAGS)

registry-test:
	docker pull $(REGISTRY_TEST_IMAGE):latest
	docker tag $(REGISTRY_TEST_IMAGE):latest $(K3D_REGISTRY)/$(REGISTRY_TEST_IMAGE):latest
	docker push $(K3D_REGISTRY)/$(REGISTRY_TEST_IMAGE):latest

create-psp:
	kubectl apply -f k3d-data/psp.yml

create-$(SWS_IMAGE_NAME):
	cd $(SWS_IMAGE_NAME) && $(MAKE) image
	kubectl apply -f k3d-data/$(SWS_IMAGE_NAME).yml
	kubectl apply -f k3d-data/ingress.yml

delete-$(SWS_IMAGE_NAME):
	kubectl delete -f k3d-data/ingress.yml
	kubectl delete -f k3d-data/$(SWS_IMAGE_NAME).yml

# https://octant.dev/
# brew install octant
octant:
	KUBECONFIG=$(shell k3d get-kubeconfig --name=$(NAME)) octant

request:
	curl http://localhost:$(K3D_PUBLIC_HTTP_PORT)/hello

# wrk2
perftest:
	wrk $(WRK_SETTINGS) http://localhost:$(K3D_PUBLIC_HTTP_PORT)/hello
