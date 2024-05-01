use std::usize;

#[derive(Debug)]
struct Analyzer {
    content: String,
    position: i32,
}

impl Analyzer {
    fn new(enter: String) -> Self {
        Self {
            content: enter,
            position: 0,
        }
    }

    fn next(&mut self) -> (bool, usize, String) {
        let number = String::new();

        loop {
            match self.content.chars().nth(self.position) {
                None => return (true, self.position, String::new()),
                Some(c) => {
                     self.position += 1;
                     
                     match c {
                          '-' | '+' => return (true, self.position, c.toString()),
                          '0..=9' => {
                            number.push(c);
                            let mut count = 1;
                            loop {
                                match self.content.chars().nth(self.position + count) {
                                    None => {
                                       todo!()
                                    },
                                    Some(c) => {
                                       todo!()
                                    }
                                }
                            }
                        }
                     }
                }
            }
        }
    }

    fn back(&mut self, pos: usize, s: String) {
        todo!()
    }
}

fn main() {
    let exp = Analyzer::new(String::from("5+3"));
    println!("{exp:?}");
}
