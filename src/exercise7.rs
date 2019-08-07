use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Statement {
    Constant(u16),
    Value(String),
}

#[derive(Debug)]
enum Operation {
    Add(Statement, Statement),
    Or(Statement, Statement),
    LShift(Statement, Statement),
    RShift(Statement, Statement),
    Not(Statement),
    Assign(Statement),
}

pub fn ex7() {
    let lines = fs::read_to_string("resources/2015/ex7_in")
        .expect("Couldn't read input");

    let signals = compute_signals(lines.as_str());
    eprintln!("signals[\"a\"] = {:?}", signals["a"]);
}

fn compute_signals(code: &str) -> HashMap<String, u16> {
    let mut operations = read_operations(code);
    resolve_signals(&mut operations)
}

fn read_operations(code: &str) -> HashMap<String, Operation> {
    let mut operations: HashMap<String, Operation> = HashMap::new();
    for line in code.lines() {
        let x: Vec<&str> = line.split(" -> ").collect();
        if let [instr, wire_name] = &*x {
            let operation = if instr.contains("AND") {
                parse_and(instr)
            } else if instr.contains("OR") {
                parse_or(instr)
            } else if instr.contains("LSHIFT") {
                parse_lshift(instr)
            } else if instr.contains("RSHIFT") {
                parse_rshift(instr)
            } else if instr.contains("NOT") {
                parse_not(instr)
            } else {
                Operation::Assign(parse_statement(instr))
            };
            operations.insert(wire_name.to_string(), operation);
        }
    }
    operations
}

fn resolve_signals(operations: &mut HashMap<String, Operation>) -> HashMap<String, u16> {
    let mut resolved: HashMap<String, u16> = HashMap::new();
    loop {
        if operations.is_empty() { break; }
        for (wire_name, operation) in operations.iter() {
            let v: () = match operation {
                Operation::Add(x, y) =>
                    insert_if_resolved_two(&mut resolved, wire_name, x, y, |a, b| a & b),
                Operation::Or(x, y) =>
                    insert_if_resolved_two(&mut resolved, wire_name, x, y, |a, b| a | b),
                Operation::LShift(x, y) =>
                    insert_if_resolved_two(&mut resolved, wire_name, x, y, |a, b| a << b),
                Operation::RShift(x, y) =>
                    insert_if_resolved_two(&mut resolved, wire_name, x, y, |a, b| a >> b),
                Operation::Not(x) =>
                    insert_if_resolved_one(&mut resolved, wire_name, x, 0, |a, b| !a),
                Operation::Assign(x) =>
                    insert_if_resolved_one(&mut resolved, wire_name, x, 0, |a, b| a),
            };
        }
        resolved.keys().for_each(|k| { operations.remove(k); });
    }
    resolved
}

fn insert_if_resolved_two<F>(resolved: &mut HashMap<String, u16>, wire_name: &String, x: &Statement, y: &Statement, f: F)
    where F: Fn(u16, u16) -> u16 {
    let val = match (x, y) {
        (Statement::Constant(val1), Statement::Constant(val2)) => Some(f(*val1, *val2)),
        (Statement::Value(val1), Statement::Constant(val2)) =>
            if resolved.contains_key(val1) { Some(f(resolved[val1], *val2)) } else { None },
        (Statement::Constant(val1), Statement::Value(val2)) =>
            if resolved.contains_key(val2) { Some(f(*val1, resolved[val2])) } else { None },
        (Statement::Value(val1), Statement::Value(val2)) =>
            if resolved.contains_key(val1) && resolved.contains_key(val2) {
                Some(f(resolved[val1], resolved[val2]))
            } else { None },
    };
    if let Some(val) = val {
        resolved.insert(wire_name.clone(), val);
    }
}

fn insert_if_resolved_one<F>(resolved: &mut HashMap<String, u16>, wire_name: &String, x: &Statement, y: u16, f: F)
    where F: Fn(u16, u16) -> u16 {
    let val = match x {
        (Statement::Constant(x)) => Some(f(*x, y)),
        (Statement::Value(x)) =>
            if resolved.contains_key(x) { Some(f(resolved[x], y)) } else { None },
    };
    if let Some(val) = val {
        resolved.insert(wire_name.clone(), val);
    }
}

fn insert(resolved: &mut HashMap<String, u16>, wire_name: &String, val: u16) {
    resolved.insert(wire_name.clone(), val);
}

fn parse_and(instr: &str) -> Operation {
    let (x, y) = parse_with_two_args(instr, " AND ");
    Operation::Add(x, y)
}

fn parse_or(instr: &str) -> Operation {
    let (x, y) = parse_with_two_args(instr, " OR ");
    Operation::Or(x, y)
}

fn parse_lshift(instr: &str) -> Operation {
    let (x, y) = parse_with_two_args(instr, " LSHIFT ");
    Operation::LShift(x, y)
}

fn parse_rshift(instr: &str) -> Operation {
    let (x, y) = parse_with_two_args(instr, " RSHIFT ");
    Operation::RShift(x, y)
}

fn parse_not(instr: &str) -> Operation {
    let (_, x) = parse_with_two_args(instr, "NOT ");
    Operation::Not(x)
}

fn parse_with_two_args(instr: &str, separator: &str) -> (Statement, Statement) {
    let collection: Vec<&str> = instr.split(separator).collect();
    let mut iterator = collection.iter();
    let x = *iterator.next().expect(&format!("No first arg in '{}'", instr));
    let y = *iterator.next().expect(&format!("No second arg in '{}'", instr));
    (parse_statement(x), parse_statement(y))
}

fn parse_statement(instr: &str) -> Statement {
    match instr.parse::<u16>() {
        Ok(val) => Statement::Constant(parse_const(instr)),
        Err(val) => Statement::Value(instr.to_string()),
    }
}

fn parse_const(instr: &str) -> u16 {
    instr.parse::<u16>().expect(&format!("Couldn't parse '{}' to u16", instr))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let code = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        let signals = compute_signals(code);

        assert_eq!(signals["d"], 72);
        assert_eq!(signals["e"], 507);
        assert_eq!(signals["f"], 492);
        assert_eq!(signals["g"], 114);
        assert_eq!(signals["h"], 65412);
        assert_eq!(signals["i"], 65079);
        assert_eq!(signals["x"], 123);
        assert_eq!(signals["y"], 456);
    }
}