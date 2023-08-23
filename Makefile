
# select app in apps/ directory
APP_NAME_DEFAULT := itworks
IMAGE_NAME := rustywasm

ifneq ($(app),)
APP_NAME = $(app)
else
APP_NAME = $(APP_NAME_DEFAULT)
endif

APP_DIR := $(PWD)/apps/$(APP_NAME)

ifeq ($(user),)
# USER retrieved from env, UID from shell.
HOST_USER ?= $(strip $(if $(USER),$(USER),root))
HOST_UID ?= $(strip $(if $(shell id -u),$(shell id -u),0))
HOST_GID ?= $(strip $(if $(shell id -u),$(shell id -g),0))
else
# allow override by adding user= and/ or uid=  (lowercase!).
# uid= defaults to 0 if user= set (i.e. root).
HOST_USER = $(user)
HOST_UID = $(strip $(if $(uid),$(uid),0))
endif

export HOST_USER
export HOST_UID
export HOST_GID

build:
	docker build \
		--build-arg HOST_USER=$(HOST_USER) \
		--build-arg HOST_UID=$(HOST_UID) \
		--build-arg HOST_GID=$(HOST_GID) \
		-t $(IMAGE_NAME) .

run: build
	@[ -d $(APP_DIR) ] || (echo "ERROR: $(APP_DIR) not found" && exit 1)
	@[ -d ./html ] || mkdir -p ./html
	docker run -it --rm \
		--name $(IMAGE_NAME) \
		-e APP_NAME=$(APP_NAME) \
		-e UID=$(HOST_UID) \
		-e GID=$(HOST_GID) \
		-e USER=$(HOST_USER) \
		-p 8080:8080 \
		-v ./html:/html \
		-v $(APP_DIR):/app $(IMAGE_NAME)

