SHELL := /bin/bash

build-site:
	echo "Building Website..."
	cd ./site && npm run build

package-api:
	echo "Zipping Api..."
	cd ./blog_api && make build-api

deploy-cdk: build-site package-api
	cdk deploy