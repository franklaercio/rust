use std::{io, usize};
use std::io::{stdout, Write};

#[derive(Debug)]
struct Analyzer {
    content: String,
    position: usize,
}

// Implement methods for the Analyzer struct
impl Analyzer {
    // The new method takes a String as input and returns a new instance of the Analyzer struct
    fn new(enter: String) -> Self {
        Self {
            content: enter,
            position: 0,
        }
    }

    // The next method analyzes the content and returns a tuple containing:
    // - a boolean indicating whether the analysis was successful
    // - the current position in the content String
    // - the analyzed part of the content as a String
    fn next(&mut self) -> (bool, usize, String) {
        let mut number = String::new();

        loop {
            match self.content.chars().nth(self.position) {
                None => return (true, self.position, String::new()),
                Some(c) => {
                    self.position += 1;

                    match c {
                        '-' | '+' => return (true, self.position, c.to_string()),
                        '0'..='9' => {
                            number.push(c);
                            loop {
                                match self.content.chars().nth(self.position) {
                                    None => {
                                        return (
                                            true,
                                            self.position,
                                            number,
                                        );
                                    }
                                    Some(c) => {
                                        match c {
                                            '-' | '+' => {
                                                return (
                                                    true,
                                                    self.position,
                                                    number,
                                                );
                                            }
                                            '0'..='9' => {
                                                self.position += 1;
                                                number.push(c);
                                                continue;
                                            }
                                            ' ' => {
                                                return (
                                                    true,
                                                    self.position,
                                                    number,
                                                );
                                            }
                                            _ => {
                                                return (
                                                    true,
                                                    self.position,
                                                    number,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        ' ' => return (true, self.position, c.to_string()),
                        _ => return (false, self.position, c.to_string()),
                    }
                }
            }
        }
    }

    // The back method is not implemented yet
    fn back(&mut self, pos: usize, s: String) {
        todo!()
    }
}

// The main function is the entry point of the program
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

// The analyzer function takes a &str as input and returns a vector of tuples,
// where each tuple contains a String and a usize.
// The function analyzes the input and returns the results of the analysis.
fn analyzer(input: &str) -> Vec<(String, usize)> {
    let mut exp = Analyzer::new(String::from(input));
    let mut results = vec![];

    loop {
        let actual_pos = exp.position;
        let (is_exp, pos, s) = exp.next();

        if s == " " {
            continue;
        }

        if is_exp {
            results.push((s, actual_pos));
        } else {
            results.push(("Error in position".to_string(), actual_pos));
        }

        if exp.content.len() == pos || !is_exp {
            break;
        }
    }

    results
}

#[cfg(test)]
mod analyzer {
    use crate::analyzer;

    #[test]
    fn test_expression() {
        assert_eq!(analyzer("450 + 20"), vec![("450".to_string(), 0), ("+".to_string(), 4), ("20".to_string(), 6)]);
    }

    #[test]
    fn test_expression_with_spaces() {
        assert_eq!(analyzer("450     +     20"), vec![("450".to_string(), 0), ("+".to_string(), 8), ("20".to_string(), 14)]);
    }

    #[test]
    fn test_expression_with_no_spaces() {
        assert_eq!(analyzer("450+20"), vec![("450".to_string(), 0), ("+".to_string(), 3), ("20".to_string(), 4)]);
    }

    #[test]
    fn test_expression_with_zeros() {
        assert_eq!(analyzer("0+-0"), vec![("0".to_string(), 0), ("+".to_string(), 1), ("-".to_string(), 2), ("0".to_string(), 3)]);
    }

    #[test]
    fn test_expression_with_multiple_operators() {
        assert_eq!(analyzer("0 +++"), vec![("0".to_string(), 0), ("+".to_string(), 2), ("+".to_string(), 3), ("+".to_string(), 4)]);
    }

    #[test]
    fn test_expression_with_error() {
        assert_eq!(analyzer("10+a"), vec![("10".to_string(), 0), ("+".to_string(), 2), ("Error in position".to_string(), 3)]);
    }

    #[test]
    fn test_expression_with_error_at_end() {
        assert_eq!(analyzer("10 + 20a"), vec![("10".to_string(), 0), ("+".to_string(), 3), ("20".to_string(), 5), ("Error in position".to_string(), 7)]);
    }
}