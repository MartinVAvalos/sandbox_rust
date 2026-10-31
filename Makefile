COMPOSE ?= docker compose
SERVICE ?= cargo

.PHONY: start restart stop logs check test fmt rs

start:
	$(COMPOSE) up -d --build

restart:
	$(COMPOSE) restart $(SERVICE)

stop:
	$(COMPOSE) down

logs:
	$(COMPOSE) logs -f $(SERVICE)

check:
	$(COMPOSE) run --rm --entrypoint sh $(SERVICE) -c "cargo check"

test:
	$(COMPOSE) run --rm --entrypoint sh $(SERVICE) -c "cargo test"

fmt:
	$(COMPOSE) run --rm --entrypoint sh $(SERVICE) -c "cargo fmt --all --check"

rs:
	@$(COMPOSE) run --rm --entrypoint sh $(SERVICE) -c '\
		file=$$(find src/leet -maxdepth 1 -type f -name "l_$(filter-out $@,$(MAKECMDGOALS))*" | head -1); \
		if [ -z "$$file" ]; then \
			echo "No file found starting with l_$(filter-out $@,$(MAKECMDGOALS))"; \
			exit 1; \
		fi; \
		echo "Running $$file"; \
		cargo eval "$$file"'

%:
	@: