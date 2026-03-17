push-service-a:
    docker build -t your-registry/service-a:latest ./services/service-a
    docker push your-registry/service-a:latest

push-service-b:
    docker build -t your-registry/service-b:latest ./services/service-b
    docker push your-registry/service-b:latest

push-all: push-service-a push-service-b

deploy:
    kubectl apply -f ./k8s/deployment.yml