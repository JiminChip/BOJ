use std::io::stdin;

fn main() {
    let mut first_line = String::new();
    stdin()
        .read_line(&mut first_line)
        .expect("Failed to read line");

    let nm: Vec<usize> = first_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse"))
        .collect();

    let mut sec_line = String::new();
    stdin()
        .read_line(&mut sec_line)
        .expect("Failed to read line");

    let se: Vec<usize> = sec_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse"))
        .collect();

    let n = nm[0];
    let m = nm[1];
    let mut dp: Vec<Vec<f64>> = vec![vec![0.0; m + 1]; n + 2];

    let s = se[0];
    let e = se[1];

    let factor: f64 = 1.0 / (n - 1) as f64;
    let factor2: f64 = (n - 3) as f64 / (n - 1) as f64;
    let factor3: f64 = (n - 2) as f64 / (n - 1) as f64;
    dp[s][0] = 1.0;
    for i in 1..=m {
        for j in 1..=n {
            if j == 1 || j == n {
                dp[j][i] = (dp[j-1][i-1] + dp[j+1][i-1]) * factor + (dp[j][i-1] * factor3);
            }
            else {
                dp[j][i] = (dp[j-1][i-1] + dp[j+1][i-1]) * factor + (dp[j][i-1] * factor2);
            }
        }
    }

    println!("{}", dp[e][m]);
}