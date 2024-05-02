# Lexical Analyzer

This Analyzer is a Rust program that analyzes a given string and returns a vector of tuples. Each tuple contains a string and a usize. The string is a part of the input string and the usize is the position of that part in the input string.

## Structs

### Analyzer

The Analyzer struct has two fields:

- `content`: a String that holds the input to be analyzed.
- `position`: a usize that keeps track of the current position in the content String.

## Methods

### new

The `new` method takes a String as input and returns a new instance of the Analyzer struct.

### next

The `next` method analyzes the content and returns a tuple containing:

- a boolean indicating whether the analysis was successful
- the current position in the content String
- the analyzed part of the content as a String

### back

The `back` method is not implemented yet.

## Usage

The main function is the entry point of the program. It will prompt the user to enter a mathematical expression, then it will call the `analyzer` function with the given input string.

```rust
fn main() {
    let mut input = String::new();
    print!("Enter your math expression: ");
    stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let results = analyzer(&input.trim());

    print!("Analyzer: ");
    for (s, pos) in results {
        print!("(\"{}\", {}) ", s, pos);
    }
}
```

Example of execution:

```bash
Enter your math expression: 450 + 20
Analyzer: ("450", 0) ("+", 4) ("20", 6)
```

## Run

To run the program, execute the following command:

```bash
cargo run
```

## Test

To run the tests, execute the following command:

```bash
cargo test
```

## License

This project is licensed under the GNU License - see the [LICENSE](../LICENSE) file for details.