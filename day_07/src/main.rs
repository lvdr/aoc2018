use std::io::prelude::*;
use std::fs::File;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;


#[derive(Eq, Copy, Clone, Hash)]
struct Node {
    id: u8
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        // Reverse order; to make min heap
        other.id.cmp(&self.id)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id
    }
}

#[derive(Clone)]
struct Relations {
    antidependencies: HashSet<Node>,
    dependencies: HashSet<Node>,
}

#[derive(Debug, Eq, Copy, Clone, Hash)]
struct Worker {
    active: bool,
    finish_time: u32,
    task: u8,
}


impl Ord for Worker {
    fn cmp(&self, other: &Worker) -> Ordering {
        // Reverse order; to make min heap
        self.active.cmp(&other.active)
            .then_with(|| other.finish_time.cmp(&self.finish_time))
            .then_with(|| other.task.cmp(&self.task))
    }
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Worker) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Worker {
    fn eq(&self, other: &Worker) -> bool {
        self.task == other.task
    }
}

impl Relations {
    fn new() -> Relations {
        Relations { antidependencies: HashSet::new(),
                    dependencies: HashSet::new() }
    }
}

fn get_next_ready_worker(workers: &Vec<Worker>) -> Option<usize> {
    let mut next_worker = None;
    let mut next_time = None;
    for i in 0..5 as usize {
        let worker = workers[i];
        if worker.active {
            if next_time == None || next_time.unwrap() > worker.finish_time {
                next_time = Some(worker.finish_time);
                next_worker = Some(i);
            }
        }
    }
    next_worker
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let lines : Vec<&str> = input.trim()
                                 .split("\n")
                                 .map(|x| x.trim())
                                 .collect();

    let mut graph = HashMap::new();
    for line in lines {
        let dependency = Node { id: line.as_bytes()[5 as usize] };
        let node = Node { id: line.as_bytes()[36 as usize] };
        graph.entry(node).or_insert( Relations::new() )
             .dependencies.insert(dependency);
        graph.entry(dependency).or_insert( Relations::new() )
             .antidependencies.insert(node);
    }

    let graph_cpy = graph.clone();

    let mut queue = BinaryHeap::new();
    for (node, rels) in &graph {
        if rels.dependencies.is_empty() {
            queue.push(*node);
        }
    }

    let queue_cpy = queue.clone();

    let mut exec_order = String::new();
    while !queue.is_empty() {
        let next_task = queue.pop().unwrap();
        exec_order.push(next_task.id as char);

        for antidep in &graph.get(&next_task).unwrap().antidependencies.clone() {
            graph.get_mut(antidep).unwrap().dependencies.remove(&next_task);
            if graph.get(antidep).unwrap().dependencies.is_empty() {
                queue.push(*antidep);
            }
        }
    }

    println!("Exec order: {}", exec_order);

    let mut graph = graph_cpy;
    let mut queue = queue_cpy;
    let mut workers = vec![ Worker {active: false, finish_time: 0, task: 0 }; 5 ];
    let mut time = 0;

    loop {
        for i in 0..5 {
            if !queue.is_empty() && workers[i].active == false {
                let next_task = queue.pop().unwrap();
                workers[i].active = true;
                workers[i].task = next_task.id;
                workers[i].finish_time = time + (next_task.id - 'A' as u8) as u32 + 61;
            }
        }
        let ret = get_next_ready_worker(&workers);
        if ret == None {
            break;
        }

        let ret = ret.unwrap();
        let task = Node {id: workers[ret].task };
        time = workers[ret].finish_time;

        for antidep in &graph.get(&task).unwrap().antidependencies.clone() {
            graph.get_mut(antidep).unwrap().dependencies.remove(&task);
            if graph.get(antidep).unwrap().dependencies.is_empty() {
                queue.push(*antidep);
            }
        }

        workers[ret].active = false;
    }

    println!("Execution time: {}", time);
}
