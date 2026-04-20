deploy:
	bash -c 'set -a; source ./.env; set +a; envsubst < ./k8s/deployment.yml | kubectl apply -f -'