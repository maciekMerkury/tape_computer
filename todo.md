# ASM Parser
    [ ] think about the actual structure and features of the asm and the assembler before doing anything
    [ ] parser structure:
        [ ] parse line and generate tokens with little validation
        [ ] analyse tokens and validate them
        [ ] turn tokens into vector of instructions
    [ ] ASM features:
        [ ] pure instruction parsing
        [*] comments
        [ ] jump labels
            [ ] for functions and stuff
        [ ] variables
            [ ] fat pointer-style location
        [ ] meaningful error messages

