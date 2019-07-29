use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn ex5() {
    let f = File::open("resources/2015/ex5_in").expect("file open failed");
    let f = BufReader::new(f);
    let mut simple_nice = 0;
    let mut complex_nice = 0;
    for lineResult in f.lines() {
        let line = lineResult.expect("lines failed");
        if is_nice(&line) {
            simple_nice += 1;
        }
        if is_complex_nice(&line) {
            complex_nice += 1;
        }
    }

    eprintln!("nr of nice strings = {:?}", simple_nice);
    eprintln!("nr of complex nice strings = {:?}", complex_nice);
}

fn is_nice(line: &String) -> bool {
    if line.len() < 3 {
        return false;
    }
    let mut vowels_count = 0;
    let mut doubles_count = 0;
    let mut no_forbidden_pairs = true;
    let mut chars_iter = line.chars();
    let mut last = chars_iter.next().unwrap();
    update_vowels(&mut vowels_count, &last);

    for curr in chars_iter {
        update_vowels(&mut vowels_count, &curr);
        update_doubles(&mut doubles_count, &curr, &last);
        update_forbiddens(&mut no_forbidden_pairs, &curr, &last);
        last = curr;
    };

    vowels_count > 2 && doubles_count > 0 && no_forbidden_pairs
}


fn update_vowels(current_count: &mut i32, c: &char) {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    if vowels.contains(c) {
        *current_count += 1
    }
}

fn update_doubles(current_count: &mut i32, curr: &char, last: &char) {
    if curr == last {
        *current_count += 1
    }
}

fn update_forbiddens(current_val: &mut bool, curr: &char, last: &char) {
    let forbidden_pairs = vec!["ab", "cd", "pq", "xy"];
    if forbidden_pairs.contains(&format!("{}{}", last, curr).as_str()) {
        *current_val = false;
    }
}

fn is_complex_nice(line: &String) -> bool {
    has_pair_appearing_twice(line) &&
        has_letter_appearing_twice(line)
}

fn has_pair_appearing_twice(line: &String) -> bool {
    let mut line = line.as_str();
    while line.len() > 3 {
        let pair = &line[..2];
        let mut slice = &line[2..];
        while slice.len() > 1 {
            if slice.contains(pair) {
                return true;
            }
            slice = &slice[1..];
        }
        line = &line[1..];
    }
    false
}

fn has_letter_appearing_twice(line: &String) -> bool {
    let mut slice: &str = line.as_str();
    while slice.len() > 2 {
        if is_expected_dupplicate(&slice[..3]) {
            return true;
        }
        slice = &slice[1..];
    }
    false
}

fn is_expected_dupplicate(slice: &str) -> bool {
    if slice.len() != 3 { return false; }
    let mut iter = slice.chars();
    let x = iter.next().unwrap();
    iter.next().unwrap();
    let z = iter.next().unwrap();
    x == z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_ugknbfddgicrmopn_nice() {
        assert_eq!(is_nice(&"ugknbfddgicrmopn".to_string()), true)
    }

    #[test]
    fn is_aaa_nice() {
        assert_eq!(is_nice(&"aaa".to_string()), true)
    }

    #[test]
    fn is_jchzalrnumimnmhp_nice() {
        assert_eq!(is_nice(&"jchzalrnumimnmhp".to_string()), false)
    }

    #[test]
    fn is_haegwjzuvuyypxyu_nice() {
        assert_eq!(is_nice(&"haegwjzuvuyypxyu".to_string()), false)
    }

    #[test]
    fn is_dvszwmarrgswjxmb_nice() {
        assert_eq!(is_nice(&"dvszwmarrgswjxmb".to_string()), false)
    }

    #[test]
    fn is_xyxy_complex_nice() {
        assert_eq!(is_complex_nice(&"xyxy".to_string()), true)
    }

    #[test]
    fn is_qjhvhtzxzqqjkmpb_complex_nice() {
        assert_eq!(is_complex_nice(&"qjhvhtzxzqqjkmpb".to_string()), true)
    }

    #[test]
    fn is_aabcdefgaa_complex_nice() {
        assert_eq!(is_complex_nice(&"aabcdefgaa".to_string()), false)
    }

    #[test]
    fn is_xxyxx_complex_nice() {
        assert_eq!(is_complex_nice(&"xxyxx".to_string()), true)
    }

    #[test]
    fn is_uurcxstgmygtbstg_complex_nice() {
        assert_eq!(is_complex_nice(&"uurcxstgmygtbstg".to_string()), false)
    }

    #[test]
    fn is_ieodomkazucvgmuy_complex_nice() {
        assert_eq!(is_complex_nice(&"ieodomkazucvgmuy".to_string()), false)
    }

    #[test]
    fn is_aaa_complex_nice() {
        assert_eq!(is_complex_nice(&"aaa".to_string()), false)
    }
}