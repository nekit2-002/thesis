all: Косенко_НИ_ВКР.pdf

Косенко_НИ_ВКР.pdf: main.pdf title/internal.pdf
	pdftk title/title_external.pdf title/internal.pdf main.pdf cat output Косенко_НИ_ВКР.pdf

main.pdf: main.typ
	typst compile main.typ main.pdf

# title.pdf: title/title.typ
# 	typst compile --root . --pages 1 title/title.typ title.pdf

.PHONY: clean
clean:
	rm -f *.pdf