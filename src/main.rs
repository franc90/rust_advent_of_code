use std::cmp::{max, min};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

fn main() -> io::Result<()> {
    ex1()?;
    ex2()?;
    Ok(())
}

fn ex2() -> io::Result<()> {
    fn get_val(vec: &Vec<&str>, idx: usize) -> u64 {
        match vec.get(idx) {
            Some(txt) => txt.parse::<u64>().unwrap(),
            None => 0
        }
    }

    let f = File::open("resources/2015/ex2_in")?;
    let f = BufReader::new(f);

    let mut total_area: u64 = 0;
    let mut ribbon_len: u64 = 0;
    for line in f.lines() {
        if let Ok(txt) = line {
            let v: Vec<&str> = txt.split("x").collect();
            assert_eq!(v.len(), 3);
            let l = get_val(&v, 0);
            let w = get_val(&v, 1);
            let h = get_val(&v, 2);
            let lw = l * w;
            let lh = l * h;
            let wh = w * h;
            let min_lw = min(l, w);
            let min_lh = min(l, h);
            let min_wh = min(w, h);
            let smallest = min(min_lw, min(min_lh, min_wh));
            let second_smallest = max(min_lw, max(min_wh, min_lh));

            total_area += 2 * lw + 2 * lh + 2 * wh + min(lw, min(lh, wh));
            ribbon_len += l * w * h + 2 * smallest + 2 * second_smallest;
        }
    }

    eprintln!("total_area = {:?}", total_area);
    eprintln!("ribbon_len = {:?}", ribbon_len);
    Ok(())
}

fn ex1() -> io::Result<()> {
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
