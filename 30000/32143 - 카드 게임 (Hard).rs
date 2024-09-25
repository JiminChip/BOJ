use std::io::{stdin, stdout, Read, Write, BufWriter};
use std::collections::{BinaryHeap, LinkedList};
use std::cmp::Reverse;
use std::usize;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let h: usize = input.next().unwrap();
    let n: usize = input.next().unwrap();
    let mut q: usize = input.next().unwrap();

    let mut card_list: Vec<usize> = vec![0; n];

    for i in 0..n {
        card_list[i] = input.next().unwrap();
    }

    card_list.sort_unstable_by(|a, b| b.cmp(a));

    let mut flag: bool = false;
    let mut flag2: bool = false;
    let mut res: usize = 0;
    let mut idx: usize = n;
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);

    let mut min_heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for i in 0..n {
        res += card_list[i];
        if res >= h {
            flag = true;
            flag2 = true;
            idx = i;
            let mut cnt: usize = 0;
            for val in card_list.iter() {
                min_heap.push(Reverse(*val));
                if idx == cnt {
                    break;
                }
                cnt += 1;
            }
            writeln!(writer, "{}", idx+1).unwrap();
            break;
        }
    }

    let mut card_list: LinkedList<usize> = card_list.into_iter().collect();
    while flag == false {
        writeln!(writer, "-1").unwrap();

        if q == 0 {
            break;
        }

        q -= 1;
        let new_card: usize = input.next().unwrap();
        card_list.push_back(new_card);
        res += new_card;
        if res >= h {
            flag = true;
            break;
        }
        idx += 1;
    }

    if flag == true {
        if flag2 == false {
            for val in card_list.iter() {
                min_heap.push(Reverse(*val));
            }

            loop {
                if let Some(Reverse(top)) = min_heap.peek() {
                    if (res - *top) >= h {
                        res -= *top;
                        min_heap.pop();
                        idx -= 1;
                    }
                    else {
                        break;
                    }
                }
            }
            writeln!(writer, "{}", idx+1).unwrap();
        }
        for _i in 0..q {
            let new_card = input.next().unwrap();
            if let Some(Reverse(top)) = min_heap.peek() {
                if *top < new_card {
                    res += new_card;
                    min_heap.push(Reverse(new_card));
                    idx += 1;
                    loop {
                        if let Some(Reverse(top)) = min_heap.peek() {
                            if (res - *top) >= h {
                                res -= *top;
                                min_heap.pop();
                                idx -= 1;
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
            }
            writeln!(writer, "{}", idx+1).unwrap();
        }
    }
}