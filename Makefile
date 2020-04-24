NAME = kube-rust-playground

K3S_IMAGE = docker.io/rancher/k3s:v1.17.4-k3s1
# OR: docker.io/rancher/k3s:v1.18.2-rc2-k3s1-amd64
# OR: docker.io/rancher/k3s:latest
# check for other candidates:
# https://hub.docker.com/r/rancher/k3s/tags?page=1&ordering=last_updated

K3D_REGISTRY_NAME = localhost
K3D_REGISTRY_PORT = 5000
K3D_REGISTRY = $(K3D_REGISTRY_NAME):$(K3D_REGISTRY_PORT)

K3D_CLUSTER_FLAGS = --name=$(NAME)

K3D_PUBLIC_HTTP_PORT = 18080

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

DOCKER_NETWORK = k3d-$(NAME)

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

REGISTRY_TEST_IMAGE = busybox

registry-test:
	docker pull $(REGISTRY_TEST_IMAGE):latest
	docker tag $(REGISTRY_TEST_IMAGE):latest $(K3D_REGISTRY)/$(REGISTRY_TEST_IMAGE):latest
	docker push $(K3D_REGISTRY)/$(REGISTRY_TEST_IMAGE):latest

# https://github.com/rancher/k3d/blob/master/docs/examples.md#expose-services
create-ingress:
	kubectl create deployment nginx --image=nginx
	kubectl create service clusterip nginx --tcp=80:80
	kubectl apply -f k3d-data/ingress.yml
	@echo "Should be reachable on port $(K3D_PUBLIC_HTTP_PORT) now,"
	@echo "Try: curl http://localhost:$(K3D_PUBLIC_HTTP_PORT)/"
