# kube-rust-playground

## Prerequisites

* direnv (<https://direnv.net/>)
* make
* docker
* kubectl
* k3d (<https://github.com/rancher/k3d>)
* rust (if you want to develop/debug on the host)

Install via your favourite system and package/dependency manager.

## Usage

```sh
make cluster
make create-simple-web-service
curl http://localhost:18080/hello
```
