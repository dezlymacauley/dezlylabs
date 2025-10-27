.SILENT:
.PHONY: run clean setup

run:
	python main.py

clean:
	rm -rf .venv/

setup: clean
	uv sync && direnv allow
