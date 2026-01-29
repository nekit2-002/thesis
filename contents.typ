#include "chapters/abstract.typ"
#import "@preview/i-figured:0.2.4"
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

// setup figure enumeration
#set figure(supplement: none)
#show figure.caption: it => {
  let custom-prefix = [Рисунок]
  [
    #custom-prefix
    #counter(heading.where(level: 1)).display()#it.counter.display(it.numbering)
    --
    #it.body
  ]
}

#set ref(supplement: it => {
  if it.func() == figure {
    "рис. " + context {counter(heading.where(level: 1)).at(it.label).first()}
    "."
  } else if it.func() == heading {
    "разд. "
  } else if it.func() == table {
    "табл. "
  }else {
    ""
  }
})

// ? main content
#include "chapters/introduction.typ"
#include "chapters/chapter1.typ"
#counter(figure.where(kind: image)).update(0)
#include "chapters/chapter2.typ"
#counter(figure.where(kind: image)).update(0)
#include "chapters/chapter3.typ"
#counter(figure.where(kind: image)).update(0)
#include "chapters/chapter4.typ"
#counter(figure.where(kind: image)).update(0)
#include "chapters/conclusion.typ"

// ? insert bibliography
#bibliography("chapters/biblio.bib", title: [Список литературы], style: "chapters/gost-r-7-0-5-2008-numeric.csl", full: false)

#pagebreak()

// ? start of appendix section
#counter(heading).update(0)
#set heading(numbering: "A.1.", supplement: [Приложение])
#include "appendices/appendix1.typ"
