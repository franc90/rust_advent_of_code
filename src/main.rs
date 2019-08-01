use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

mod exercise5;
mod exercise6;


fn main() -> io::Result<()> {
    ex1();
    ex2()?;
    ex3();
//    ex4(); time consuming
    exercise5::ex5();
    exercuse6::ex6();
    Ok(())
}


fn ex4() {
    let secret_key = String::from("bgvyzdsv");
    let mut found = false;
    let mut nr: i64 = 0;
    while !found && nr >= 0 {
        let nr_as_str = nr.to_string();
        let mut data = secret_key.clone();
        data.push_str(nr_as_str.as_str());
        let digest = md5::compute(data);
        let digest = format!("{:x}", digest);
        if digest.starts_with("000000") {
            found = true;
        } else {
            nr += 1;
        }
    }
    eprintln!("smallest nr = {:?}", nr);
}

fn ex3() {
    #[derive(Eq, PartialEq, Hash)]
    struct House(i32, i32);
    struct Santa(i32, i32);

    let mut santa = Santa(0, 0);
    let mut robot_santa = Santa(0, 0);
    let mut visited_houses = HashSet::new();
    let mut santa_turn = true;

    visited_houses.insert(House(0, 0));

    let f = File::open("resources/2015/ex3_in").expect("file open failed");
    let f = BufReader::new(f);

    for line in f.lines() {
        for c in line.expect("lines failed").chars() {
            let mut current_santa = if santa_turn {
                &mut santa
            } else {
                &mut robot_santa
            };
            visit_next_house(&c, &mut current_santa, &mut visited_houses);
            santa_turn = !santa_turn;
        };
    };

    fn visit_next_house(c: &char, santa: &mut Santa, visited_houses: &mut HashSet<House>) {
        match c {
            '>' => santa.0 += 1,
            '<' => santa.0 -= 1,
            'v' => santa.1 -= 1,
            '^' => santa.1 += 1,
            _ => eprintln!("unknown char = {:?}", c)
        };
        visited_houses.insert(House(santa.0, santa.1));
    }

    eprintln!("Number of visited houses = {:?}", visited_houses.len());
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

fn ex1() {
    let mut f = File::open("resources/2015/ex1_in").expect("couldn't open file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("couldn't read file");
    let mut floor_nr = 0;
    for x in buffer {
        match x {
            40 => floor_nr += 1,
            41 => floor_nr -= 1,
            _ => println!("unknown char, ignoring")
        }
    }
    eprintln!("floor_nr = {:?}", floor_nr);
}
