include shared.mk

K3D_CLUSTER_CREATE_FLAGS = \
	$(K3D_CLUSTER_FLAGS) \
	--image "$(K3S_IMAGE)" \
	--enable-registry \
	--registry-volume $(NAME)-registry \
	--registry-name $(K3D_REGISTRY_NAME) \
	--registry-port $(K3D_REGISTRY_PORT) \
	--auto-restart \
	--api-port=6550 \
	--publish $(K3D_PUBLIC_HTTP_PORT):80 \
	--workers 3

K3D_CLUSTER_DELETE_FLAGS = \
	$(K3D_CLUSTER_FLAGS) \
	--prune

get-kubeconfig:
	@$(K3D) get-kubeconfig --name=$(NAME)

cluster:
	$(K3D) --verbose create $(K3D_CLUSTER_CREATE_FLAGS)
	@echo "Waiting a bit (9 sec) ..."
	@sleep 9
	@echo
	@$(KUBECTL) cluster-info

namespaces:
	@$(KUBECTL) apply -f k3d-data/namespaces.yml

clean-cluster:
	$(K3D) delete $(K3D_CLUSTER_DELETE_FLAGS)

registry-test:
	$(DOCKER) pull $(REGISTRY_TEST_IMAGE):latest
	$(DOCKER) tag $(REGISTRY_TEST_IMAGE):latest $(K3D_REGISTRY)/$(REGISTRY_TEST_IMAGE):latest
	$(DOCKER) push $(K3D_REGISTRY)/$(REGISTRY_TEST_IMAGE):latest

create-nginx-test:
	$(KUBECTL) apply -f k3d-data/nginx.yml
	@echo "nginx should be reachable under /default soon."

delete-nginx-test:
	$(KUBECTL) delete -f k3d-data/nginx.yml

build-image-$(SWS_IMAGE_NAME):
	$(MAKE) -C $(SWS_IMAGE_NAME) image
	$(DOCKER) images $(K3D_REGISTRY)/*

create-$(SWS_IMAGE_NAME):
	$(MAKE) -C $(SWS_IMAGE_NAME) image-push
	sed 's!__SWS_IMAGE_FULL__!$(SWS_IMAGE_FULL)!g' k3d-data/$(SWS_IMAGE_NAME).yml | $(KUBECTL) apply -f -
	@echo "workload should be reachable under any route (but not /default) soon."

delete-$(SWS_IMAGE_NAME):
	$(KUBECTL) delete -f k3d-data/$(SWS_IMAGE_NAME).yml

create-all: create-nginx-test create-$(SWS_IMAGE_NAME)

delete-all: delete-nginx-test delete-$(SWS_IMAGE_NAME)

# https://octant.dev/
# brew install octant
octant:
	KUBECONFIG=$(shell k3d get-kubeconfig --name=$(NAME)) octant

request:
	curl http://localhost:$(K3D_PUBLIC_HTTP_PORT)/hello

# wrk2
perftest:
	# nginx:
	wrk $(WRK_SETTINGS) http://localhost:$(K3D_PUBLIC_HTTP_PORT)/default
	# simple web service:
	wrk $(WRK_SETTINGS) http://localhost:$(K3D_PUBLIC_HTTP_PORT)/rusty-service

# -----

### https://github.com/fission/fission

# CLI:
# curl -Lo fission https://github.com/fission/fission/releases/download/1.9.0/fission-cli-linux && chmod +x fission && sudo mv fission /usr/local/bin/
fission:
	$(KUBECTL) create namespace fission
	with-k3d helm install \
		--namespace fission \
		--name-template fission \
        https://github.com/fission/fission/releases/download/1.9.0/fission-all-1.9.0.tgz

fission-envs:
	with-k3d fission env create --name nodejs --image fission/node-env
	with-k3d fission env create --name binary-env --image fission/ruby-env
	with-k3d fission env create --name binary-env --image fission/binary-env

fission-fn:
	curl https://raw.githubusercontent.com/fission/fission/master/examples/nodejs/hello.js > /tmp/hello.js
	with-k3d fission function create --name hello --env nodejs --code /tmp/hello.js
	with-k3d fission function test --name hello
	with-k3d fission httptrigger create --name hello --url /hello --method GET --function hello --createingress

# -----
# wrk tests:
# wrk -c 32 -t 16 -d 120 -R 512 -L http://127.0.0.1:18080/default
# wrk -c 32 -t 16 -d 120 -R 512 -L http://127.0.0.1:18080/hello
# wrk -c 32 -t 16 -d 120 -R 512 -L http://127.0.0.1:18080/my-rusty-service
