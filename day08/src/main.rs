use std::collections::BinaryHeap;
use itertools::Itertools;
use union_find::{QuickUnionUf, UnionFind, UnionByRank};
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Unable to read input file");

    let boxes: Vec<(i32, i32, i32)> = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    println! {"boxes: {:?}", boxes};

    println!("problem_1: {:?}", problem_1(&boxes));
    println!("problem_2: {:?}", problem_2(&boxes));
}

fn problem_1(boxes: &Vec<(i32, i32, i32)>) -> usize {
    let n = 1000;
    let mut heap: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();

    // nlogn pairwise distance calc
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let d = dist(boxes[i], boxes[j]);
            
            if heap.len() < n {
                heap.push((d, i, j));
            } else if let Some(&(max_dist, _, _)) = heap.peek() {
                if d < max_dist {
                    heap.pop();
                    heap.push((d, i, j));
                }
            }
        }
    }

    // i think once i have my top n, I can do the union-find in any order... i.e. just kep popping them. 
    let mut uf: QuickUnionUf<UnionByRank> = QuickUnionUf::new(boxes.len());

    for junction in heap.iter() {
        uf.union(junction.1, junction.2);
    }

    let mut component_sizes: HashMap<usize, usize> = HashMap::new();
    for i in 0..boxes.len() {
        let root = uf.find(i);
        *component_sizes.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = component_sizes.into_values().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a)); // descending

    let n_biggest = 3;
    
    println!("sizes: {:?}", &sizes[..n_biggest.min(sizes.len())]);

    sizes[..n_biggest.min(sizes.len())].iter().product::<usize>()
}

fn problem_2(boxes: &Vec<(i32, i32, i32)>) -> i64 {
    let mut pairs: Vec<(i64, usize, usize)> = Vec::new();

    // nlogn pairwise distance calc
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let d = dist(boxes[i], boxes[j]);
            pairs.push((d, i, j));
        }
    }

    pairs.sort();

    let mut uf: QuickUnionUf<UnionByRank> = QuickUnionUf::new(boxes.len());
    let mut num_components = boxes.len();

    for (d, i, j) in pairs {
        if uf.find(i) != uf.find(j) {
            uf.union(i, j);
            num_components -= 1;

            if num_components == 1 {
                // solution to the problem is multiplying the x coord of the last two junction boxes
                let answer = boxes[i].0 as i64 * boxes[j].0 as i64;
                return answer;
            }
        }
    }

    return -1;
}

fn dist(box_1: (i32, i32, i32), box_2: (i32, i32, i32)) -> i64 {
    let dx = (box_2.0 - box_1.0) as i64;
    let dy = (box_2.1 - box_1.1) as i64;
    let dz = (box_2.2 - box_1.2) as i64;
    dx * dx + dy * dy + dz * dz
}