# polya

A little program in rust to brute-force disprove the [Pólya conjecture](https://en.wikipedia.org/wiki/P%C3%B3lya_conjecture)

## Running

```
cargo test     # runs tests
cargo run 450  # tests all numbers up through 450
```

You can disprove the conjecture by running

```
cargo run 1000000000 # one billion
```

On a 2021 M1 Macbook Pro, it took me 2h9m3s to output 906,150,257;
the smallest number that disproves the conjecture.
