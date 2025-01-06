// This implementation was inspired by:
// https://github.com/rhoo19937/beam-search-library/blob/main/src/tree_beam.rs
// Thank you to the author for their excellent work!

use crate::input::Input;
use crate::state::State;

#[allow(non_camel_case_types)]
type uint = u32;

#[derive(Clone)]
struct Cand {
    op: char,
    parent: uint,
    score: i64,
    empty_i: usize,
    empty_j: usize,
    hash: u64,
}

impl Cand {
    fn to_node(&self) -> Node {
        Node {
            child: !0,
            prev: !0,
            next: !0,
            op: self.op,
            parent: self.parent,
            score: self.score,
        }
    }
}

#[derive(Clone, Default)]
struct Node {
    op: char,
    parent: uint,
    child: uint,
    prev: uint,
    next: uint,
    score: i64,
}

const MAX_WIDTH: usize = 3000;
const MAX_NODES: usize = MAX_WIDTH * 50;

pub struct BeamSearch {
    state: State,
    leaf: Vec<uint>,
    next_leaf: Vec<uint>,
    nodes: Vec<Node>,
    cur_node: usize,
    free: Vec<uint>,
}

impl BeamSearch {
    pub fn new(state: State, prev_op: char) -> BeamSearch {
        assert!(MAX_NODES < uint::MAX as usize, "uintのサイズが足りないよ");

        let mut score = 0;
        for i in 0..state.tile_positions.len() {
            score += state.weighted_distance(i);
        }
        score += state.mismatch_cost();

        let node = Node {
            op: prev_op,
            parent: !0,
            child: !0,
            prev: !0,
            next: !0,
            score: score as i64,
        };
        let mut nodes = vec![Node::default(); MAX_NODES];
        nodes[0] = node;

        let mut leaf = Vec::with_capacity(MAX_WIDTH);
        leaf.push(0);
        let next_leaf = Vec::with_capacity(MAX_WIDTH);
        let free = (1..nodes.len() as uint).rev().collect();

        BeamSearch { state, nodes, free, leaf, next_leaf, cur_node: 0 }
    }

    fn add_node(&mut self, cand: Cand) {
        let next = self.nodes[cand.parent as usize].child;

        let new = if let Some(n) = self.free.pop() {
            self.nodes[n as usize] = Node { next, ..cand.to_node() };
            n
        } else {
            let n = self.nodes.len() as uint;
            assert!(n != 0, "uintのサイズが足りないよ");
            self.nodes.push(Node { next, ..cand.to_node() });
            n
        };

        if next != !0 {
            self.nodes[next as usize].prev = new;
        }
        self.nodes[cand.parent as usize].child = new;

        self.next_leaf.push(new);
    }

    fn del_node(&mut self, mut idx: uint) {
        loop {
            self.free.push(idx);
            let Node { prev, next, parent, .. } = self.nodes[idx as usize];
            assert_ne!(parent, !0, "全てのノードを消そうとしています");

            if prev & next == !0 {
                idx = parent;
                continue;
            }

            if prev != !0 {
                self.nodes[prev as usize].next = next;
            } else {
                self.nodes[parent as usize].child = next;
            }
            if next != !0 {
                self.nodes[next as usize].prev = prev;
            }

            break;
        }
    }

    fn no_dfs(&mut self, cands: &mut Vec<Cand>) {
        loop {
            let Node { next, child, .. } = self.nodes[self.cur_node];
            if next == !0 || child == !0 {
                break;
            }
            self.cur_node = child as usize;
            self.state.apply(self.nodes[self.cur_node].op);
        }

        let root = self.cur_node;
        loop {
            let child = self.nodes[self.cur_node].child;
            if child == !0 {
                self.append_cands(self.cur_node, cands);
                loop {
                    if self.cur_node == root {
                        return;
                    }
                    let node = &self.nodes[self.cur_node];
                    self.state.revert(node.op);
                    if node.next != !0 {
                        self.cur_node = node.next as usize;
                        self.state.apply(self.nodes[self.cur_node].op);
                        break;
                    }
                    self.cur_node = node.parent as usize;
                }
            } else {
                self.cur_node = child as usize;
                self.state.apply(self.nodes[self.cur_node].op);
            }
        }
    }

    fn enum_cands(&mut self, cands: &mut Vec<Cand>) {
        self.no_dfs(cands);
    }

    fn update(&mut self, cands: impl Iterator<Item = Cand>) {
        self.next_leaf.clear();
        for cand in cands {
            self.add_node(cand);
        }

        for i in 0..self.leaf.len() {
            let n = self.leaf[i];
            if self.nodes[n as usize].child == !0 {
                self.del_node(n);
            }
        }

        std::mem::swap(&mut self.leaf, &mut self.next_leaf);
    }

    fn restore(&self, mut idx: uint) -> Vec<char> {
        let mut ret = vec![];
        loop {
            let Node { op, parent, .. } = self.nodes[idx as usize];
            if parent == !0 {
                break;
            }
            ret.push(op);
            idx = parent;
        }

        ret.reverse();
        ret
    }

    fn append_cands(&mut self, idx: usize, cands: &mut Vec<Cand>) {
        let node = &self.nodes[idx];
        assert_eq!(node.child, !0);

        let was_clockwise = ['1', '2', '3', '4', '5', '6'].contains(&node.op);
        let was_anticlockwise = ['A', 'B', 'C', 'D', 'E', 'F'].contains(&node.op);
        for op in "123456ABCDEF".chars() {
            if was_clockwise && ['1', '2', '3', '4', '5', '6'].contains(&op) {
                continue;
            }
            if was_anticlockwise && ['A', 'B', 'C', 'D', 'E', 'F'].contains(&op) {
                continue;
            }
            let surrounding = self.state.surrounding(0);
            let mut diff = 0;
            for &i in surrounding.iter() {
                diff -= self.state.weighted_distance(i) as i64;
            }
            diff -= self.state.mismatch_cost() as i64;
            self.state.apply(op);
            let next_hash = self.state.hash;
            for &i in surrounding.iter() {
                diff += self.state.weighted_distance(i) as i64;
            }
            diff += self.state.mismatch_cost() as i64;
            let (empty_i, empty_j) = self.state.zero_position;
            self.state.revert(op);

            let cand = Cand {
                op,
                parent: idx as uint,
                score: node.score + diff,
                empty_i,
                empty_j,
                hash: next_hash,
            };
            cands.push(cand);
        }
    }

    pub fn solve(&mut self, input: &Input) -> Vec<char> {
        let M = MAX_WIDTH;

        let mut cands: Vec<Cand> = vec![];
        let mut set = rustc_hash::FxHashSet::default();

        let mut t = 0;
        'outer: loop {
            if t != 0 {
                let M0 = (M as f64 * 2.).round() as usize;
                cands.sort_unstable_by_key(|a| a.score);
                let mut count = vec![vec![0; 2 * input.n - 1]; 2 * input.n - 1];
                let mut new_cands = vec![];
                let width = 10;
                for cand in cands.into_iter() {
                    if count[cand.empty_i][cand.empty_j] == width {
                        continue;
                    }
                    count[cand.empty_i][cand.empty_j] += 1;
                    new_cands.push(cand);
                }
                cands = new_cands;
                set.clear();

                self.update(cands.drain(..).filter(|cand| set.insert(cand.hash)).take(M));
            }
            t += 1;

            cands.clear();
            self.enum_cands(&mut cands);
            assert!(!cands.is_empty());
            let mut min_dist = 1 << 30;
            for cand in cands.iter() {
                let score = cand.score;
                if score == 0 {
                    break 'outer;
                }
                min_dist = min_dist.min(score);
            }
            if t % 10 == 0 {
                eprintln!("t: {}, dist: {}", t, min_dist);
                // let best_candidate = cands.iter().min_by_key(|a| a.score).unwrap();
                // let mut ret = self.restore(best_candidate.parent.clone());
                // ret.push(best_candidate.op.clone());
                // let ret_str = ret.iter().map(|op| op.to_string()).collect::<String>();
                // eprintln!("Current best: {}", ret_str);
            }
        }

        let best = cands.into_iter().min_by_key(|a| a.score).unwrap();

        let mut ret = self.restore(best.parent);
        ret.push(best.op);

        ret
    }
}
