use std::io::{stdout, Write};

fn draw_board(out: &mut impl Write) {
    for i in 0..9 {
        write!(out, " X ").unwrap();
        if i % 3 != 2 {
            write!(out, "|").unwrap();
        } else if i != 8 {
            writeln!(out, "\n---+---+---").unwrap();
        }
    }

    out.flush().unwrap();
}

fn main() {
    //let mut player = BufReader::new(stdin());
    draw_board(&mut stdout());
}
