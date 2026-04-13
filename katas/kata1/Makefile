include .env

push-service-a:
	docker build -t $(ECR_REGISTRY)/service-a:latest ./services/service-a
	docker push $(ECR_REGISTRY)/service-a:latest

push-service-b:
	docker build -t $(ECR_REGISTRY)/service-b:latest ./services/service-b
	docker push $(ECR_REGISTRY)/service-b:latest

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