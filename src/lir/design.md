# LIR design file

## The Stack Architecture

- Three "temp" cells empty for copying
  - The temporary cells must be at 0 after every instruction!
- Variables
  - All variables and their sizes are known at compile time
- Stack
  - Essentially everything after variables
  - Program needs to manage it itself


Using a variable automatically defines it.


