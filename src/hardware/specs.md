# Specs
## Terminology
TP -> tape pointer, the current cell adress pointed by the pointer
PC -> program counter, the current cell adress pointed by the program counter

## Instructions: 
### Arithmetic Instructions
Increment -> increments the value pointed to by TP
Decrement -> decreaments the value pointed to by TP
Add(u32) -> adds the values pointed to by TP and u32, stores the result in TP

### Tape Pointer Instructions
MoveTapePointer(u32) -> sets TP to u32
ShiftTPForwards(u32) -> adds u32 to the TP
ShiftTPBackwards(u32) -> substracts u32 from the TP

### Program Counter Instructions
MovePC(u32) -> sets PC to u32
MovePCIfZero(u32) -> sets PC to u32 if value pointed to by TP is zero

### Return Instructions
Return(u8) -> return the u8
ReturnCell -> returns the value pointed to by TP

### Cell Instructions
SetCellValue(u8) -> sets the value pointed to by TP to u8
CopyCellValue(u32) -> copies the value pointed to by TP to u32

### Binary Operator Instructions
Negate -> negates the value pointed to by TP, stores the results in TP
Or(u32) -> ors the values pointed to by TP and u32, stores the results in TP
And(u32) -> ands the values pointed to by TP and u32, stores the result in TP

