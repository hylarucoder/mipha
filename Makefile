.PHONY:  help
.DEFAULT_GOAL := help

define PRINT_HELP_PYSCRIPT
import re, sys

for line in sys.stdin:
	match = re.match(r'^([a-zA-Z_-]+):.*?## (.*)$$', line)
	if match:
		target, help = match.groups()
		print("%-30s %s" % (target, help))
endef
export PRINT_HELP_PYSCRIPT

help:
	@python -c "$$PRINT_HELP_PYSCRIPT" < $(MAKEFILE_LIST)

dev: ## build local
	maturin develop

build-local: ## build local
	maturin build && pip install -e .

build: ## build rust and python mixed
	maturin build

build-prod: ## build rust and python mixed
	maturin build --release

test: ## test
	pytest

publish: ## publish package to pypi
	poetry version
	poetry publish --build