mod year2013;

use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) -> RustGymResult {
    let mut it = reader.lines().map(|l| l.unwrap());
    let line = it.next().unwrap();
    write!(writer, "{}", line)?;
    Ok(())
}

fn main() -> RustGymResult {
    let mut res = "".to_string();
    solve(&mut BufReader::new(stdin()), &mut res)?;
    print!("{}", res);
    Ok(())
}
