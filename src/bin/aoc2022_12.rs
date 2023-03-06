use std::collections::VecDeque;

const INF: usize = 1 << 60;

fn main() {
    let mut input = vec![];
    let mut buffer = String::new();
    loop {
        buffer.clear();
        match std::io::stdin().read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => input.push(buffer.trim().to_string()),
            Err(e) => panic!("{}", e),
        }
    }
    let mut field: Vec<Vec<_>> = input
        .into_iter()
        .map(|line| line.bytes().collect())
        .collect();
    let mut start_candidates = vec![];
    let mut si = 0;
    let mut sj = 0;
    let mut ei = 0;
    let mut ej = 0;
    for (i, row) in field.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            match value {
                b'S' => {
                    si = i;
                    sj = j;
                    start_candidates.push((i, j));
                }
                b'a' => {
                    start_candidates.push((i, j));
                }
                b'E' => {
                    ei = i;
                    ej = j;
                }
                _ => (),
            }
        }
    }
    field[si][sj] = b'a';
    field[ei][ej] = b'z';

    let ans1 = bfs(&field, si, sj, ei, ej);
    println!("Part One: {}", ans1);
    let ans2 = start_candidates
        .iter()
        .map(|&(si, sj)| bfs(&field, si, sj, ei, ej))
        .min()
        .unwrap();
    println!("Part Two: {}", ans2);
}

fn bfs(field: &[Vec<u8>], si: usize, sj: usize, ei: usize, ej: usize) -> usize {
    let rows = field.len();
    let cols = field[0].len();
    let mut cost = vec![vec![INF; cols]; rows];
    cost[si][sj] = 0;
    let mut q = VecDeque::new();
    q.push_back((si, sj));
    while let Some((i, j)) = q.pop_front() {
        for (ni, nj) in adjacent_grids_4(i, j, rows, cols) {
            if field[ni][nj] > field[i][j] + 1 {
                continue;
            }
            let nc = cost[i][j] + 1;
            if cost[ni][nj] > nc {
                cost[ni][nj] = nc;
                q.push_back((ni, nj));
            }
        }
    }
    cost[ei][ej]
}

fn adjacent_grids(
    i: usize,
    j: usize,
    height: usize,
    width: usize,
    directions: &[(usize, usize)],
) -> impl Iterator<Item = (usize, usize)> + '_ {
    assert!(height < !0 && width < !0);
    directions.iter().filter_map(move |&(di, dj)| {
        let ni = i.wrapping_add(di);
        let nj = j.wrapping_add(dj);
        if ni < height && nj < width {
            Some((ni, nj))
        } else {
            None
        }
    })
}
fn adjacent_grids_4(
    i: usize,
    j: usize,
    height: usize,
    width: usize,
) -> impl Iterator<Item = (usize, usize)> {
    adjacent_grids(i, j, height, width, &[(0, 1), (1, 0), (0, !0), (!0, 0)])
}
