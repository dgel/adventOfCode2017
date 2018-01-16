use std::io::{self, Read};
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
extern crate combine;
use combine::*;
use combine::char::{char, digit, letter, spaces, string};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Balance {
    Balanced,
    Unbalanced,
}

#[derive(Debug)]
struct NodeData {
    parent: Option<String>,
    children: Vec<String>,
    weight: i32,
    total_weight: i32,
    balance: Balance,
}

impl NodeData {
    fn from_weight(w: i32) -> NodeData {
        NodeData {
            parent: None,
            children: Vec::new(),
            weight: w,
            total_weight: 0,
            balance: Balance::Balanced,
        }
    }

    fn from_weight_children(w: i32, c: Vec<String>) -> NodeData {
        NodeData {
            parent: None,
            children: c,
            weight: w,
            total_weight: 0,
            balance: Balance::Balanced,
        }
    }
}

fn construct_graph(nodes: Vec<(String, i32, Option<Vec<String>>)>) -> BTreeMap<String, NodeData> {
    let mut graph: BTreeMap<String, NodeData> = BTreeMap::new();
    for node in nodes.into_iter() {
        match node.2 {
            Some(children) => {
                for child in children.iter() {
                    let mut entry = graph.entry(child.clone()).or_insert(NodeData::from_weight(0));
                    entry.parent = Some(node.0.clone());
                }
                match graph.entry(node.0) {
                    Entry::Vacant(v) => {
                        v.insert(NodeData::from_weight_children(node.1, children));
                    }
                    Entry::Occupied(mut o) => {
                        let entry = o.get_mut();
                        entry.weight = node.1;
                        entry.children = children;
                    }
                }
            }
            None => {
                let entry = graph.entry(node.0).or_insert(NodeData::from_weight(node.1));
                entry.weight = node.1;
            }
        }
    }
    graph
}

fn parse_graph(s: &str) -> Option<BTreeMap<String, NodeData>> {
    let ident = many1(letter()).skip(spaces());
    let num = between(char('(').skip(spaces()), char(')'), many1(digit()))
        .skip(spaces())
        .map(|v: String| v.parse::<i32>().unwrap());
    let edges = (string("->").skip(spaces()),
                 sep_by1(ident.clone(), (char(','), spaces())).skip(spaces()))
        .map(|(_, v)| v);
    let node = (ident, num.skip(spaces()), optional(edges));
    let mut nodes = (spaces(), many1(node).skip(spaces()), eof())
        .map(|(_, nodes, _)| construct_graph(nodes));
    match nodes.parse(State::new(s)) {
        Ok((graph, _)) => Some(graph),
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

fn find_root(g: &BTreeMap<String, NodeData>) -> Option<&str> {
    for (key, value) in g {
        if value.parent.is_none() {
            return Some(key);
        }
    }
    None
}


fn weigh_tree(label: &str, graph: &mut BTreeMap<String, NodeData>) -> (i32, Balance) {
    let mut total_weight = 0;
    let mut balance = Balance::Balanced;
    let mut children = None;
    if let Some(node) = graph.get(label) {
        children = Some(node.children.clone());
        total_weight += node.weight;
    }
    if let Some(c) = children {
        let mut prev_weight = None;
        for child in c.iter() {
            let (child_weight, child_balance) = weigh_tree(child, graph);
            total_weight += child_weight;
            if child_balance == Balance::Unbalanced {
                balance = Balance::Unbalanced;
            } else if let Some(weight) = prev_weight {
                if weight != child_weight {
                    balance = Balance::Unbalanced;
                }
            }
            prev_weight = Some(child_weight);
        }
    }
    if let Some(node) = graph.get_mut(label) {
        node.total_weight = total_weight;
        node.balance = balance;
    }
    (total_weight, balance)
}

fn balance_tree<'a>(label: &'a str,
                    graph: &'a mut BTreeMap<String, NodeData>)
                    -> Option<(&'a str, i32)> {
    weigh_tree(label, graph);
    if let Some(node) = graph.get(label) {
        if node.balance == Balance::Unbalanced {
            balance_tree_helper(node, graph, None)
        } else {
            None
        }
    } else {
        println!("graph label could not be found in graph");
        None
    }
}

fn balance_tree_helper<'a>(node: &'a NodeData,
                           graph: &'a BTreeMap<String, NodeData>,
                           desired_weight: Option<i32>)
                           -> Option<(&'a str, i32)> {
    let mut unbalanced_node = None;
    let mut child_weight = None;
    // find the unbalanced node
    // and a target weight for a child node
    for child in node.children.iter() {
        if let Some(child_node) = graph.get(child) {
            if child_node.balance == Balance::Unbalanced {
                unbalanced_node = Some(child_node);
            } else {
                child_weight = Some(child_node.total_weight);
            }
        } else {
            println!("graph label could not be found in graph");
        }
    }
    if let Some(child_node) = unbalanced_node {
        // if there is a child node but it had no siblings,
        // we calculate the desired weight if available
        if child_weight.is_none() {
            child_weight = desired_weight;
            child_weight.map(|x| x - node.weight);
        }
        balance_tree_helper(child_node, graph, child_weight)
    } else {
        // the current node is unbalanced, but none of the children are
        // this means the one of the children is erroneous
        if let Some(mut weight) = desired_weight {
            weight -= node.weight;
            weight /= node.children.len() as i32;
            for child in node.children.iter() {
                if let Some(child_node) = graph.get(child) {
                    if child_node.total_weight != weight {
                        return Some((child,
                                     weight - (child_node.total_weight - child_node.weight)));
                    }
                }
            }
            // if none of the children differ from the desired weight, the tree was already
            // balanced
            None
        } else {
            if node.children.len() > 2 {
                let mut weights: Vec<(i32, i32, &'a str)> = Vec::new();
                // find total and local weights of children and their label
                for child in node.children.iter() {
                    if let Some(child_node) = graph.get(child) {
                        weights.push((child_node.total_weight, child_node.weight, child));
                    }
                }
                // sort by total weight
                weights.sort_by_key(|&(tw, _, _)| tw);
                let ln = weights.len();
                let calc = |(tw, w, label), (tw_correct, _, _)| (label, tw_correct - (tw - w));
                // either the first or the last weight is incorrect
                if weights[0].0 != weights[1].0 {
                    Some(calc(weights[0], weights[1]))
                } else if weights[ln - 1].0 != weights[ln - 2].0 {
                    Some(calc(weights[ln - 1], weights[ln - 2]))
                } else {
                    // the tree is balanced
                    None
                }
            } else {
                // this node must have 2 children and the tree is ambiguous
                None
            }
        }
    }
}

fn main() {
    let mut data = String::new();
    if let Ok(_) = io::stdin().read_to_string(&mut data) {
        if let Some(mut g) = parse_graph(&data) {
            if let Some(root) = find_root(&g).map(|s| s.to_owned()) {
                println!("root of graph: {}", root);
                match balance_tree(&root, &mut g) {
                    None => println!("We didn't manage to balance the tree"),
                    Some((label, val)) => {
                        println!("The value of the unbalanced node '{}' should be: {}",
                                 label,
                                 val)
                    }
                }
            }
        }
    }
}
