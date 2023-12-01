# Advent Of Code 2023

My solutions for [Advent Of Code](https://adventofcode.com) 2023.

## Usage

This project uses [`just`](https://github.com/casey/just) . For some commands to work  a session token needs to be provided in the `AOC_SESSION` environment variable. The easiest way to set it is to create the file `.env` with `AOC_SESSION=your token` inside in the root of this repository. The token can be received by reading the session cookie from the AOC website.

Download the puzzle input, create a file for the current day and open it in RustRover:

```shell
just begin        # Prepare the current day
just day=09 begin # Prepare day 9
```

Run:

```shell
just        # Runs the current day
just day=09 # Runs day 9
```
