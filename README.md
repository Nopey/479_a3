# fourfight: connect-four game and agent
A connect four agent implemented using the minimax algorithm,
with a very simple heuristic for early termination.


## Compilation
To compile, run `make`.
To make a debug build, run `make fourfight_dbg`
`make clean` is also supported.

Code documentation can be generated, run `make doc` to generate the documentation from the doc comments.
Once generated, the documentation can be found in the `doc` folder, start at `doc/fourfight/index.html`.

No moving of source code is needed, the directory structure is flat
(with the exception of the doc folder for documentation, which is automatically created).


## Running

To run, simply enter `./fourfight`.


## User Interface
For each human player in the game, it will ask for a name.

It will repeatedly print the board state, as each move is made.

If a human player enters an invalid column, the human player will be considered as cheating.


## Heuristics
Heuristic is implemented in game.rs, as the estimate_ trio of functions.

This heuristic rewards "streaks" of same-colored discs, with a blank space on either end to grow into.


## Bugs
While there are no currently known bugs, this code is not to be trusted.

