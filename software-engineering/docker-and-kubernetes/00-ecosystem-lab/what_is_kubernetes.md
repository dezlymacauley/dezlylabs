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
_______________________________________________________________________________

To check that everything is working:

Run this command (you can leave out the dash)
```sh
kubectl version
```

_______________________________________________________________________________
