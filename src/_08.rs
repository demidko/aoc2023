use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rusty_pool::ThreadPool;

#[derive(Clone)]
struct LR {
    vec: Vec<usize>,
    pos: usize,
}

impl LR {
    fn parse(str: &str) -> Self {
        Self {
            vec: str.chars()
                .map(|char| match char {
                    'L' => 0,
                    'R' => 1,
                    _ => panic!("Unexpected char {}", char)
                }).collect_vec(),
            pos: 0,
        }
    }

    fn lr(&mut self) -> usize {
        let element = self.vec[self.pos];
        self.pos += 1;
        if self.pos == self.vec.len() {
            self.pos = 0;
        }
        element
    }
}

#[derive(Debug)]
struct Tree {
    pos: usize,
    zzz: HashSet<usize>,
    nodes: Vec<usize>,
    jumps: u128,
}

impl Tree {
    fn is_zzz(&self) -> bool {
        self.zzz.contains(&self.pos)
    }

    fn jump(&mut self, lr: usize) {
        self.pos = self.nodes[self.pos + lr];
        self.jumps += 1;
    }

    fn compile(nodes: Vec<&str>) -> Vec<Self> {
        let primitive_map =
            nodes.iter().map(|&str| {
                let (name, lr) = str.split_once(" = ").unwrap();
                let (l, r) = lr[1..lr.len() - 1].split_once(", ").unwrap();
                (name, (l, r))
            }).collect::<HashMap<&str, (&str, &str)>>();
        let mut id = 0usize;
        let mut ids = Vec::new();
        let mut name_to_id = HashMap::new();
        let mut id_to_name = HashMap::new();
        for (&name, _) in primitive_map.iter() {
            name_to_id.insert(name, id);
            id_to_name.insert(id, name);
            ids.push(id);
            id += 2;
        }

        let mut nodes = Vec::new();
        for id in &ids {
            let name = id_to_name.get(id).copied().unwrap();
            let (l, r) = primitive_map.get(name).copied().unwrap();
            let l = name_to_id.get(l).copied().unwrap();
            let r = name_to_id.get(r).copied().unwrap();
            nodes.push(l);
            nodes.push(r);
        }


        let mut pos = HashSet::new();
        let mut zzz = HashSet::new();

        for (name, id) in name_to_id {
            match &name[2..3] {
                "A" => pos.insert(id),
                "Z" => zzz.insert(id),
                _ => continue
            };
        };

        pos.iter().map(|&pos| {
            Self {
                pos,
                zzz: zzz.clone(),
                nodes: nodes.clone(),
                jumps: 0u128,
            }
        }).collect_vec()
    }
}


struct TreeSolver {
    program: LR,
    tree: Tree,
}

impl TreeSolver {
    fn create(program: LR, tree: Tree) -> Self {
        Self { program, tree }
    }

    fn is_zzz(&self) -> bool {
        self.tree.is_zzz()
    }

    fn jump(&mut self) {
        let lr = self.program.lr();
        self.tree.jump(lr);
    }

    fn jumps(mut self, jumps: u128) -> Self {
        for _ in 0..jumps {
            self.jump();
        }
        self
    }

    fn jumps_counter(&self) -> u128 {
        self.tree.jumps
    }

    fn go_to_zzz(mut self) -> Self {
        loop {
            self.jump();
            if self.is_zzz() {
                break;
            }
        }
        self
    }
}

struct Platoon {
    soldiers: Vec<TreeSolver>
}

impl Platoon {
    fn new() -> Self {
        Self {
            thread_pool: ThreadPool::default(),
            soldiers: vec![],
        }
    }

    fn subdue_soldier(&mut self, soldier: TreeSolver) {
        self.soldiers.push(soldier)
    }

    fn march_to_zzz(&mut self) {
        self.soldiers =
            (0..self.soldiers.len())
                .filter_map(|i| self.soldiers.pop())
                .map(|s| self.thread_pool.evaluate(|| s.go_to_zzz()))
                .map(|t| t.await_complete())
                .collect_vec();
    }

    fn panic(&self) {
        let view = self.soldiers.iter().map(|x| &x.tree).collect_vec();
        panic!("{:?}", view)
    }

    fn stand_ho(&mut self) {
        let best_jumps_counter =
            self.soldiers.iter()
                .map(|soldier| soldier.jumps_counter())
                .max()
                .unwrap();
        self.soldiers =
            (0..self.soldiers.len())
                .filter_map(|i| self.soldiers.pop())
                .map(|s| self.thread_pool.evaluate(move || {
                    let curr_jumps = s.jumps_counter();
                    let need_jumps = best_jumps_counter - curr_jumps;
                    s.jumps(need_jumps)
                }))
                .map(|t| t.await_complete())
                .collect_vec();
    }

    fn is_zzz(&self) -> bool {
        let random_jumps_counter = self.jumps_counter();
        for solver in &self.soldiers {
            if solver.is_zzz() && solver.jumps_counter() == random_jumps_counter {
                continue;
            }
            return false;
        }
        true
    }

    fn jumps_counter(&self) -> u128 {
        unsafe {
            self.soldiers.get_unchecked(0).jumps_counter()
        }
    }
}


#[test]
fn part_two_solution() {
    let mut input = include_str!("../08.input").trim().lines();
    let program = input.next().unwrap();
    let program = LR::parse(program);
    let input = input.skip(1).collect_vec();
    let mut platoon = Platoon::new();
    for tree in Tree::compile(input) {
        let program = program.clone();
        let solver = TreeSolver::create(program, tree);
        platoon.subdue_soldier(solver);
    }
    loop {
        platoon.march_to_zzz();
        if platoon.is_zzz() {
            break;
        }
        platoon.stand_ho();
        if platoon.is_zzz() {
            break;
        }
    }
    println!("{}", platoon.jumps_counter())
}