#set page(
  margin: (
    right: 10mm,
    bottom: 20mm,
    left: 30mm,
    top: 20mm,
  ),
)
#set text(font: "Times New Roman", lang: "ru", hyphenate: true, size: 13pt)
#set par(justify: true, first-line-indent: (amount: 1.25cm, all: true), spacing: 1em, leading: 1em)


// title
// #include "title/title.typ"
// #image("title/title.pdf", format: "pdf")
// task
// #include "task/task.typ"

#set page(numbering: "1")
#counter(page).update(2)
#set heading(numbering: "1.1.")

// ! Эта срань специально для росла чтобы не доебался до точек в заголовках первого уровня
// #set heading(numbering: (..args) => {
//   let nums = args.pos()
//   if nums.len() == 1 {
//     // Level 1: Number only, no dot
//     numbering("1", ..nums)
//   } else {
//     // Level 2+: Standard nested numbering (e.g., 1.1.)
//     numbering("1.1.", ..nums)
//   }
// })
#show heading: set align(left)
#show heading.where(level: 1): set align(center)
#set enum(indent: 7mm)
#set list(indent: 7mm, marker: [--])
#include "contents.typ"

