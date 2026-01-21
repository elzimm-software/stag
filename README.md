# [S]tate Diagrams from [T]ikz Defined in [A]SCII and [G]enerated by LaTeX

STAG is a DSL designed to quickly define state transition diagrams as plaintext which can be compiled to 
tikz latex standalones and then rendered with any desired latex compiler.
The initial intent for this DSL was to be integrated into org-mode via org-babel to enable inline state diagrams.

## Grammar

### States

A state is defined in the form `(<var>)[[name]]` where `var` is the identifier used to reference the state when
defining connections and `name` is the displayed text inside the state e.g. `(q1)[S_1]`.
The name block is optional and when omitted `var` will be used as the displayed text.
To define a node with no text simply leave the `[]` empty.
A state must be defined before it is referenced by an edge.
`var` may include alphanumeric characters, '-', and '_' and must start with a letter.
`name` is copied into the compiled latex verbatim and may include any valid LaTeX.


#### Initial

A state can be identified as the initial state by adding a `<` immediately following the parenthesis e.g. `()<` or `()<[]`
Only one initial state may be declared.

#### Accepting
A state can be identified as accepting by adding a `*` immediately following the parenthesis e.g. `()*` or `()*[]`

#### Initial and Accepting

A state can be defined as both accepting and initial by including both `*` and `>` following the parenthesis
e.g. `()*<[]` or `()<*[]`.

### Transitions

Transitions are defined in the form `<var start> --(<positional arguments>)[[edge text]]> <var end>`
where `var start` is the state the transition starts from,
`var end` is the state the transition goes to,
`edge text` is the text displayed on the transition,
and `positional arguments` define the displayed position of the transition (Eventually I'd like this to be optional.)
e.g. `q1--(above)[1]>q1`.

## Source Code

STAG is built with passing the source code as a string at the command line, but a `.stag` file may also be used.
The `-i` flag may also be specified to force the compiler to treat `<SOURCE>` as a string.  (This is useful if a state name contains `.stag`.)