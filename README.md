# kube-rust-playground

## Prerequisites

* direnv (<https://direnv.net/>)
* make
* docker
* kubectl
* k3d (<https://github.com/rancher/k3d>)
* rust (if you want to develop/debug on the host)

Install via your favourite system and package/dependency manager.

* add `127.0.0.1 registry.localhost` to `/etc/hosts`

## Usage

```sh
make cluster
make create-simple-web-service
curl http://localhost:18080/hello
```

Finished with the work?

```sh
# only the service
make delete-simple-web-service

# everything
make clean-cluster
```

## Scratchpad

* <http://technosophos.com/2019/08/07/writing-a-kubernetes-controller-in-rust.html>
* <https://github.com/technosophos/rust-k8s-controller/>
* <https://github.com/pacman82/throttle/blob/master/src/favicon.rs>
* <https://github.com/async-rs/async-std/pull/733/files#diff-a8556a073aa4f101599aca791fd46accR15-R23>
* <https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=8004f1ac04b47c6db4ad12af74782a06>
