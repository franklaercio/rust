use std::io::{stdin, stdout, BufRead, BufReader, Result, Write};

fn read_play(r: &mut impl BufRead, w: &mut impl Write, t:usize) -> Result<usize> {
    let mut line = String::new();
    show_status(w, t, "It is your turn, select one place: ")?;

    loop {
        r.read_line(&mut line)?;
        if let Ok(index) = line.trim().parse::<usize>() {
            if index >= 1 && index <= 9 && (t >> (index - 1) * 2) & 3 == 0 {
                return Ok(index);
            }
        }
        write!(w, "Option invalid, try again: ")?;
        w.flush()?;
        line.clear();
    }
}

fn turn(r: &mut impl BufRead, w: &mut impl Write, t: &mut usize, x: usize) -> Result<bool> {
    *t |= x << (read_play(r, w, *t)? - 1) * 2;
    let cmp = *t >> (x - 1);
    for v in [0x15, 0x540, 0x15000, 0x1041, 0x4104, 0x10410, 0x10101, 0x1110] {
        if cmp & v == v {
            show_status(w, *t, "Congrats, you won!\n")?;
            return Ok(true);
        }
    }
    return Ok(false);
}

fn show_status(out: &mut impl Write, tab: usize, status: &str) -> Result<()> {
    for i in 0..9 {
        write!(out, " {} ", b" xo"[(tab >> i * 2) & 3] as char)?;
        if i % 3 != 2 {
            write!(out, "|")?;
        } else {
            writeln!(out, "{}", ["\n---+---+---", ""][i >> 3])?;
        }
    }
    write!(out, "{}", status)?;
    out.flush()
}

fn main() -> Result<()> {
    let mut breader = BufReader::new(stdin());
    let mut tab = 0;

    for i in [1, 2].iter().cycle().take(9) {
        if turn(&mut breader, &mut stdout(), &mut tab, *i)? {
            return Ok(());
        }
    }

    writeln!(stdout(), "Draw game!")
}
