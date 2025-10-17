# Error Handling

We have a seperate lib for Error handling, Which all the crates and libs use it.
Using some traits and sturcts we have a working error module :), We can have multiple types of errors
and for each part of compiler, Also more importantly we can collect errors not just crash after one error
**We should crash after part of the compiler is having errors for example if the lexer has errors we should crash and print the errors so user can fix them, not just show them and move to parser part (That would be stupid)**

# Lexer

### Developing Lexer

Our lexer recives str slice and returns List of tokens!
**Used Peekable Chars struct so we can peek and move around chars of the input slice freely**
**Token** Enum has more helpfull impls that is making the lexer clean code (Lexer needs to do less work than before)

# Parser
