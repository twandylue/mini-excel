# mini-excel

mini-excel in Rust.

Implement a batch program that can accept a CSV file such as this.

```csv
A     , B
1     , 2
3     , 4
=A1+B1, =A2+B2
```

And outputs:

```csv
A , B
1 , 2
3 , 4
3 , 7
```

The calculation does not need any UI.


## Quick Start

```console
$ cargo run ./input.csv
...
```

## Reference

- [Mini Excel in C](https://www.youtube.com/watch?v=HCAgvKQDJng&t=47s&ab_channel=TsodingDaily)
