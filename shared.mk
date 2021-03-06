NAME = kube-rust-playground

# k3d:
# curl -s https://raw.githubusercontent.com/rancher/k3d/master/install.sh | TAG=v3.0.0-beta.1 bash

# kubebox
# curl -Lo ~/bin/kubebox https://github.com/astefanutti/kubebox/releases/download/v0.8.0/kubebox-linux && chmod +x ~/bin/kubebox

# K3S_IMAGE = docker.io/rancher/k3s:v1.17.5-k3s1
K3S_IMAGE = docker.io/rancher/k3s:v1.18.2-k3s1
# OR: docker.io/rancher/k3s:latest
# check for other candidates:
# https://hub.docker.com/r/rancher/k3s/tags?page=1&ordering=last_updated

K3D = k3d
K3D_REGISTRY_NAME = registry.localhost
K3D_REGISTRY_PORT = 5000
K3D_REGISTRY = $(K3D_REGISTRY_NAME):$(K3D_REGISTRY_PORT)
K3D_CLUSTER_FLAGS = --name $(NAME)
K3D_PUBLIC_HTTP_PORT = 18080

DOCKER = docker
DOCKER_BUILDKIT ?= 1
BK_BUILD = $(DOCKER) build --progress=plain -f Dockerfile
DOCKER_NETWORK = k3d-$(NAME)

DC = docker-compose
DC_RUN = $(DC) run --rm
DC_UP = $(DC) up -d


KUBECTL = bin/kubectl

REGISTRY_TEST_IMAGE = busybox

SWS_IMAGE_NAME = simple-web-service
SWS_IMAGE_REPO = $(K3D_REGISTRY)/$(SWS_IMAGE_NAME)
SWS_IMAGE_VER  = 1.0.0
SWS_IMAGE_SIMPE = $(SWS_IMAGE_NAME):$(SWS_IMAGE_VER)
SWS_IMAGE_FULL = $(SWS_IMAGE_REPO):$(SWS_IMAGE_VER)
SWS_IMAGE_PORT = 8080

WRK_SETTINGS =  -t 4 -c 10 -R 200 -d 30 -L
