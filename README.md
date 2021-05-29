# Spyder
A basic stack based interpreted language/bytecode format.

## Syntax
### Available instructions are:
- `push <val>`
- `load` load value from the store onto the top of the stack using the index on
  the top of the stack
- `store` pops off destination address then the value to store
- `pop` pops off top of stack
- `goto <label>` unconditionally jumps to `label`
- `gotoEqual <label>` pops top of stack and checks if it is zero if it is it
  jumps to `label`
- `rePush` copy top of stack and push it again
- `noOp`
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
note, you cannot put a label on a `goto` or `gotoEqual` if you would like to
to this just precede it with a `noOp` and jump there.