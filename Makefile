ifneq (,$(wildcard ./.env))
    include .env
    export
endif

CMD ?= cargo
DOCKER ?= docker

.PHONY: install
# Install Rust using rustup.
install:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

.PHONY: run
# Run the application.
run:
	$(CMD) run

.PHONY: fmt
# Format the Rust files.
fmt:
	$(CMD) fmt

.PHONY: up
# Start the containers.
up:
	$(DOCKER) compose up -d

.PHONY: down
# Stop the containers.
down:
	$(DOCKER) compose down

.PHONY: enter
# Enter the database.
enter:
	$(DOCKER) exec -it axum-surreal /surreal sql -u $(DB_USER) -p $(DB_PASS)

help:
	@echo ''
	@echo 'Usage:'
	@echo ' make [target]'
	@echo ''
	@echo 'Targets:'
	@awk '/^[a-zA-Z\-\0-9]+:/ { \
	helpMessage = match(lastLine, /^# (.*)/); \
		if (helpMessage) { \
			helpCommand = substr($$1, 0, index($$1, ":")-1); \
			helpMessage = substr(lastLine, RSTART + 2, RLENGTH); \
			printf " - \033[36m%-20s\033[0m %s\n", helpCommand, helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST)

.DEFAULT_GOAL := help
