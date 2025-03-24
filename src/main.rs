use address_book_rust::{Result, start};
use std::io;

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    start(&mut stdout, &stdin)?;

    Ok(())
}
