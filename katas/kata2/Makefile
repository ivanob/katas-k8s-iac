deploy:
	bash -c 'set -a; source ./.env; set +a; kubectl apply -f ./k8s/'

init-minikube:
	minikube start
	minikube tunnel

check-minikube-status:
	kubectl get all -n kata2

get-nginx-address:
	minikube service nginx-service -n kata2 --url

restart-nginx:
	kubectl apply -f ./k8s/nginx-deployment.yml && kubectl rollout restart deployment/nginx -n kata2 && kubectl rollout status deployment/nginx -n kata2