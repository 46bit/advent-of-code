use std::fmt;
use core::cmp::{min, max};
use std::collections::{HashMap, HashSet, BTreeMap};

#[aoc(day23, part1)]
fn part1(input: &str) -> u64 {
    let (_, yx_graph, y_max, x_max) = old_part1(input);
    print_yx_graph(&yx_graph, y_max, x_max);
    println!();

    let mut cheapest_way_to_state: HashMap<YXGraph, u64> = HashMap::new();
    let mut previous_state: HashMap<YXGraph, YXGraph> = HashMap::new();
    let mut winning_state = None;

    let mut possibilities = vec![yx_graph.clone()];
    cheapest_way_to_state.insert(yx_graph.clone(), 0);
    while let Some(possibility) = possibilities.pop() {
        let cost = cheapest_way_to_state[&possibility];

        for (next_state, cost2) in next_states(possibility.clone(), cost, y_max, x_max) {
            if winning_state.is_none() && won(&next_state, x_max) {
                winning_state = Some(next_state.clone());
            }

            if cheapest_way_to_state.contains_key(&next_state) && cheapest_way_to_state[&next_state] <= cost2 {
                continue;
            }
            cheapest_way_to_state.insert(next_state.clone(), cost2);
            previous_state.insert(next_state.clone(), possibility.clone());
            possibilities.push(next_state);
        }
    }

    let winning_state = winning_state.unwrap();
    println!("-----");
    println!("WINNING:");
    print_yx_graph(&winning_state, y_max, x_max);
    println!("cost = {}", cheapest_way_to_state[&winning_state]);
    println!("-----");

    let mut curr = winning_state.clone();
    while let Some(prev) = previous_state.get(&curr) {
        let cost = cheapest_way_to_state[&prev];
        print_yx_graph(&prev, y_max, x_max);
        println!("cost = {}", cost);
        println!("-----");
        curr = prev.clone();
    }

    return cheapest_way_to_state[&winning_state];
}

type Coord = (usize, usize);
type YXGraph = BTreeMap<Coord, Node>;

fn next_states(yx_graph: YXGraph, cost: u64, y_max: usize, x_max: usize) -> Vec<(YXGraph, u64)> {
    let mut possibilities = vec![];
    for ((y, x), cell) in yx_graph.clone() {
        if let Some(agent) = cell.agent {
            for (y2, x2) in allowed_hallway_moves(&agent, &yx_graph, x_max) {
                assert_eq!(y2, 0);
                let mut next = yx_graph.clone();
                let mut agent = agent.clone();
                agent.y = y2;
                agent.x = x2;
                let from = next.get_mut(&(y, x)).unwrap();
                from.agent = None;
                let to = next.get_mut(&(y2, x2)).unwrap();
                to.agent = Some(agent.clone());
                let dist = y + x2.abs_diff(x);
                possibilities.push((next, cost + agent.move_cost() * dist as u64));
            }
            for (y2, x2) in allowed_room_moves(&agent, &yx_graph, y_max) {
                assert!(y2 > 0);
                let mut next = yx_graph.clone();
                let mut agent = agent.clone();
                agent.y = y2;
                agent.x = x2;
                let from = next.get_mut(&(y, x)).unwrap();
                from.agent = None;
                let to = next.get_mut(&(y2, x2)).unwrap();
                to.agent = Some(agent.clone());
                let dist = y2.abs_diff(y) + x2.abs_diff(x);
                possibilities.push((next, cost + agent.move_cost() * dist as u64));
            }
        }
    }
    return possibilities;
}

fn print_yx_graph(yx_graph: &YXGraph, y_max: usize, x_max: usize) {
    for y in 0..=y_max {
        for x in 0..=x_max {
            if !yx_graph.contains_key(&(y, x)) {
                print!(" ");
            } else {
                print!("{}", yx_graph[&(y, x)]);
            }
        }
        println!();
    }
}

fn won(yx_graph: &YXGraph, x_max: usize) -> bool {
    for x in 0..x_max {
        if yx_graph[&(0, x)].agent.is_some() {
            return false;
        }
    }
    let room_xs = &[2, 4, 6, 8];
    let room_height = 2;
    let mut targets = HashMap::new();
    targets.insert('A', room_xs[0]);
    targets.insert('B', room_xs[1]);
    targets.insert('C', room_xs[2]);
    targets.insert('D', room_xs[3]);
    for y in 1..=room_height {
        for room_x in room_xs {
            let node = &yx_graph[&(y, *room_x)];
            assert!(node.occupyable);
            assert_eq!(node.kind, NodeKind::Room);
            match &node.agent {
                Some(agent) => {
                    if agent.kind != node.desired_agent_kind {
                        return false;
                    }
                },
                None => return false,
            }
        }
    }
    return true;
}

fn allowed_hallway_moves(agent: &Agent, yx_graph: &YXGraph, x_max: usize) -> Vec<Coord> {
    if agent.y == 0 {
        return vec![];
    }

    if yx_graph[&(agent.y, agent.x)].desired_agent_kind == agent.kind {
        let mut alien_below = false;
        for y in (agent.y + 1).. {
            if let Some(node) = yx_graph.get(&(y, agent.x)) {
                if let Some(ref other_agent) = node.agent {
                    if other_agent.kind != agent.kind {
                        alien_below = true;
                        break;
                    }
                }
            } else {
                break;
            }
        }
        if !alien_below {
            return vec![];
        }
    }

    if agent.y > 1 {
        for y in 1..agent.y {
            if yx_graph[&(y, agent.x)].agent.is_some() {
                return vec![];
            }
        }
    }

    let mut hallways: Vec<_> = (0..=x_max).into_iter().collect();
    for x in 0..agent.x {
        if yx_graph[&(0, x)].agent.is_some() {
            hallways = hallways
                .into_iter()
                .filter(|x2| *x2 >= x)
                .collect();
        }
    }
    for x in (agent.x+1)..=x_max {
        if yx_graph[&(0, x)].agent.is_some() {
            hallways = hallways
                .into_iter()
                .filter(|x2| *x2 <= x)
                .collect();
        }
    }

    hallways
        .into_iter()
        .map(|x| (0, x))
        .filter(|c| yx_graph[&c].occupyable)
        .filter(|c| yx_graph[&c].agent.is_none())
        .collect()
}

fn allowed_room_moves(agent: &Agent, yx_graph: &YXGraph, y_max: usize) -> Vec<Coord> {
    if agent.y > 0 {
        return vec![];
    }

    let room_xs = [2, 4, 6, 8];
    for x in room_xs {
        let mut blocked = false;
        for x2 in min(agent.x, x)..=max(agent.x, x) {
            if x2 != agent.x && yx_graph[&(agent.y, x2)].agent.is_some() {
                blocked = true;
                break;
            }
        }
        if blocked {
            continue;
        }

        if yx_graph[&(1, x)].desired_agent_kind != agent.kind {
            continue;
        }
        let mut last = None;
        for y in 1..=y_max {
            if yx_graph[&(y, x)].agent.is_none() {
                last = Some((y, x));
                continue;
            }
            if yx_graph[&(y, x)].agent.as_ref().unwrap().kind != agent.kind {
                return vec![];
            }
        }
        if let Some(yy) = last {
            return vec![yy];
        }
    }
    return vec![];
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    y: usize,
    x: usize,
    kind: NodeKind,
    occupyable: bool,
    desired_agent_kind: char,
    agent: Option<Agent>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NodeKind {
    Hallway,
    Room,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            NodeKind::Hallway => {
                if self.occupyable {
                    match &self.agent {
                        None => write!(f, "."),
                        Some(agent) => write!(f, "{}", agent.kind.to_ascii_lowercase()),
                    }
                } else {
                    write!(f, "+")
                }
            },
            NodeKind::Room => {
                match &self.agent {
                    None => write!(f, "r"),
                    Some(agent) => write!(f, "{}", agent.kind),
                }
            },            
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Agent {
    kind: char,
    y: usize,
    x: usize,
}

impl Agent {
    fn move_cost(&self) -> u64 {
        match self.kind {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

// #[aoc(day23, part1)]
fn old_part1(input: &str) -> (Vec<Agent>, YXGraph, usize, usize) {
    let mut hallways = HashSet::new();

    let mut agents = vec![];
    let mut yx_graph = BTreeMap::new();
    //let mut room_xs = HashSet::new();
    let room_xs = &[2, 4, 6, 8];
    
    for (y, line) in input.lines().enumerate() {
        for (x, char_) in line.chars().enumerate() {
            match char_ {
                '.' => {
                    yx_graph.insert((y, x), Node{
                        y,
                        x,
                        kind: NodeKind::Hallway,
                        occupyable: true,
                        desired_agent_kind: ' ',
                        agent: None,
                    });
                    hallways.insert((y, x));
                },
                'A' | 'B' | 'C' | 'D' => {
                    let agent = Agent{kind: char_, y, x};
                    agents.push(agent.clone());
                    if y == 0 {
                        yx_graph.insert((y, x), Node{
                            y,
                            x,
                            kind: NodeKind::Hallway,
                            occupyable: true,
                            desired_agent_kind: ' ',
                            agent: Some(agent),
                        });
                        hallways.insert((y, x));
                    } else {
                        let desired_agent_kind = match x {
                            2 => 'A',
                            4 => 'B',
                            6 => 'C',
                            8 => 'D',
                            _ => unreachable!(),
                        };
                        yx_graph.insert((y, x), Node{
                            y,
                            x,
                            kind: NodeKind::Room,
                            desired_agent_kind,
                            occupyable: true,
                            agent: Some(agent),
                        });
                        //room_xs.insert(x);
                    }
                },
                _ => continue,
            }
        }
    }

    // Manual hack to recognise entranceways and find bounds
    let mut y_max = 0;
    let mut x_max = 0;
    for ((y, x), node) in &mut yx_graph {
        if *y > y_max {
            y_max = *y;
        }
        if *x > x_max {
            x_max = *x;
        }

        if *y == 0 && room_xs.contains(x) && node.kind == NodeKind::Hallway {
            hallways.remove(&(*y, *x));
            node.occupyable = false;
        }

        if *y > 0 && room_xs.contains(x) {
            let desired_agent_kind = match x {
                2 => 'A',
                4 => 'B',
                6 => 'C',
                8 => 'D',
                _ => unreachable!(),
            };
            node.desired_agent_kind = desired_agent_kind;
            node.kind = NodeKind::Room;
        }
    }

    return (agents, yx_graph, y_max, x_max);
}
