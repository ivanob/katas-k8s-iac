REGISTRY = 123456789.dkr.ecr.eu-west-1.amazonaws.com

push-service-a:
    docker build -t $(REGISTRY)/service-a:latest ./services/service-a
    docker push $(REGISTRY)/service-a:latest

push-service-b:
    docker build -t $(REGISTRY)/service-b:latest ./services/service-b
    docker push $(REGISTRY)/service-b:latest

push-all: push-service-a push-service-b

deploy:
    kubectl apply -f ./k8s/deployment.yml