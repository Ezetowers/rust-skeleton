SHELL := /bin/bash
PWD := $(shell pwd)

GIT_REMOTE = github.com/Ezetowers/rust-skeleton
PROJECT_NAME = minigrep
default: build

all:

build:
	cargo build --release
.PHONY: build

build-docker-base-images:
	docker build -f ./dockerfiles/deploy-base.Dockerfile -t deploy-base:latest .
.PHONY: build-docker-base-images

docker-image:
	docker build --build-arg PROJECT_NAME=${PROJECT_NAME} -f Dockerfile -t "${PROJECT_NAME}:latest" .
.PHONY: docker-image

docker-compose-up: docker-image
	docker-compose -f docker-compose-dev.yaml up -d --build
.PHONY: docker-compose-up

docker-compose-down:
	docker-compose -f docker-compose-dev.yaml stop -t 1
	docker-compose -f docker-compose-dev.yaml down
.PHONY: docker-compose-down

docker-compose-logs:
	docker-compose -f docker-compose-dev.yaml logs -f
.PHONY: docker-compose-logs
