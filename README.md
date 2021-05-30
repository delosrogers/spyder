# Spyder
A basic stack based interpreted language/bytecode format.

## Syntax
### Available instructions are:
note, parameters enclosed in `[]` are optional while those in `<>` are
mandatory.
- `var <name> = <value>` initialize or update a variable stored in the global
  store. Value must be an immediate value and not another variable.
- `push <val>`
- `load [variable]` load value from the store onto the top of the stack using
  the index on the top of the stack or if variable name is present load it to
  the top of the stack
- `store [variable]` pops off destination address then the value to store or if
  variable name is present store top of stack to that variable
- `pop` pops off top of stack
- `goto <label>` unconditionally jumps to `label`
- `gotoEqual <label>` pops top of stack and checks if it is zero if it is it
  jumps to `label`
- `rePush` copy top of stack and push it again
- `noOp`
- `return`
- `call <label>` jump to label and push the address of the next statement onto
  the callees stack. Also clears the callers stack.
- Arithmetic: all instructions pop the top two elements of the stack then
  perform an operation then push it back onto the stack.
  - `add`
  - `sub`
  - `mul`
  - `div`

### Label syntax:
```
!![<label>] <instruction>
```
