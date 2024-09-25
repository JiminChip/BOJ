use std::io::{stdin, Read};

#[derive(Clone)]
struct PrefixSum {
    val: isize,
    idx: usize,
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<isize>);

    let n: usize = input.next().unwrap() as usize;

    let default_val = PrefixSum { val: 0, idx: 0 };
    let mut liq_list: Vec<PrefixSum> = vec![default_val; n+1];
    liq_list[0] = PrefixSum { val: 0, idx: 0 };

    let mut sum: isize = 0;
    let mut min: PrefixSum = PrefixSum { val: 0, idx: 0 };
    for i in 1..=n {
        let val = input.next().unwrap();
        if i == 1 {
            min = PrefixSum { val: val, idx: i };
        }
        else {
            if min.val.abs() > val.abs() {
                min = PrefixSum { val: val, idx: i };
            }
        }
        sum += val;
        liq_list[i] = PrefixSum{ val: sum, idx: i };
    }

    if n == 1 {
        println!("{}", min.val);
        println!("1 1");
        return;
    }

    liq_list.sort_unstable_by_key(|item| item.val);

    let mut min2: PrefixSum = PrefixSum { val: 0, idx: 0 };
    for i in 0..n {
        let val: isize = liq_list[i].val - liq_list[i+1].val;
        if i == 0 {
            min2.val = val;
            min2.idx = i;
        }
        else {
            if min2.val.abs() > val.abs() {
                min2.val = val;
                min2.idx = i;
            }
        }
    }

    if min.val.abs() <= min2.val.abs() {
        println!("{}", min.val);
        println!("{} {}", min.idx, min.idx);
    }
    else {
        let tmp1: &PrefixSum = &liq_list[min2.idx];
        let tmp2: &PrefixSum = &liq_list[min2.idx + 1];

        if tmp1.idx > tmp2.idx {
            println!("{}", tmp1.val - tmp2.val);
            println!("{} {}", tmp2.idx + 1, tmp1.idx);
        }
        else {
            println!("{}", tmp2.val - tmp1.val);
            println!("{} {}", tmp1.idx + 1, tmp2.idx);
        }
    }
}