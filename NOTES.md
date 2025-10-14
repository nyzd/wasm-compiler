# Error Handling

We have a seperate lib for Error handling, Which all the crates and libs use it.

# Lexer

### Developing Lexer

Our lexer recives str slice and returns List of tokens!
**Used Peekable Chars struct so we can peek and move around chars of the input slice freely**
**Token** Enum has more helpfull impls that is making the lexer clean code (Lexer needs to do less work than before)

# Parser
