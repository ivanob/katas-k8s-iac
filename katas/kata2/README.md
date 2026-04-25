## Authentication with keycloak

- This kata will integrate NGINX + Keycloak for authentication/authorization of users
- All the deployment would be automated: creation of realms, creation of Client... try to avoid manual config in keycloak
- It will contain ROLES: Admin, Customer, Operator
- It will allow social login: Facebook, Google
- It will be locally deployed in K8s, but lets deploy too to the VPS with K3s
- It will connect to a database to store users
- Add some observability tool to manage:
    - Content of DB
    - Whatever happened through NGINX
    - Whatever happened in Keycloak
- Maybe add a couple of resources in a microservice just to validate JWTs


## Explanation of make check-minikube-status

`kubectl get all -n kata2` groups output by resource type.

### Pods

```
pod/keycloak-74667d8f68-wm7gn   1/1 Running
pod/keycloak-db-0               1/1 Running
pod/nginx-5b84b5ddbd-prktb      1/1 Running
```

A pod is the actual running container workload.

What this means:

- `keycloak` is running
- `keycloak-db` is running
- `nginx` is running
1/1 means each pod has one container, and that container is ready.

RESTARTS tells you whether the container inside that pod has crashed and been restarted.

- `keycloak-db` shows `1 (3d20h ago)`, which means it restarted once a long time ago, not recently.
- the others have `0`, so no recent crash/restart.
The pod names have random suffixes because they are created by higher-level controllers.

### Services

```
service/keycloak-db-service   ClusterIP   ...   5432/TCP
service/keycloak-service      ClusterIP   ...   8080/TCP
service/nginx-service         NodePort    ...   80:32085/TCP
```

A service is a stable network endpoint in front of pods.

What this means:

- `keycloak-db-service` exposes Postgres internally on port `5432`
- `keycloak-service` exposes Keycloak internally on port `8080`
- `nginx-service` exposes NGINX on port `80`, and because it is `NodePort`, Kubernetes also opened host/node port `32085`

`ClusterIP` means internal-only inside the cluster.
`NodePort` means externally reachable through the node or through Minikube’s forwarding.

For `nginx-service`, `80:32085/TCP` means:

- service port inside Kubernetes: `80`
- node port exposed by Kubernetes: `32085`

### Deployments
```
deployment.apps/keycloak   1/1 1 1
deployment.apps/nginx      1/1 1 1
```

A deployment manages stateless pods and rolling updates.

The columns mean:

- `READY`: how many desired pods are ready
- `UP-TO-DATE`: how many pods match the latest deployment spec
- `AVAILABLE`: how many are available to serve traffic
So both deployments are healthy:

one desired pod
one current pod
one available pod

### ReplicaSets
```
replicaset.apps/keycloak-59d9bb8dd    0 0 0
replicaset.apps/keycloak-6dfd56d979   0 0 0
replicaset.apps/keycloak-74667d8f68   1 1 1
replicaset.apps/nginx-5b84b5ddbd      1 1 1
replicaset.apps/nginx-69856b4f86      0 0 0
```

A ReplicaSet is what a Deployment creates under the hood to manage a specific version of the pod template.

This is rollout history.

For Keycloak:

- `keycloak-74667d8f68` is the current active version
- the older ones are still listed, but scaled down to `0`

For NGINX:

- `nginx-5b84b5ddbd` is the current active version
- `nginx-69856b4f86` is the previous version, now inactive

So when you changed config and restarted/redeployed, Kubernetes created a new ReplicaSet and moved traffic to it.

### StatefulSet
```
statefulset.apps/keycloak-db   1/1
```

A StatefulSet is like a Deployment, but for stateful apps that need stable identity and storage.

That is why Postgres is managed differently from NGINX and Keycloak.

This means:

- desired DB pod count is `1`
- one DB pod is ready

That pod is named `keycloak-db-0`, which is stable and predictable. StatefulSets do that on purpose.

### Overall interpretation

Your namespace currently has:

- 3 running pods
- 3 services
- 2 deployments for stateless apps
- 1 statefulset for Postgres
- several old ReplicaSets kept as rollout history

So the stack is healthy at the Kubernetes object level.

A concise mental model:

- `Pod:` the actual running thing
- `Service:` stable network name in front of pods
- `Deployment:` manages rolling updates for stateless apps
- `ReplicaSet:` versioned pod group created by a deployment
- `StatefulSet:` manages stateful pods like databases

The most important lines for you operationally are:

- pod status: are the apps actually running?
- service type/ports: how can they talk to each other or be reached?
- deployment/statefulset ready counts: is Kubernetes happy with the desired state?


## Important links
- https://www.kubermatic.com/blog/keeping-the-state-of-apps-4-persistentvolumes-and-persistentvolum/