use std::io::{stdin, stdout, Read, BufWriter, Write};

fn main() {
    let mut test_case = String::new();
    stdin().read_line(&mut test_case).unwrap();

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    
    let modular = 1_000_000_007;

    let mut res: Vec<Vec<usize>> = vec![vec![0; 1_000_001]; 4];
    for i in 1..1_000_001 {
        res[0][i] = 1;
    }
    res[1][1] = 1;
    res[1][2] = 2;
    for i in 3..1_000_001 {
        res[1][i] = (res[1][i-1] * res[1][i-2] + res[1][i-1] * res[1][i-1]) % modular;
    }
    res[2][1] = 1;
    res[2][2] = 2;
    for i in 3..1_000_001 {
        res[2][i] = (res[2][i-1] * res[2][i-2] * 2) % modular;
    }
    res[3][1] = 1;
    res[3][2] = 3;
    for i in 3..1_000_001 {
        res[3][i] = (res[3][i-1] * res[3][i-2] * 2 + res[3][i-1] * res[3][i-1]) % modular;
    }

    for chunk in lines.chunks(2) {
        match chunk {
            [line1, line2] => {
                let mut first_line = line1.split_ascii_whitespace().flat_map(str::parse::<usize>);
                let h = first_line.next().unwrap();
                let mut out: usize = 0;
                match *line2 {
                    "0" | "-1" | "1" => {
                        out = res[0][h];
                    },
                    "-1 0" | "0 1" => {
                        out = res[1][h];
                    },
                    "-1 1" => {
                        out = res[2][h];
                    },
                    "-1 0 1" => { 
                        out = res[3][h];
                    },
                    _ => {
                    },
                }
                writeln!(writer, "{out}").unwrap();
            }
            _ => {}
        }
    }
}