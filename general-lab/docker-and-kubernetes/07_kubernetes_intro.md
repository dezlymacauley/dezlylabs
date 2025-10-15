what is kubernetes?

It is an open-source tool, created by Google that 
is the standard for container orchestration.

kubernetes is used to address the complexity of running containers in
production. It automates the process.

Kubernetes uses a declarative style. So you declare the final state and then
let Kubernetes figure out how to get there.

Kubernetes can also monitor if there is any difference between the current
state of the cluster and the desired final state.

Kubernetes also makes it very easy to implement horizontal scaling.

Resilience, Kubernetes ensures that applications are always available even
in the face of failures.

Kubernetes also makes it easy to roll out new versions of an application.

control plane vs data plane

nodes and kubernetes objects

kubectl (the kubernetes cli)

kubectl vs kubernetes cluster

how does kubectl communicate with the cluster

structure of kubectl commands.
_______________________________________________________________________________
how kubernetes comes into the picture and why docker by itself is not enough.

Docker and Docker Compose alone are enough for running individual containers
or creating development environments but they do not meet the requirements of
production workloads.

Benefits of Kubernetes:
1. Container Scheduling
2. Load Balancing
3. Scaling Application
4. Self-Healing (health checks on containers)
5. Service Discovery

_______________________________________________________________________________

Kubernetes cluster is made up of:

- Control plane = contains the components required to manage 
the entire system.

- Worker nodes = The machines where you application actually runs.

_______________________________________________________________________________
### Install The kubernetes cli

kubectl [command] [resource type] [name] [flags]

To check that you have this installed:

```sh
kubectl version --client
```
_______________________________________________________________________________
### Install Docker

And start the docker service

_______________________________________________________________________________
### Install minikube

Start minikube
```sh
minikube start
```
❯ minikube start
😄  minikube v1.37.0 on Arch
✨  Automatically selected the docker driver
📌  Using Docker driver with root privileges
👍  Starting "minikube" primary control-plane node in "minikube" cluster
🚜  Pulling base image v0.0.48 ...
💾  Downloading Kubernetes v1.34.0 preload ...
    > preloaded-images-k8s-v18-v1...:  337.07 MiB / 337.07 MiB  100.00% 1.09 Mi
    > gcr.io/k8s-minikube/kicbase...:  488.51 MiB / 488.52 MiB  100.00% 790.05
🔥  Creating docker container (CPUs=2, Memory=3072MB) ...
🐳  Preparing Kubernetes v1.34.0 on Docker 28.4.0 ...
🔗  Configuring bridge CNI (Container Networking Interface) ...
🔎  Verifying Kubernetes components...
    ▪ Using image gcr.io/k8s-minikube/storage-provisioner:v5
🌟  Enabled addons: storage-provisioner, default-storageclass
🏄  Done! kubectl is now configured to use "minikube" cluster and "default" namespace by default


_______________________________________________________________________________

To check that everything is working:

Run this command (you can leave out the `--client` now)
```sh
kubectl version
```
Client Version: v1.34.1
Kustomize Version: v5.7.1
Server Version: v1.34.0

_______________________________________________________________________________
