# simple data generator 
## For testing the "simple data parser"

Generates n lines with the following data:
* index 
* name, composed of a single capital letter
* previous number, randomly chosen between lower and upper bounds given as arguments
* current number, randomly chosen between lower and upper bounds given as arguments

## Usage: 

_datagen nlines lowerbound upperbound outputfilename_

e.g. _datagen 1000 0 5 output.txt_

## TO DO:

* add a possibility to use different separators

## Improvements:
Use of format! macro instead of a less clear to understand string construction.
Some improvements suggested by Clippy: unwrap_or_else, some useless imports removed, static lifetime removed from constants.


## Time measured with Linux time command:

1_000_000_000 lines: 

* Rust: 18 minutes
* Python: 92 minutes

