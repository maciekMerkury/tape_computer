# Specs

## General Syntax
Structure:
``` 
inst arg arg arg 
```

## Comments:
No support for after-line comments, i.e. the comment must span the entire line
Start with ; and go for the rest of the line
Can be precedeed by whitespace

## Labels:
Marked with .NAME
Example: 
```
.TEST
```

## Constants
The whitespace betwenn '#' and "ptr"/"cell" is optional
```
#ptr l 2137
# cell c 255
add l
ret c
```
