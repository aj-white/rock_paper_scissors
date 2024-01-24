# Rock, Paper, Scissors

Yes, it's the classic rock, paper, scissors game you know and love in [rust](https://www.rust-lang.org/).


## Running

To get up and running you will need to have rust installed and either cloned or downloaded the repo.

To run:

```shell
$ cargo run
```

## Example

```shell
Enter choice [(r)ock, (p)aper, (S)cissors] or 'q' to quit: 
r
=========ROUND==========
Computer played: Rock
You played: Rock

🤷‍♂️ Draw 🤷‍♀️
========SCORES=========
Computer: 0
You: 0
Enter choice [(r)ock, (p)aper, (S)cissors] or 'q' to quit:
p
=========ROUND==========
Computer played: Scissors
You played: Paper

😭 Lose 😭
========SCORES=========
Computer: 1
You: 0
Enter choice [(r)ock, (p)aper, (S)cissors] or 'q' to quit:
r
=========ROUND==========
Computer played: Scissors
You played: Rock

✨ Win ✨
========SCORES=========
Computer: 1
You: 1
```

## Why ?

Currently learning rust, and as anyone knows to learn a language you need to build things and games are often a good place to start.

With this project I wanted to focus on a few things:

- breaking code out into modules (no long main.rs file please)
- use implemenations for enums and structs rather than separate functions