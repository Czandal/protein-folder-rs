# protein-folder-rs
It is a port of old algorithm used [here](https://github.com/Czandal/ProteinFolder/).

This repo is aiming to be a benchmark/test of results presented in this [article](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5172541/).
That is - using branch and bound to get approximate energy of simplistic of model of protein, which energy is solely based on existence of two Hydrophobic monomers next to each other.

## Algorithm
Takes a string, which consists of letter 'P' (for polar) and 'H' (for Hydrophobic) and returns score of best computed conformation (score is negative energy, so a positive number) and number of branches used to get the result.
Algorithm *is* case sensitive and naive. It assumes that the input string is correct, looks for 'H' and treats any other character in the string as 'P'.

## Benchmarks
Here there will be different benchmarks of different versions of the algorithm

### Bruteforce (p1 and p2 equal to 1.0 or not-implemented):
|  Input  |  Number of branches | Score | Time elapsed (ns) | Timeout exceeded | Ran |
|---------|---------------------|-------|-------------------|------------------|-----|
| HHHH | 4 | 1 | 5240 | false | true |
| HPHPPHHPHPPHPHHPPHPH | 240805 | 9 | 229571529 | false | true |
| HHPPHPPHPPHPPHPPHPPHPPHH | 11546787 | 9 | 11908906042 | true | true |
| PPHPPHHPPPPHHPPPPHHPPPPHH | N/A | N/A | N/A | N/A | false |
| PPPHHPPHHPPPPPHHHHHHHPPHHPPPPHHPPHPP | N/A | N/A | N/A | N/A | false |
| PPHPPHHPPHHPPPPPHHHHHHHHHHPPPPPPHHPPHHPPHPPHHHHH | N/A | N/A | N/A | N/A | false |
| PPHPPHPHPHHHHPHPPPHPPPHPPPPHPPPHPPPHPHHHHPHPHPHPHH | N/A | N/A | N/A | N/A | false |
| PPHHHPHHHHHHHHPPPHHHHHHHHHHPHPPPHHHHHHHHHHHHPPPPHHHHHHPHHPHP | N/A | N/A | N/A | N/A | false |
| HHHHHHHHHHHHPHPHPPHHPPHHPPHPPHHPPHHPPHPPHHPPHHPPHPHPHHHHHHHHHHHH | N/A | N/A | N/A | N/A | false |
| HHHHPPPPHHHHHHHHHHHHPPPPPPHHHHHHHHHHHHPPPHHHHHHHHHHHHPPPHHHHHHHHHHHHPPPHPPHHPPHHPPHPH | N/A | N/A | N/A | N/A | false |
| PPPHHPPHHHHPPHHHPHHPHHPHHHHPPPPPPPPHHHHHHPPHHHHHHPPPPPPPPPHPHHPHHHHHHHHHHHPPHHHPHHPHPPHPHHHPPPPPPHHH | N/A | N/A | N/A | N/A | false |

Each version of code was tested using following procedure:
```
cargo build --release
./target/release --bench
```
