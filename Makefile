all: main.pdf

main.pdf: main.typ
	typst compile main.typ main.pdf

.PHONY: clean
clean:
	rm -f *.pdf