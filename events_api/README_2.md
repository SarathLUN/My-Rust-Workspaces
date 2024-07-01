# How to build a REST Api with Rust, Diesel and Postgres, part 2: Kubernetes

## Introduction

In this previous [post](./README.md), we described how to build a simple Web API in Rust with Actix, Diesel, and Postgres. Now it is time to deploy this app.

### Prerequisites

To follow along, you will need the following:

- [Docker desktop](): we need this to build a Docker image
- A Docker hub account: because we will push our image to the hub
- If you want to experiment with Kubernetes locally, [Minikube]() is a good choice to install.
- The code from my previous post, which can be found [here](./README.md).

## Building the Dockerfile

If we want to deploy our app to Kubernetes, we need to `dockerize` it first, by using a dockerfile. If you have the solution from my previous post open in your editor or IDE, add a file named `Dockerfile` to the root of the project and make sure it reads like this:

```yaml
FROM rust:1.72-buster AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:bullseye as runner
RUN apt-get update \
     && apt-get -y install libpq-dev gcc
EXPOSE 8080
COPY --from=builder /app/target/release/events_api /app/target/release/events_api
COPY migrations /app/target/release/migrations
CMD ["/app/target/release/events_api"]
```

I will not go into details, suffice to say that we use a rust image to build our application and then use a tiny Debian (Linux) image to install that application on. Since we only have one binary, that is all we need.

Also, we need to copy the migrations directory to the target directory, so the app can do the migrations automatically.

Also in the root of your project, create a `.dockerignore` file which will prevent unneeded files from ending up in your docker image:

```dockerignore
**/target/
Dockerfile
.env
**/kubernetes/
**/.github/
**/.idea/
**/.vscode/
```

Now in your terminal type:

```shell
docker build -t <your dockerhub username>/eventsrust:latest .
```

Make sure you have Docker desktop or at least the Docker daemon running. This command will build the docker image. Do not worry if this takes several minutes.

Once this is a finished type:

```shell
docker login
```

That will log you in to the docker hub and then:

```shell
docker push <your dockerhub username>/eventsrust:latest
```

This will push your image to the Docker hub for later use.

## Deploying to Kubernetes

If your image has been successfully deployed to the Docker hub, we can start working on deploying it to Kubernetes. For the sake of simplicity, I assume you have Minikube or another local Kubernetes cluster running.

### Starting Minikube

If you want to start Minikube, in your terminal type:

```shell
minikube start
```

Another handy feature of Minikube is its dashboard. In a new terminal type:

```shell
minikube dashboard
```

We will use YAML-files to configure our Kubernetes cluster. Create a directory ‘kubernetes’ in the root directory of the project to put them in.

### The configmap

We will start with an easy one: the configmap. This resource stores key-value pairs which can be used as environment variables. In your kubernetes folder create a file called `db-configmap.yaml`:

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: db-secret-credentials
  labels:
    app: events_db
data:
  POSTGRES_DB: "events_db"
  POSTGRES_USER: "events_api"
  POSTGRES_PASSWORD: "events_api"
  HOST: "events_db.default"
  PORT: "5432"
  DATABASE_URL: "postgres://events_api:events_api@postgresdb.default:5432/events_db"
```

Without going in too much detail, you can see the key value pairs in the data section. In production, you probably use secrets, but I wanted to keep things simple for now.

Now change directory to your kubernetes directory in your terminal and type:

```shell
kubectl apply -f db-configmap.yaml
```

### The storage

The database will be needing some storage to store its data. For that, we need a persistent volume and a persistent volume claim.

We will start with the persistent volume file `db-persistent-volume.yaml`:

```yaml
apiVersion: v1
kind: PersistentVolume
metadata:
  name: events_db-pv
  labels:
    type: local
    app: events_db
spec:
  storageClassName: manual
  capacity:
    storage: 8Gi
  accessModes:
    - ReadWriteMany
  hostPath:
    path: "/data/db"
```

Again not going into much detail:

- The name is `events_db-pv` which will we need later
- We are using `hostPath` for simplicity, as it works on the node's filesystem
- The volume is 8Gb, which should be more than sufficient.

Now we also need a Persistent Volume Claim. Create a file called `db-persistent-volume-claim.yaml` in your kubernetes directory:

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: db-persistent-pvc
spec:
  storageClassName: manual
  accessModes:
    - ReadWriteMany
  resources:
    requests:
      # the PVC storage
      storage: 8Gi
```

Suffice to say this claims a persistent volume with specified specs, like being 8Gb or bigger and having storage class manual.

Now in your terminal type:

```shell
kubectl apply -f db-persistent-volume.yaml
kubectl apply -f db-persistent-volume-claim.yaml
```

And your volumes are deployed.

### The database deployment

We also need to deploy a pod with the database itself. In your kubernetes directory, create a file called `db-deployment.yaml`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: postgresdb
spec:
  replicas: 1
  selector:
    matchLabels:
      app: events_db
  template:
    metadata:
      labels:
        app: events_db
    spec:
      containers:
        - name: events_db
          image: postgres
          ports:
            - containerPort: 5432
          envFrom:
            - configMapRef:
                name: db-secret-credentials
          volumeMounts:
            - mountPath: /var/lib/postgres/data
              name: db-data
      volumes:
        - name: db-data
          persistentVolumeClaim:
            claimName: db-persistent-pvc
```

Here we deploy the postgres database itself. A few things to note:

- The environment variables (in the `envFrom` tag) are pulled from the configmap
- The volume is using the persistent volume claim
- And for the record: `/var/lib/postgres/data` is the standard location for the Postgres datafiles.

Save this file and in your terminal type:

```shell
kubectl apply -f db-deployment.yaml
```

### The database service

The web app needs to be able to access the database, in Kubernetes we use a service resource for that. In your kubernetes directory, create a file called `db-service.yaml`:

```yaml
apiVersion: v1
kind: Service
metadata:
  name: events_db
  labels:
    app: events_db
spec:
  ports:
    - port: 5432
  selector:
    app: events_db
```

There are two things to note here:

1. The selector, which is `app:events_db`. That is how this service is connected to the deployment
2. And no `type` like `LoadBalancer`. We want this database to not be directly reachable from the outside, hence no type. That way it is only reachable within the pod.

In your terminal type:

```shell
kubectl apply -f db-service.yaml
```

## The web app

Now we can safely deploy the `web-app`. In your kubernetes directory, create a file called `web-deployment.yaml`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: web-deployment
spec:
  replicas: 1
  selector:
    matchLabels:
      app: web
  template:
    metadata:
      labels:
        app: web
    spec:
      containers:
        - name: web
          image: <your docker username>/eventsrust:latest
          imagePullPolicy: IfNotPresent

          env:
            - name: "DATABASE_URL"
              valueFrom:
                configMapKeyRef:
                  key: DATABASE_URL
                  name: db-secret-credentials
```

Some notes:

- Note the image name in the containers, make sure that it is the same as the one you pushed to the Docker hub.
- We see here another way to get our environment variable, DATABASE_URL in this case from a ConfigMap. Again, in production you would probably use secrets.

In your terminal type:

```shell
kubectl apply -f web-deployment.yaml
```

Creating the web container can take some time, as Kubernetes will need to pull it from the hub.

If you are using the minikube dashboard, you should see the deployment in green, otherwise type:

```shell
kubectl get deployments
```

And you should see something like this:

```shell
NAME             READY   UP-TO-DATE   AVAILABLE   AGE
events_db        1/1     1            1           8h
web-deployment   1/1     1            1           6m31s
```

### The Web service

To make this API accessible to the outside world, we need a Kubernetes service. In your kubernetes directory, create a file called `web-service.yaml`:

```yaml
apiVersion: v1
kind: Service
metadata:
  name: web-service
spec:
  selector:
    app: web
  ports:
    - protocol: TCP
      port: 80
      targetPort: 8080
  type: LoadBalancer
```

Two notes here:

1. Note the selector: `app:web`. That is used to `connect` the service to the deployment
2. The type is `LoadBalancer` which makes it possible to reach this from the outside, unlike the database, for example.

In your terminal type:

```shell
kubectl apply -f web-service.yaml
```

## Testing it

There are two ways to test this, for both I would recommend using Postman

### Testing without minikube

In your terminal type:

```shell
kubectl get services
```

And you will get back something like this:

```shell
NAME          TYPE           CLUSTER-IP     EXTERNAL-IP   PORT(S)        AGE
kubernetes    ClusterIP      10.96.0.1      <none>        443/TCP        59d
events_db     ClusterIP      10.96.36.246   <none>        5432/TCP       8h
web-service   LoadBalancer   10.110.47.71   <pending>     80:32398/TCP   27m
```

Note the external IP address and save it somewhere.

### Testing with minikube

In a new terminal type:

```shell
minikube service web-service
```

This will produce something like:

```shell
|-----------|-------------|-------------|---------------------------|
| NAMESPACE |    NAME     | TARGET PORT |            URL            |
|-----------|-------------|-------------|---------------------------|
| default   | web-service |          80 | http://192.168.49.2:32398 |
|-----------|-------------|-------------|---------------------------|
🏃  Starting tunnel for service web-service.
|-----------|-------------|-------------|------------------------|
| NAMESPACE |    NAME     | TARGET PORT |          URL           |
|-----------|-------------|-------------|------------------------|
| default   | web-service |             | http://127.0.0.1:56190 |
|-----------|-------------|-------------|------------------------|
🎉  Opening service default/web-service in default browser...
❗  Because you are using a Docker driver on windows, the terminal needs to be open to run it.
```

Note the second URL, in this case the one with port number 56190.

### Putting it through the test in Postman

For the rest of this post, I assume you have Postman installed, although all of these tests can be done using curl.

Try using the url: `<url with port from minikube or url from external IP address with port>/api/events`. If things went right, you have an empty array `[]`

Now try the following:

![postman2.png](./postman2.png)

And press [Send], you should get your event back with an ID.

Now try the first URL and you should see an array with one element.

Congratulations! You have deployed a Rust web app, with Actix and Diesel, including automatic migrations to Kubernetes.
