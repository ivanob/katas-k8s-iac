
# Introduction

- 2 microservices (pods) deployed in EKS via k8s.
- pod service-a will expose an API to the exterior and send info the the second pod (service-b).
- pod service-a will have redundancy up to 2 instances, so it can handle more traffic than standalone.
    - Exposes a simple API with a POST and a GET endpoint.
    - Can a small swagger be exposed in a GET endpoint?
- A load balancer will receive requests from outside internet.
- Communicates with a second microservice (service-b) and that has a database persistance
    - Has a database attached that persists and stores number of requests received
- All deployed with the same deployment.yml using the K8s approach. 
- Nothing created with terraform, AWS is just the cloud to deploy. No usage of AWS services besides EKS.
- I will add traceID with any known framework. In both microservices. The metrics service will store requests per session and in global. Session is, lets say, received in the last 5 min.
- Docker images are stored in ECR in AWS, so I give permissions to pull them:

```
kubectl create secret docker-registry ecr-secret \
  --docker-server=REPLACE_BY_IMAGE_URL \
  --docker-username=AWS \
  --docker-password=$(aws ecr get-login-password --region eu-west-1) \
  -n kata1
```

# Config
Add a .env file with this info:
```
ECR_REGISTRY="path-to-your-ecr-registry"
```

Before pulling or pushing images to ECR, if you are using AWS then you need to login with your credentials. Check that you have your `access_key_id` and `secret_access_key` in ~/.aws/credentials and if you do a `echo $AWS_PROFILE` it displays your user. Then do in command line:
```
aws ecr get-login-password --region eu-west-1 | docker login --username AWS --password-stdin HERE_THE_ECR_URL

ECR_URL=XXXXXXXX.dkr.ecr.eu-west-1.amazonaws.com
```

# Steps

I will use kubectl as tool to connect to the k8s cluster.
To execute the following commands I use the tool Makefile

1. Build the infra in AWS for the K8s cluster
    - `terraform apply` in the /infra folder
2. Build and push the docker images of the services
    - `make push-all`
3. Deploy the K8s namespace
    - `make deploy`

# Instructions

This kata will use the tool Makefile to build and push the docker images to the repository (ECR). Also to deploy the workload.

# Ahas!

- 1 Pod can contain 1 or more containers (a collection of them). 1 Pod != 1 Container.
    - we define them in the deployment.yml
    - The general agreement is that 1 pod means 1 microservice
- The whole group of pods that define the app is called namespace.
- The `scaling_config` in the eks.tf terraform indicates how many EC2 workers are available. 1 Pod != 1 EC2
- K8s manages the distribution of Pods into EC2s. i.e. If only 1 EC2 is available and we have 3 Pods, it will put all of them inside the same EC2.
    - If we dont provide enough EC2s for the Pods requirements, then pods will fail to start and stay with "Pending" state

# replicas vs scaling_config:
- Replicas scales application instances. Scaling_config scales infraestructure.
- If I set `replica: 2` there will be 2 identical pods with the same microservice running. If one dies it will automatically spin up again
- Scaling_config: This is a cap more than a requirement. WARNING: We can define a waste of resources. i.e.
    - There is only one pod declared in our deployment with `replicas: 1` and I set the scaling_config like this:
        desired_size = 2  ← You're PAYING for 2 nodes always
        max_size = 3      ← Can scale UP to 3 if needed (not waste, just capacity)

...So replicas in deployment.yml and scaling_config in eks.tf are not necesarly related. They will determine the number of pods running but:
    - replicas is a strong requirement of our app.
    - scaling_config defines limits and controls the physical deployment. It determines how many EC2s are available as workers for our system. We could deploy the app in another cluster with different limits.
