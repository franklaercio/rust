use std::io::{stdin, stdout, Read, Result, Write};

// eco(r: dyn Read), but doesn't compile because it's a indirect call 
// eco(r: impl Read) use when you need to one direct call

fn eco(r: &mut dyn Read) -> Result<()> {
    for b in r.bytes() {
        stdout().write(&[b?])?;
    }
    Ok(())
}

fn main() -> Result<()> {
    eco(&mut stdin())
}
