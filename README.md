# Project2: Getting Information from IMDB

## Introduction

This project is about getting information from IMDB. The information is about the title, rate and cast of the movie.

## Usage

### installation

To have it run locally, first please have rust installed on your computer.  

To have in run on Kubernetes, first please install minikube. The instruction can be found [here](https://minikube.sigs.k8s.io/docs/start/). I have already push the docker to [Docker Hub](https://hub.docker.com/repository/docker/chloechen79/imdb).

### Run

To run locally, please run `cargo run` in the root directory.  

To run on Kubernetes, please follow the steps below:

1. `minikube start`
2. Create a deployment: `kubectl create deployment imdb-api --image=registry.hub.docker.com/chloechen79/imdb`
3. View deployment: `kubectl get deployments`
6. Create service and expose it: `kubectl expose deployment imdb-api --type=LoadBalancer --port=8080`
7. View services:  `kubectl get service imdb-api`
8.  `minikube service imdb-api --url`
9. Curl web service: i.e. `curl http://192.168.49.2:30082`
10. Cleanup
```bash
kubectl delete service imdb-api
kubectl delete deployment imdb-api
minikube stop
````

### Route

1. `/`: The home page of the website.
2. `/search/{name}: Use the name to search the movie. **You can find redirect buttons on this page!**
3. `/title/{id}`: Use the id to get the information of the movie.
4. `/reviews/{id}`: Use the id to get the reviews of the movie.

### Example

`/search/you` will get the search result of the movie "You". It will show the top 5 results.



`/title/tt7335184` will get the information of the TV show "You", including the title, rate and cast.

## Reference

1. [IMDB](https://www.imdb.com/)
2. [Kubernetes](https://kubernetes.io/docs/tutorials/hello-minikube/)
3. [MiniKube]((https://minikube.sigs.k8s.io/docs/start/))
4. [Applied Kubernetes by Noah Gift on Github](https://github.com/nogibjj/coursera-applied-de-kubernetes-lab)
5. [Rust Template by Noah Gift on Github](https://github.com/noahgift/rust-new-project-template)
