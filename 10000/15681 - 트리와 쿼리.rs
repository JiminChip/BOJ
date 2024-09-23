use std::io::{stdin, stdout, Read, BufWriter, Write};
use std::collections::LinkedList;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n: usize = input.next().unwrap();
    let r: usize = input.next().unwrap();
    let q: usize = input.next().unwrap();

    let mut connect_list: Vec<LinkedList<usize>> = vec![LinkedList::new(); n+1];

    for _i in 1..n {
        let node1: usize = input.next().unwrap();
        let node2: usize = input.next().unwrap();
        
        connect_list[node1].push_back(node2);
        connect_list[node2].push_back(node1);
    }

    let mut res_list: Vec<usize> = vec![0; n+1];

    make_tree(r, 0, &connect_list, &mut res_list);

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);

    for _i in 0..q {
        let cur_q = input.next().unwrap();
        writeln!(writer, "{}", res_list[cur_q]).unwrap();
    }
}

fn make_tree(current_node: usize, parent: usize, connect_list: &Vec<LinkedList<usize>>, res_list: &mut Vec<usize>) {
    let mut res: usize = 0;
    for node in &connect_list[current_node] {
        if *node != parent {
            make_tree(*node, current_node, connect_list, res_list);
            res += res_list[*node];
        }
    }
    res_list[current_node] = res + 1;
}