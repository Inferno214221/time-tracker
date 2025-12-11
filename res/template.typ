#let invoice = sys.inputs;

#let constants = (
  name: "...",
  abn: "...",
  bsb: "...",
  acc: "..."
);

#import "@preview/nth:1.0.1": nth

#set page(width: 10cm, height: auto, margin: 1cm)

#set text(font: "Ubuntu Mono")

#align(center)[
  = Tax Invoice
]

== Invoice Details

#set align(right)

Reference Number: #h(1fr) #invoice.num\
Date:             #h(1fr) #nth(invoice.created.display("[day]"))
                          of #invoice.created.display("[month repr:long] [year]")

*From:*           #h(1fr) #constants.name\
ABN:              #h(1fr) #constants.abn

*To:*             #h(1fr) #invoice.recipient.name\
Address:          #h(1fr) #invoice.recipient.addr\

#set align(left)

== Provided Services

#let hrs(dur) = [
  #dur #if dur == 1 {"hr"} else {"hrs"}
];

#let total-dur = 0.0;

#let total-price = 0.0;

#table(
  align: (x, y) => if y <= 1 {
    center
  } else if calc.even(y) {
    left
  } else {
    right
  },
  stroke: 0.5pt,
  columns: (1fr, 1fr, 1fr),
  table.header(table.cell(colspan: 3)[*Description*]),
  table.header([*Unit Price*], [*Duration*], [*Price*]),
  ..(for activity in invoice.activities {
    let price = if activity.dur == 0 {
      activity.uprice
    } else {
      calc.round(activity.uprice * activity.dur, digits: 2)
    };
    total-dur += activity.dur;
    total-price += price;

    let tickets = if activity.tickets.len() == 0 {""} else {
      " (" + activity.tickets.join(", ") + ")"
    };

    (
      table.cell(colspan: 3)[#activity.desc#tickets],
      ..if activity.dur == 0 {(
        table.cell(colspan: 3)[\$#activity.uprice],
      )} else {(
        [\$#activity.uprice],
        hrs(activity.dur),
        [\$#price]
      )}
    )
  }).flatten(),
  [*TOTAL*], align(right, hrs(total-dur)), align(right)[\$#total-price]
)

#align(center)[_No GST has been charged._]

== Bank Account Details
Name:           #h(1fr) #constants.name\
BSB:            #h(1fr) #constants.bsb\
Account Number: #h(1fr) #constants.acc