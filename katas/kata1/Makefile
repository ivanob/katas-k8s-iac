include .env

push-service-a:
	docker build -t $(ECR_REGISTRY)/service-a:latest ./services/service-a
	docker push $(ECR_REGISTRY)/service-a:latest

push-service-b:
	docker build -t $(ECR_REGISTRY)/service-b:latest ./services/service-b
	docker push $(ECR_REGISTRY)/service-b:latest

push-all: push-service-a push-service-b

deploy:
	kubectl apply -f ./k8s/deployment.yml
