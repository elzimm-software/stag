# [S]tate Diagrams from [T]ikz Defined in [A]SCII and [G]enerated by LaTeX

STAG is a DSL designed to quickly define state transition diagrams as plaintext which can be compiled to 
tikz latex standalones and then rendered with any desired latex compiler.
The initial intent for this DSL was to be integrated into org-mode via org-babel to enable inline state diagrams.

## Grammar

```
program -> (TOP_MATTER "\n")? alphabet "\n" nodes EOF ;
alphabet -> "{" (LETTER ",")* "}";
nodes -> node*;
node -> variable flags name? transitions "\n"?;
variable -> "(" VAR ")";
flags -> "" | "<" | "*" | "<*" | "*<";
name -> "[" NAME "]";
transitions -> "{" transition* "}";
transition -> VAR ",";
```

### Top Matter

Any additional top matter, such as extra `\usepackage` declarations, may be included prior to the alphabet declaration.



### Comments

While lines do not need to end in a semicolon, if a semicolon is present the rest of the line will be ignored
and can be used for comments.  (In future I'd like to implement a 'doc comment' functionality,
where text after a double semicolon (`;;`) will be copied over into the compiled .tex output.)

## Source Code

STAG is built with passing the source code as a string at the command line in mind, but a `.stag` file may also be used.
The `-i` flag may also be specified to force the compiler to treat `<SOURCE>` as a string.  (This is useful if a state name contains `.stag`.)