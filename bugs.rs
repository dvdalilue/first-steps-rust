use std::io::{self, BufRead, Lines, StdinLock};

macro_rules! next_line {
    ($it:expr) => { $it.next().unwrap().unwrap(); };
}

macro_rules! next_number {
    ($it:expr) => { $it.next().unwrap().parse::<usize>().unwrap(); };
}

macro_rules! new_zeros_vec {
    ($n:expr) => { std::iter::repeat(0).take($n).collect::<Vec<u32>>(); };
}

fn dfs_aux(
    mut current: usize,
    graph: &Vec<Vec<usize>>,
    colors: &mut Vec<u32>,
    parents: &mut Vec<u32>,
    visited: &mut Vec<u32>
) -> bool {
    let mut stack = Vec::new(); stack.push(current);

    if parents[current] == 0 { parents[current] = 1 }

    while stack.len() > 0 {
        current = stack.pop().unwrap();

        if visited[current] == 1 {
            stack.extend(&graph[current]);
            continue;
        }

        visited[current] = 1;

        let mut color = colors[current];
        if color != 0 {
            if parents[current] != 0 && color == parents[current] {
                return true;
            }

            let mut j = 0;
            while j < graph[current].len() {
                if colors[graph[current][j]] == color {
                    return true;
                }

                j += 1;
            }
        }

        if color == 0 {
            color = 1;

            if parents[current] == 1 {
                color = 2;
            }
        }

        if color == parents[current] { return true; }
        colors[current] = color;

        let mut i = 0;
        while i < graph[current].len() {
            if parents[graph[current][i]] != 0 &&
                parents[graph[current][i]] != color {
                return true;
            }

            parents[graph[current][i]] = color;
            i += 1;
        }

        stack.extend(&graph[current]);
    }

    false
}

fn dfs(nodes: usize, graph: &Vec<Vec<usize>>) {
    let mut colors = new_zeros_vec!(nodes);
    let mut parents = new_zeros_vec!(nodes);
    let mut visited = new_zeros_vec!(nodes);

    let mut bug = false;

    let mut i = 0;

    while i < nodes {
        if colors[i] == 0 {
            if dfs_aux(i, graph, &mut colors, &mut parents, &mut visited) {
                bug = true;
                break;
            }
        }

        i += 1
    }

    if bug {
        println!("Suspicious bugs found!");
    } else {
        println!("No suspicious bugs found!");
    }
}

fn scenario(it: &mut Lines<StdinLock<'_>>) {
    let mut line: String = next_line!(*it);

    let nodes: usize;
    let edges: usize;

    {
        let mut split = line.split(' ');
        nodes = next_number!(split);
        edges = next_number!(split);
    }

    let mut from: usize;
    let mut to: usize;

    let mut graph: Vec<Vec<usize>> = std::iter::repeat(Vec::new())
        .take(nodes)
        .collect::<Vec<Vec<usize>>>();

    let mut j = 0;
    while j < edges {
        j += 1;

        line = next_line!(*it);

        {
            let mut split = line.split(' ');

            from = next_number!(split);
            to = next_number!(split);
        }

        graph[from - 1].push(to - 1);
    }

    dfs(nodes, &graph);
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();

    let n = next_line!(iter).parse::<u8>().unwrap();
    let mut i = 0;

    while i < n {
        i += 1;
        println!("Scenario #{}:", i);

        scenario(&mut iter);
    }
}