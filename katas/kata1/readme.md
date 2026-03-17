
# Introduction

- One service (pod) deployed in EKS via k8s.
- This pod will have redundancy up to 3 instances, so it can handle more traffic than standalone.
- Exposes a simple API with a POST and a GET endpoint.
- A load balancer will receive requests from outside internet.
    - Can a small swagger be exposed in a GET endpoint?
- Communicates with a second microservice (pod) and that has a database persistance
    - Has a database attached that persists and stores number of requests received
- All deployed with the same deployment.yml using the K8s approach. 
- Nothing created with terraform, AWS is just the cloud to deploy. No usage of AWS services besides EKS.

# Steps

I will use kuibectl as tool to connect to the k8s cluster.


# Gotchas!

- 1 Pod can contain 1 or more images. 1 Pod != 1 Image.
    - we define them in the deployment.yml
- The general agreement is that 1 pod means 1 microservice
- The scaling_config in the eks.tf terraform indicates how many EC2 workers are available. 1 Pod != 1 EC2
- K8s manages the distribution of Pods into EC2s. i.e. If only 1 EC2 is available and we have 3 Pods, it will put all of them inside the same EC2.
    - If we dont provide enough EC2s for the Pods requirements, then pods will fail to start and stay with "Pending" state
 