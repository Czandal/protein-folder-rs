# protein-folder-rs
It is a port of old algorithm used [here](https://github.com/Czandal/ProteinFolder/).

This repo is aiming to be a benchmark/test of results presented in this [article](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5172541/).
That is - using branch and bound to get approximate energy of simplistic of model of protein, which energy is solely based on existence of two Hydrophobic monomers next to each other.

## Algorithm
Takes a string, which consists of letter 'P' (for polar) and 'H' (for Hydrophobic) and returns score of best computed conformation (score is negative energy, so a positive number) and number of branches used to get the result.
Algorithm *is* case sensitive and naive. It assumes that the input string is correct, looks for 'H' and treats any other character in the string as 'P'.

## Benchmarks
Here there will be different benchmarks of different versions of the algorithm

Each version of code was tested using following procedure:
```
cargo build --release
./target/release --bench
```
