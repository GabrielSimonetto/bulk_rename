# bulk_rename

This is something under construction, but it is being built to quickly deal with files that have a similar structure and should generalize from similar contexts to specific contents.

Although the name is bulk_rename, I've just implemented the copy method for now :yes


# Basic Usage:

```bash
$ ls
input_abacaxi.txt  output_abacaxi.txt  intermediary_abacaxi.txt

$ cargo run ./ "*_abacaxi.txt" "*_bolovo.txt"

$ ls
input_abacaxi.txt  output_abacaxi.txt  intermediary_abacaxi.txt  
input_bolovo.txt  output_bolovo.txt  intermediary_bolovo.txt
```

And voilá! Now you have input, intermediary, and output for `bolovo`, that you copied from `abacaxi`.

For development I am just using script_test_1.sh while I think how the hell am I gonna test this.


# TODO
- [ ] - Allow patterns with different positionings (e.g.: `*_abacaxi.txt` -> `cenouras_*.txt`)
- [ ] - Make `rename` command
- [ ] - Return decent errors
- [ ] - Make globbing from CWD work. (e.g.: This works: `cargo run ./ "*_abacaxi.txt" "*_bolovo.txt"`, but removing quotes makes glob expand and fuck up the command)
- [ ] - Remove clones
- [ ] - Generic types on functions
- [ ] - Show what's gonna happen and ask if it's ok.
- [ ] - Unsuck the suck
