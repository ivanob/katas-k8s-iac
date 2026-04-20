include .env

push-service-a:
	docker build -t service-a:latest ./services/service-a

push-service-b:
	docker build -t service-b:latest ./services/service-b

push-all: push-service-a push-service-b

deploy:
	bash -c 'set -a; source ./.env; set +a; envsubst < ./k8s/deployment.yml | kubectl apply -f -'

check-k8s-context:
	kubectl config current-context

check-k8s-status:
	kubectl get all -n kata1

aws-login:
	aws ecr get-login-password --region eu-west-1 | docker login --username AWS --password-stdin $(ECR_REGISTRY)

delete-k8s-services:
	kubectl delete pod -n kata1 -l app=service-b && kubectl delete pod -n kata1 -l app=service-a

expose-service-a:
	kubectl port-forward -n kata1 svc/service-a 8080:80

start-minikube-tunnel: #creates a network bridge between your local machine and the minikube cluster.
	minikube tunnel

get-minikube-ip:
	kubectl get ingress -n kata1

nginx-logs:
	kubectl logs -n ingress-nginx -l app.kubernetes.io/name=ingress-nginx -f

nginx-status:
	kubectl get pods -n ingress-nginx && echo "---" && kubectl get ingress -n kata1

ingress-info:
	kubectl describe ingress -n kata1 kata1-ingress

destroy-k8s-cluster:
	kubectl delete namespace kata1 && minikube delete