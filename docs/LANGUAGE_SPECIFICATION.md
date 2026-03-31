# LEXOR Language Specification

LEXOR is an AST-interpreted programming language combining strong typing with rigid block enclosures.

## Program Structure
All functional LEXOR logic must be entirely contained within the root script block.
```text
SCRIPT AREA
START SCRIPT
    %% Executable code goes here
END SCRIPT
```

## Data Types & Variables
- `INT`: Whole numbers (e.g., `5`, `-10`).
- `FLOAT`: Decimal numerals (e.g., `3.14`).
- `CHAR`: Distinct isolated characters enclosed by single quotes (e.g., `'A'`).
- `BOOL`: Natively strictly `TRUE` or `FALSE`.

Variables explicitly require the `DECLARE` keyword for initialization.
```text
DECLARE INT limit = 10
DECLARE FLOAT baseline = 98.6

%% Subsequent re-assignment does not require DECLARE
limit = limit + 5
```

## Console Input/Output
I/O commands require a trailing colon.
```text
PRINT: "Result: " & limit & $
SCAN: limit
```
- `&` serves as the universal concatenation operator chaining text and numbers.
- `$` specifically represents a standalone string newline character.

## Control Flow
### IF / ELSE Block
Conditions map directly to standard expressions. All branches require their own `START` / `END` blocks.
```text
IF (x > 5)
START IF
    PRINT: "Greater" & $
END IF
ELSE
START IF
    PRINT: "Lesser or equal" & $
END IF
```

### REPEAT WHEN Loop
Iterates continuously as long as the inner condition equates to `TRUE`.
```text
REPEAT WHEN (count < 10)
START REPEAT
    count = count + 1
END REPEAT
```

### FOR Loop
C-style loop iteration requiring three comma-separated boundaries initialization, condition, and variable update sequentially.
```text
FOR (i = 1, i <= 10, i = i + 1)
START FOR
    PRINT: "Iteration: " & i & $
END FOR
```

## Code Comments
Comments are established with double-percentage symbols and are universally ignored by the internal lexer.
```text
%% This entire line is completely ignored.
```
