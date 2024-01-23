# protein-folder-rs
It is a port of old algorithm used [here](https://github.com/Czandal/ProteinFolder/).

This repo is aiming to be a benchmark/test of results presented in this [article](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5172541/).
That is - using branch and bound to get approximate energy of simplistic of model of protein, which energy is solely based on existence of two Hydrophobic monomers next to each other.

## Algorithm
Takes a string, which consists of letter 'P' (for polar) and 'H' (for Hydrophobic) and returns score of best computed conformation (score is negative energy, so a positive number) and number of branches used to get the result.
Algorithm *is* case sensitive and naive. It assumes that the input string is correct, looks for 'H' and treats any other character in the string as 'P'.

## Benchmarks
Here there will be different benchmarks of different versions of the algorithm

### Bruteforce (bounding not implemented):
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

### Bounding implemented (p1 set to 0.8 and p2 to 0.99)
|  Input  |  Number of branches | Score | Time elapsed (ns) | Timeout exceeded | Ran |
|---------|---------------------|-------|-------------------|------------------|-----|
| HHHH | 4 | 1 | 17391 | false | true |
| HPHPPHHPHPPHPHHPPHPH | 18 | 9 | 364645 | false | true |
| HHPPHPPHPPHPPHPPHPPHPPHH | 941 | 8 | 13387463 | false | true |
| PPHPPHHPPPPHHPPPPHHPPPPHH | 3215 | 7 | 40413682 | false | true |
| PPPHHPPHHPPPPPHHHHHHHPPHHPPPPHHPPHPP | 277 | 13 | 6935525 | false | true |
| PPHPPHHPPHHPPPPPHHHHHHHHHHPPPPPPHHPPHHPPHPPHHHHH | 24 | 17 | 8269019 | false | true |
| PPHPPHPHPHHHHPHPPPHPPPHPPPPHPPPHPPPHPHHHHPHPHPHPHH | 4343 | 17 | 40205298250 | true | true |
| PPHHHPHHHHHHHHPPPHHHHHHHHHHPHPPPHHHHHHHHHHHHPPPPHHHHHHPHHPHP | N/A | N/A | N/A | N/A | false |
| HHHHHHHHHHHHPHPHPPHHPPHHPPHPPHHPPHHPPHPPHHPPHHPPHPHPHHHHHHHHHHHH | N/A | N/A | N/A | N/A | false |
| HHHHPPPPHHHHHHHHHHHHPPPPPPHHHHHHHHHHHHPPPHHHHHHHHHHHHPPPHHHHHHHHHHHHPPPHPPHHPPHHPPHPH | N/A | N/A | N/A | N/A | false |
| PPPHHPPHHHHPPHHHPHHPHHPHHHHPPPPPPPPHHHHHHPPHHHHHHPPPPPPPPPHPHHPHHHHHHHHHHHPPHHHPHHPHPPHPHHHPPPPPPHHH | N/A | N/A | N/A | N/A | false |

### Optimized `find` algorithm (p1,p2)=(0.8,0.99)
|  Input  |  Number of branches | Score | Time elapsed (ns) | Timeout exceeded | Ran |
|---------|---------------------|-------|-------------------|------------------|-----|
| HHHH | 4 | 1 | 10616 | false | true |
| HPHPPHHPHPPHPHHPPHPH | 89 | 8 | 721678 | false | true |
| HHPPHPPHPPHPPHPPHPPHPPHH | 233 | 8 | 7845637 | false | true |
| PPHPPHHPPPPHHPPPPHHPPPPHH | 323 | 7 | 27071965 | false | true |
| PPPHHPPHHPPPPPHHHHHHHPPHHPPPPHHPPHPP | 1121 | 13 | 9956194 | false | true |
| PPHPPHHPPHHPPPPPHHHHHHHHHHPPPPPPHHPPHHPPHPPHHHHH | 7 | 19 | 9732559 | false | true |
| PPHPPHPHPHHHHPHPPPHPPPHPPPPHPPPHPPPHPHHHHPHPHPHPHH | 288 | 16 | 1736984195 | false | true |
| PPHHHPHHHHHHHHPPPHHHHHHHHHHPHPPPHHHHHHHHHHHHPPPPHHHHHHPHHPHP | 27 | 31 | 1869462 | false | true |
| HHHHHHHHHHHHPHPHPPHHPPHHPPHPPHHPPHHPPHPPHHPPHHPPHPHPHHHHHHHHHHHH | 0 | 26 | 2099871 | false | true |
| HHHHPPPPHHHHHHHHHHHHPPPPPPHHHHHHHHHHHHPPPHHHHHHHHHHHHPPPHHHHHHHHHHHHPPPHPPHHPPHHPPHPH | 31 | 50 | 19605152 | false | true |
| PPPHHPPHHHHPPHHHPHHPHHPHHHHPPPPPPPPHHHHHHPPHHHHHHPPPPPPPPPHPHHPHHHHHHHHHHHPPHHHPHHPHPPHPHHHPPPPPPHHH | 128 | 44 | 190303729 | false | true |

Each version of code was tested using following procedure:
```
cargo build --release
./target/release --bench
```
