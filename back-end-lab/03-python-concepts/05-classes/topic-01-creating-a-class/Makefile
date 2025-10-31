.SILENT:
.PHONY: run clean setup

run:
	python main.py

clean:
	rm -rf .venv/
	rm -rf .ruff_cache/

setup: clean
	uv sync && direnv allow
