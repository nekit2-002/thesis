#include "chapters/abstract.typ"

// ? main outline header settings: align to center and make it bigger
// make top level entries bold
#show outline: set text(size: 14pt)
#show outline.entry.where(level: 1): set text(weight: "bold")
// do not include appendices to main outline
#outline(target: selector(heading).before(<fst_appendix>, inclusive: false))

// ? outline for appendices
#set outline.entry(fill: none)
#outline(target: selector(heading).after(<fst_appendix>, inclusive: true), title: [Приложения])

#pagebreak()

// ? main content
#include "chapters/introduction.typ"
#include "chapters/chapter1.typ"
#include "chapters/chapter2.typ"
#include "chapters/chapter3.typ"
#include "chapters/chapter4.typ"
#include "chapters/conclusion.typ"

// ? insert bibliography
#bibliography("chapters/biblio.bib", title: [Список литературы], style: "chapters/gost-r-7-0-5-2008-numeric.csl", full: false)

#pagebreak()

// ? start of appendix section
#counter(heading).update(0)
#set heading(numbering: "A.1.", supplement: [Приложение])
#include "appendices/appendix1.typ"
