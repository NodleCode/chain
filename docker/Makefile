SERVICE_NAME   := nodle-chain
VERSION        := $(shell git rev-parse --short HEAD)
PROJECT_ID     := $(shell gcloud config get-value core/project)
CONTAINER_NAME := gcr.io/${PROJECT_ID}/${SERVICE_NAME}:${VERSION}
WORKERS_FILE   := workers.json

docker-build:
	cd ../ && docker build -t ${CONTAINER_NAME} -f docker/Dockerfile . && cd docker

docker-push: docker-build
	docker push ${CONTAINER_NAME}
	
setup:
	kubectl create secret generic validator-creds --from-env-file=./creds.env
	
deploy: docker-push
	cat kube/deployment.yml | sed 's#SERVICE_NAME#${SERVICE_NAME}#g' | sed 's#DOCKER_IMAGE#${CONTAINER_NAME}#g' | kubectl apply -f -
	cat kube/service.yml | sed 's#SERVICE_NAME#${SERVICE_NAME}#g' | kubectl apply -f -

clean:
	cat kube/service.yml | sed 's#SERVICE_NAME#${SERVICE_NAME}#g' | kubectl delete -f -
	cat kube/deployment.yml | sed 's#PROJECT_ID#${PROJECT_ID}#g' | sed 's#SERVICE_NAME#${SERVICE_NAME}#g' | sed 's#DOCKER_IMAGE#${CONTAINER_NAME}#g' | kubectl delete -f -
	kubectl delete secret validator-creds