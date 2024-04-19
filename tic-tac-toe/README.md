# Tic-tac-toe

This is a simple implementation of the game tic-tac-toe in Rust. 

1. The game is played in the terminal.
2. The first move is for player X. 
3. Show the board. 
4. Read in which position the player wants to play.
   1. Positions are numbered from 1 to 9.
   2. Ignore invalid positions. 
5. Update the board. 
6. If the player wins, show the final board and announce the end of the game. 
7. Otherwise, switch to the next player. 
8. Repeat from step 2.

## How to play

To play the game, you need to have Rust installed on your machine. If you don't have Rust installed, you can install it by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

To play the game, clone the repository and navigate to the project directory in your terminal. Then run the following command:

```bash
cargo run
```

This will start the game. You will be prompted to enter the position the cell where you want to place your mark. The positions are from 1 to 9, so they go from 0 to 2. For example, to place your mark in the top-left cell, you would enter `1`.

__Positions__
```terminal
 1 | 2 | 3  
---+---+---
 4 | 5 | 6
---+---+---
 7 | 8 | 9  
```

__Example board when you select 1__
```terminal
 X |   |   
---+---+---
   |   |
---+---+---
   |   |   
```

The game will continue until there is a winner or a draw. If you want to play again, you can run the game again with the `cargo run` command.

## Contributing

If you find any bugs or have suggestions for improvements, feel free to open an issue or create a pull request.

## About me

I'm Frank and I'm a software developer. You can find me on [GitHub](https://github.com/franklaercio) and [LinkedIn](https://www.linkedin.com/in/frank-laercio/).