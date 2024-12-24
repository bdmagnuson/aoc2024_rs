use std::fs; 
use std::collections::VecDeque;
use pest::Parser;
use pest_derive::Parser;
use petgraph::graph::{UnGraph, NodeIndex};
use rustc_hash::{FxHashSet as HashSet, FxHashMap as HashMap };
use petgraph::algo::tarjan_scc;

#[derive(Parser)]
#[grammar = "day23.pest"]
struct Day23Parser;

type Blah = UnGraph<String, ()>;
type NodeMap = HashMap<String, NodeIndex>;

fn node_idx (m : &NodeMap, s: &str) -> NodeIndex {
    *m.get(s).unwrap()
}

fn cycles3(gr: &Blah, start: NodeIndex) -> Vec<Vec<NodeIndex>> {

    let mut fifo = VecDeque::default();
    fifo.push_back((start, vec![start], 0));
    let mut cycles = Vec::default();

    while let Some((n, stack, depth)) = fifo.pop_front() {
        match depth {
            0..=1 => {
                for next in gr.neighbors(n) {
                    if !stack.contains(&next) {
                        let mut new_stack = stack.clone();
                        new_stack.push(next);
                        fifo.push_back((next, new_stack, depth + 1));
                    }
                }
            }
            2 => if gr.neighbors(n).any(|nn| nn == start) {
                cycles.push(stack);
            }
            _ => unreachable!()
        }
    }
    cycles
}

fn parse_input() -> (i32, String) {
    let data = fs::read_to_string("input/day23.txt").expect("Diable to read file");
    let top = Day23Parser::parse(Rule::top, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut gr : Blah = Blah::default();
    let mut edges = Vec::default();
    let mut nodes = HashSet::default();
    for elem in top.into_inner() {
        match elem.as_rule() {
            Rule::pair => {
                let [n1, n2] = elem.into_inner().map(|n| n.as_str()).collect::<Vec<_>>()[..] else {panic!("wtf");};
                edges.push((n1,n2));
                nodes.insert(n1);
                nodes.insert(n2);
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }

    let mut node_map : NodeMap = HashMap::default();
    for n in nodes.iter() {
        node_map.insert(n.to_string(), gr.add_node(n.to_string()));
    }

    for p in edges.iter() {
        gr.add_edge(node_idx(&node_map, p.0), node_idx(&node_map, p.1), ());
    }

    let mut cycles = gr.node_indices().flat_map(|n| cycles3(&gr, n)).collect::<Vec<_>>();
    cycles.iter_mut().for_each(|c| c.sort());
    cycles.sort();
    cycles.dedup();
    let mut contains_t = 0;
    for c in cycles.iter() {
        if c.iter().any(|s| gr[*s].chars().next().unwrap() == 't') {
            contains_t += 1;
        }
    }

    let mut scc = maximal_cliques(&gr);
    scc.sort_by(|a,b| a.len().cmp(&b.len()));
    let max = scc.last().unwrap();
    let mut part2 = max.iter().map(|n| gr[*n].clone()).collect::<Vec<_>>();
    part2.sort();

    (contains_t,part2.join(","))


}

pub fn day23() -> (i32, String) {
    parse_input()
}

use petgraph::visit::{GetAdjacencyMatrix, IntoNeighbors, IntoNodeIdentifiers};
use std::hash::Hash;
use std::iter::FromIterator;

/// Finds maximal cliques containing all the vertices in r, some of the
/// vertices in p, and none of the vertices in x.
fn bron_kerbosch_pivot<G>(
    g: G,
    adj_mat: &G::AdjMatrix,
    r: HashSet<G::NodeId>,
    mut p: HashSet<G::NodeId>,
    mut x: HashSet<G::NodeId>,
) -> Vec<HashSet<G::NodeId>>
where
    G: GetAdjacencyMatrix + IntoNeighbors,
    G::NodeId: Eq + Hash,
{
    let mut cliques = Vec::with_capacity(1);
    if p.is_empty() {
        if x.is_empty() {
            cliques.push(r);
        }
        return cliques;
    }
    // pick the pivot u to be the vertex with max degree
    let u = p.iter().max_by_key(|&v| g.neighbors(*v).count()).unwrap();
    let mut todo = p
        .iter()
        .filter(|&v| *u == *v || !g.is_adjacent(adj_mat, *u, *v) || !g.is_adjacent(adj_mat, *v, *u)) //skip neighbors of pivot
        .cloned()
        .collect::<Vec<G::NodeId>>();
    while let Some(v) = todo.pop() {
        let neighbors = HashSet::from_iter(g.neighbors(v));
        p.remove(&v);
        let mut next_r = r.clone();
        next_r.insert(v);

        let next_p = p
            .intersection(&neighbors)
            .cloned()
            .collect::<HashSet<G::NodeId>>();
        let next_x = x
            .intersection(&neighbors)
            .cloned()
            .collect::<HashSet<G::NodeId>>();

        cliques.extend(bron_kerbosch_pivot(g, adj_mat, next_r, next_p, next_x));

        x.insert(v);
    }

    cliques
}

pub fn maximal_cliques<G>(g: G) -> Vec<HashSet<G::NodeId>>
where
    G: GetAdjacencyMatrix + IntoNodeIdentifiers + IntoNeighbors,
    G::NodeId: Eq + Hash,
{
    let adj_mat = g.adjacency_matrix();
    let r = HashSet::default();
    let p = g.node_identifiers().collect::<HashSet<G::NodeId>>();
    let x = HashSet::default();
    return bron_kerbosch_pivot(g, &adj_mat, r, p, x);
}

