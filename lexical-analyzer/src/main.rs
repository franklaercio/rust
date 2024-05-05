use std::{io, usize};
use std::io::{stdout, Write};

#[derive(Debug)]
struct Analyzer {
    content: String,
    current_pos: usize,
}

// Implement methods for the Analyzer struct
impl Analyzer {
    // The new method takes a String as input and returns a new instance of the Analyzer struct
    fn new(enter: String) -> Self {
        Self {
            content: enter,
            current_pos: 0,
        }
    }

    // The next method analyzes the content and returns a tuple containing:
    // - a boolean indicating whether the analysis was successful
    // - the current position in the content String
    // - the analyzed part of the content as a String
    fn next(&mut self) -> (bool, usize, String) {
        let mut number = String::new();

        loop {
            match self.content.chars().nth(self.current_pos) {
                None => return (true, self.current_pos, String::new()),
                Some(c) => {
                    let start_pos = self.current_pos;
                    self.current_pos += 1;

                    match c {
                        '-' | '+' => return (true, start_pos, c.to_string()),
                        '0'..='9' => {
                            number.push(c);
                            while let Some(next_c) = self.content.chars().nth(self.current_pos) {
                                if next_c.is_digit(10) {
                                    self.current_pos += 1;
                                    number.push(next_c);
                                } else {
                                    break;
                                }
                            }
                            return (true, start_pos, number);
                        }
                        c if c.is_whitespace() => continue,
                        _ => return (false, start_pos, c.to_string()),
                    }
                }
            }
        }
    }
}

// The main function is the entry point of the program
fn main() {
    let mut input = String::new();
    print!("Enter your math expression: ");
    stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let tokens = analyzer(&input.trim());

    print!("Analyzer: ");
    for (token, pos) in tokens {
        print!("(\"{}\", {}) ", token, pos);
    }
}

// The analyzer function takes a &str as input and returns a vector of tuples,
// where each tuple contains a String and a usize.
// The function analyzes the input and returns the results of the analysis.
fn analyzer(input: &str) -> Vec<(String, usize)> {
    let mut analyzer = Analyzer::new(String::from(input));
    let mut tokens = vec![];

    loop {
        let (valid, pos, token) = analyzer.next();
        if valid && !token.is_empty() {
            tokens.push((token, pos));
        } else if !valid {
            tokens.push(("Error in position".to_string(), pos));
        } else {
            break
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use crate::analyzer;

    #[test]
    fn test_basic_expression() {
        assert_eq!(
            analyzer("450 + 20"),
            vec![
                ("450".to_string(), 0),
                ("+".to_string(), 4),
                ("20".to_string(), 6),
            ]
        );
    }

    #[test]
    fn test_expression_spaces() {
        assert_eq!(
            analyzer("450     +     20"),
            vec![
                ("450".to_string(), 0),
                ("+".to_string(), 8),
                ("20".to_string(), 14),
            ]
        );
    }

    #[test]
    fn test_expression_with_no_spaces() {
        assert_eq!(
            analyzer("450+20"),
            vec![
                ("450".to_string(), 0),
                ("+".to_string(), 3),
                ("20".to_string(), 4),
            ]
        );
    }

    #[test]
    fn test_expression_with_zeros() {
        assert_eq!(
            analyzer("0+-0"),
            vec![
                ("0".to_string(), 0),
                ("+".to_string(), 1),
                ("-".to_string(), 2),
                ("0".to_string(), 3),
            ]
        );
    }

    #[test]
    fn test_expression_with_multiple_operators() {
        assert_eq!(
            analyzer("0 +++"),
            vec![
                ("0".to_string(), 0),
                ("+".to_string(), 2),
                ("+".to_string(), 3),
                ("+".to_string(), 4),
            ]
        );
    }

    #[test]
    fn test_expression_with_after_binary_operator() {
        assert_eq!(
            analyzer("10+a"),
            vec![
                ("10".to_string(), 0),
                ("+".to_string(), 2),
                ("Error in position".to_string(), 3),
            ]
        );
    }

    #[test]
    fn test_expression_with_error_after_number() {
        assert_eq!(
            analyzer("10 + 20a"),
            vec![
                ("10".to_string(), 0),
                ("+".to_string(), 3),
                ("20".to_string(), 5),
                ("Error in position".to_string(), 7),
            ]
        );
    }

    #[test]
    fn test_expression_with_whitespaces() {
        assert_eq!(
            analyzer("10\n+\t20"),
            vec![
                ("10".to_string(), 0),
                ("+".to_string(), 3),
                ("20".to_string(), 5),
            ]
        );
    }
}

