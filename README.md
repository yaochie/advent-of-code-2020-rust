# Advent of Code 2020 solutions in Rust

Link to challenges: [Advent of Code](https://adventofcode.com/2020)

These solutions were done while trying to learn Rust at the same time,
so the code probably isn't the best.

Some especially janky solutions:

- Day 17: Used tuples to represent coordinates, so when 3D changed to 4D,
I just changed all the 3D coordinates to 4D. This means the solution only
works for the second part.

- Day 20: Very tedious. Fitting the tiles together was done using
pattern-matching on which sides matched and in which direction,
so a lot of cases were written.

- Day 21: After getting the list of ingredients that could contain
the allergens, the final answer was obtained by hand since there
was only 8 allergens (in my input).
