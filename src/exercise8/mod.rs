use std::fs;

mod decode;
mod encode;

pub fn run() {
    let lines = fs::read_to_string("resources/2015/ex8_in")
        .expect("Couldn't read input");

    println!("Decoding...");
    print_diff(decode::compute_lengths(&lines));

    println!("\nEncoding...");
    print_diff(encode::compute_lengths(&lines));
}

fn print_diff((code_len, mem_len): (usize, usize)) {
    eprintln!("code_len = {:?}", code_len);
    eprintln!("mem_len = {:?}", mem_len);
    eprintln!("diff = {:?}", code_len - mem_len);
}