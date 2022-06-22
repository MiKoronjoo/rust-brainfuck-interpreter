use std::fs;
use std::io::Read;

const MAX_N: usize = 30000;
pub fn run_code(code: String) {
    let mut tape: [u8; MAX_N] = [0; MAX_N];
    let mut ret_addr: [usize; MAX_N / 10] = [0; MAX_N / 10];
    let mut rp: usize = 0;
    let mut ptr: usize = 0;
    let mut ip: usize = 0;
    while ip < code.len() {
        match code.as_bytes()[ip] as char {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => tape[ptr] = if tape[ptr] == 255 { 0 } else { tape[ptr] + 1 },
            '-' => tape[ptr] = if tape[ptr] == 0 { 255 } else { tape[ptr] - 1 },
            '.' => print!("{}", tape[ptr] as char),
            ',' => {
                tape[ptr] = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .unwrap();
            }
            '[' => {
                if tape[ptr] == 0 {
                    while code.as_bytes()[ip] as char != ']' {
                        ip += 1;
                    }
                } else {
                    ret_addr[rp] = ip;
                    rp += 1;
                }
            }
            ']' => {
                if tape[ptr] != 0 {
                    ip = ret_addr[rp - 1];
                } else {
                    rp -= 1;
                }
            }
            _ => continue,
        }
        ip += 1;
    }
}

pub fn run_file(path: &String) {
    let code = fs::read_to_string(path).expect("Can not read from the file");
    run_code(code);
}
