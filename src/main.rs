use std::io;
use std::fs::File;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut f = File::open("resources/2015/ex1_in")?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)?;

    let mut floor_nr = 0;
    for x in buffer {
        match x {
            40 => floor_nr += 1,
            41 => floor_nr -= 1,
            _ => println!("unknown char, ignoring")
        }
    }

    eprintln!("floor_nr = {:?}", floor_nr);
    Ok(())
}
