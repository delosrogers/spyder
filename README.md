# Spyder
A basic stack based interpreted language/bytecode format.

## Syntax
### Available instructions are:
- `Push <val>`
- `Load` load value from the store onto the top of the stack using the index on
  the top of the stack
- `Store` pops off destination address then the value to store
- `Pop` pops off top of stack
- `Goto <label>` unconditionally jumps to `label`
- `GotoIfEqual <label>` pops top of stack and checks if it is zero if it is it
  jumps to `label`
- `RePush` copy top of stack and push it again
- `NoOp`
- Arithmetic: all instructions pop the top two elements of the stack then
  perform an operation then push it back onto the stack.
  - `Add`
  - `Sub`
  - `Mul`
  - `Div`

### Label syntax:
```
!![<label>] <instruction>
```
note, you cannot put a label on a `Goto` or `GotoIfEqual` if you would like to
to this just precede it with a `NoOp` and jump there.