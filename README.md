# [S]imple [T]ext [A]utomata [G]enerator

STAG is a DSL designed to quickly define state transition diagrams as plain text and render them as images.
The initial intent for this DSL was to be integrated into org-mode via org-babel to enable inline state diagrams.

## Language Features

### Alphabet

The set of transitions, or formally the alphabet, is defined in the form of a comma separated list surrounded by curly braces.
The order of this set defines the order of the connections in the state declarations.

### State

States are declared in the following parts:
- An identifier surrounded by parentheses.
- Flags may be specified: `*` for accepting states and `<` for the start state.
- A display name surrounded by brackets; accepts basic formatting including single and double asterisks, `_`, and `^` for italics, bold, subscript, and superscript respectively.
- A list of other identifiers (which do not need to have been previously declared) separated by spaces and positionally bound to the alphabet set to define outward transitions.
State definitions may be in any order.

### Comments

While lines do not need to end in a semicolon, if a semicolon is present the rest of the line will be ignored
and can be used for comments.

## Command Line Arguments

### Output File

STAG will output a png image with the name of the input file or `stag_out.png` for string inputs.
The `-o` flag may be specified to define a custom output file; changing the file extension will have no bearing on the format.

### Implied Naming

STAG's default behavior is such that states without an explicit name (within `[]`) are rendered without any text.
The `-f` flag may be specified to force the compiler to use the identifier (within `()`) as the rendered name for states without an explicit name.

### Source Code

STAG is built with passing the source code as a string at the command line in mind, but a `.stag` file may also be used.
The `-i` flag may be specified to force the compiler to treat `<SOURCE>` as a string.  (This is useful if a state name contains `.stag`.)

## Context Free Grammar

```
program -> alphabet nl states EOF ;
alphabet -> "{" (TRANSITION ("," TRANSITION)*)? "}" ;
states -> state (nl state)* ;
state -> identifier flags name? transitions ;
identifier -> "(" IDENT ")" ;
flags -> ("<" | "*" | "<*" | "*<")? ;
name -> "[" NAME "]" ;
transitions -> (IDENT (" " IDENT)*)? ;
nl -> (";" | NL | ";" NL) ;
```
