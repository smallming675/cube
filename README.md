## A simple solver for the Rubik's Cube
This program provides a UI for entering Rubik's Cube states and provides a list of the moves used to solve the cube. For reference, it takes ~300ms per solve with the average solution being ~23 moves long. 

### Usage 
```
Usage: solver <COMMAND>

Commands:
  init-cache    Initialize data needed for the solver
  gen-scramble  Generate a scramble of a given length
  solve         Solve a cube given a scramble
  benchmark     Benchmarks the solver by solving a given amount of cubes
  ui            Provides a GUI for the user to input the cube
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Process
A lot of the math and concepts behind this are based on [Cube Explorer](https://kociemba.org/cube.htm), To simplify, The program searches iteratively starting at the scramble, finding a solution that matches G1 = <U,D,R2,L2,F2,B2>, where all of the corners and edges are orientated, and the equator edges are in the equator. During this process, we use lookup table to estimate the lower bound of the current node, and pruning off bad branches as needo. Once we found a solution to G1, we perform another search, now with a restricted move set, we then use another lookup table for estimation until we solve the cube.

### Lookup Tables
Lookup tables are used heavily in this program as they massively increase the speed of evaluating and producing positions. We have multiple lookup tables which can be seperated into two categories: move tables and coordinate tables. Move tables provide a lower bound for solving a particular subset of the cube (like solving all the corners) while Coordinate tables provide the transformation from one coordinate (which are natural numbers which are compressed to described the cube) to another given a single move. 



