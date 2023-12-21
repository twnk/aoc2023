use std::collections::HashMap;

use petgraph::{graph::{Graph, NodeIndex, EdgeReference}, visit::EdgeRef};
use rayon::{prelude::*, str::Lines};
use crate::part_one::{parse_workflow, Workflow, Step};

enum Comparison {
    LessThan(u16),
    GreaterThan(u16)
}

enum Node {
    X(Comparison),
    M(Comparison),
    A(Comparison),
    S(Comparison),
    Accept,
    Reject,
    Start
}

enum Edge {
    FromPass,
    FromFail
}

impl From<&Step<'_>> for Node {
    fn from(value: &Step) -> Self {
        let comp = match value.comp {
            crate::part_one::Comparison::LessThan => Comparison::LessThan(value.val),
            crate::part_one::Comparison::GreaterThan => Comparison::GreaterThan(value.val),
        };
        match value.category {
            crate::part_one::Category::X => Node::X(comp),
            crate::part_one::Category::M => Node::M(comp),
            crate::part_one::Category::A => Node::A(comp),
            crate::part_one::Category::S => Node::S(comp),
        }
    }
}

type FlowLabel<'a> = &'a str;

fn flow_onto_graph(
    workflow: Workflow, 
    entrypoint: NodeIndex, 
    graph: &mut Graph<Node, Edge>,
    entrypoints: &HashMap<FlowLabel, NodeIndex>,
    accept: NodeIndex,
    reject: NodeIndex
) {
    let mut steps = workflow.steps.iter().peekable();
    
    // handle first node (entrypoint) 
    let mut prev_node = {
        // there is always at least one step
        let step = steps.next().unwrap();

        // Create edge for condition met 
        let condition_passed_node = match step.dest {
            crate::part_one::Label::Rejected => reject,
            crate::part_one::Label::Accepted => accept,
            crate::part_one::Label::Flow(lbl) => entrypoints[lbl],
        };
        graph.add_edge(condition_passed_node, entrypoint, Edge::FromPass);

        entrypoint
    };

    // we can only create edges to nodes which are already on the graph
    // therefore at each step, we can only create:
    //
    // - links from its condition met node to itself
    //     (because it exists in entrypoints or is Accept or Reject)
    //  
    // - links the from itself to the previous node
    //     (because we just created that!)
    //
    // Visually: links marked with * are created each iteration
    //
    // (prev) <-Fail- (curr) <-Fail- (next)
    //   ^      *       ^
    //   |              |
    //  Pass          *Pass
    //   |              |
    //  ...            ...
    // 
    for step in steps {
        let node = graph.add_node(Node::from(step));

        // Create edge for prev node, then overwrite
        graph.add_edge(node, prev_node, Edge::FromFail);
        prev_node = node;

        // Create edge for condition met for current node
        let condition_met_node = match step.dest {
            crate::part_one::Label::Rejected => reject,
            crate::part_one::Label::Accepted => accept,
            crate::part_one::Label::Flow(lbl) => entrypoints[lbl],
        };
        graph.add_edge(condition_met_node, node, Edge::FromPass);
    }

    // handle fail condition for last step
    {
        let condition_failed_node = match workflow.dest {
            crate::part_one::Label::Rejected => reject,
            crate::part_one::Label::Accepted => accept,
            crate::part_one::Label::Flow(lbl) => entrypoints[lbl],
        };
        graph.add_edge(condition_failed_node, prev_node, Edge::FromFail);
    }
}

#[derive(Copy, Clone, Debug)]
struct Range {
    min: u16,
    max: u16
}

#[derive(Copy, Clone, Debug)]
struct Bounds {
    x: Range,
    m: Range,
    a: Range,
    s: Range
}

pub fn parse_lines(lines: Lines) -> usize {
    let mut workflows: HashMap<FlowLabel, Workflow> = lines
        .into_par_iter()
        .filter(|l| *l != "" && !l.starts_with("{"))
        .filter_map(|line| {
            let (label, flow) = parse_workflow(line).unwrap().1;
            match label {
                // these won't really happen
                crate::part_one::Label::Rejected => None,
                crate::part_one::Label::Accepted => None,
                // this is the only actual result of parsing
                crate::part_one::Label::Flow(lbl) => Some((lbl, flow)),
            }
        })
        .collect();

    let mut graph = Graph::<Node, Edge>::new();
    let reject = graph.add_node(Node::Reject);
    let accept = graph.add_node(Node::Accept);
    let start = graph.add_node(Node::Start);

    // create a lookup table for the first Node in each Workflow
    let mut entrypoints: HashMap<FlowLabel, NodeIndex> = HashMap::with_capacity(workflows.len());
    for (lbl, flow) in &workflows {
        let entrypoint = Node::from(&flow.steps[0]);
        let index = graph.add_node(entrypoint);
        entrypoints.insert(*lbl, index);
    }

    // special case: the starting workflow ("in")
    let in_flow = workflows.remove("in").unwrap();

    // no more mutations
    let workflows = workflows;
    let entrypoints = entrypoints;

    flow_onto_graph(in_flow, start, &mut graph, &entrypoints, accept, reject);

    for (lbl, workflow) in workflows {
        let entrypoint = entrypoints[lbl];
        flow_onto_graph(workflow, entrypoint, &mut graph, &entrypoints, accept, reject);
    }

    // we now have a graph that points from every Accept condition backwards
    // we can walk it and generate a set of bounds for each route ?

    let mut accumulator: Vec<Bounds> = Vec::with_capacity(graph.edges(accept).count());

    for edge in graph.edges_directed(accept, petgraph::Direction::Outgoing) {
        let bounds = Bounds { 
            x: Range { min: 1, max: 4000 }, 
            m: Range { min: 1, max: 4000 }, 
            a: Range { min: 1, max: 4000 }, 
            s: Range { min: 1, max: 4000 }, 
        };
        walk(&graph, &mut accumulator, bounds, edge);
    }

    println!("{:?}", accumulator);
    
    overlapped_hyperrectangles(accumulator)
    
}

fn reduce_bounds(range: &mut Range, comparison: &Comparison, edge: &Edge) {
    // imagine the nodes are from a<801:vmr,s<2880:zhh
    //
    // we arrived at s<2880 by failing a<801
    // therefore in this pathway, a must be at least 801
    // 
    // or, if we arrived from vmr, we must have passed a<801
    // therefore a is at most 800
    // 
    // now imagine the nodes are from a>2801:pm,x<2760:rs
    // we arrived at x<2760 by failing a>2801
    // therefore a is at most 2801
    //
    // or, if we arrived from pm, we must have passed a>2801
    // therefore a is at least 2802

    match comparison {
        Comparison::LessThan(val) => match edge {
            Edge::FromPass => range.max = range.max.min(*val - 1),
            Edge::FromFail => range.min = range.min.max(*val),
        },
        Comparison::GreaterThan(val) => match edge {
            Edge::FromPass => range.min = range.min.max(*val + 1),
            Edge::FromFail => range.max = range.max.min(*val)
        },
    }
}

fn walk(
    graph: &Graph<Node, Edge>, 
    accumulator: &mut Vec<Bounds>, 
    mut bounds: Bounds, 
    edge: EdgeReference<'_, Edge>
) {
    let weight = edge.weight();

    let from = edge.target();
    match graph.node_weight(from).unwrap() {
        Node::X(cmp) => reduce_bounds(&mut bounds.x, cmp, weight),
        Node::M(cmp) => reduce_bounds(&mut bounds.m, cmp, weight),
        Node::A(cmp) => reduce_bounds(&mut bounds.a, cmp, weight),
        Node::S(cmp) => reduce_bounds(&mut bounds.s, cmp, weight),
        Node::Accept => unreachable!(),
        Node::Reject => unreachable!(),
        Node::Start => return accumulator.push(bounds),
    };

    for edge in graph.edges_directed(from, petgraph::Direction::Outgoing) {
        walk(graph, accumulator, bounds.clone(), edge);
    }
}

struct Point4D {
    x: u16,
    m: u16,
    a: u16,
    s: u16
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
enum Dimension {
    X,
    M,
    A,
    S
}

fn overlapped_hyperrectangles(hyperrects: Vec<Bounds>) -> usize {
    // let (maxima, minima): (Vec<_>, Vec<_>) = hyperrects
    //     .par_iter()
    //     .map(|bounds| (
    //         Point4D { x: bounds.x.min, m: bounds.m.min, a: bounds.a.min, s: bounds.s.min },
    //         Point4D { x: bounds.x.max, m: bounds.m.max, a: bounds.a.max, s: bounds.s.max },
    //     ))
    //     .collect();

    // these are basically like if you extended each edge in each hyperrectangle
    // along the grid lines
    // so if a rectangle had opposite corners (1, 2) and (3, 4) 
    // then it has X edges 1 and 3, and Y edges 2 and 4
    let partitioned_space: Vec<(Dimension, u16)> = hyperrects.iter().flat_map(|bounds| [
        (Dimension::X, bounds.x.min),
        (Dimension::X, bounds.x.max),
        (Dimension::M, bounds.x.min),
        (Dimension::M, bounds.x.max),
        (Dimension::A, bounds.x.min),
        (Dimension::A, bounds.x.max),
        (Dimension::S, bounds.x.min),
        (Dimension::S, bounds.x.max),
    ])
    .collect();

    println!("len space {}", partitioned_space.len());

    let vols = generate_volumes_from_partitioned_space(partitioned_space);

    // println!("{:#?}", vols);

    println!("len vols {}", vols.len());

    for (idx, vol) in vols.iter().enumerate() {
        let num = idx + 1;

        println!(
            "1_s_{}_a_{}_m_{}_x_{}     {}     {} {}",
            vol.id.s.unwrap_or(999),
            vol.id.a.unwrap_or(999),
            vol.id.m.unwrap_or(999),
            vol.id.x.unwrap_or(999),
            vol.min,
            vol.max, 
            match vol.dimension {
                Dimension::X => 'x',
                Dimension::M => 'm',
                Dimension::A => 'a',
                Dimension::S => 's',
            }
        )
    }


    0
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct VolumeIdentifier {
    x: Option<u16>,
    m: Option<u16>,
    a: Option<u16>,
    s: Option<u16>,
}

#[derive(Clone, Copy, Debug)]
struct ExpandedVolume {
    id: VolumeIdentifier,
    min: u16,
    max: u16,
    dimension: Dimension
}

fn generate_volumes_from_partitioned_space(space: Vec<(Dimension, u16)>) -> Vec<ExpandedVolume> {
    // start with the dimension of the first line in the space (arbitrarily)
    let dimension_of_interest = if let Some((d, _)) = space.first() {
        *d
    } else {
        // if there wasn't any data we're done recursing
        return Vec::with_capacity(0);
    };

    let mut bounds_in_dimension: Vec<u16> = space
        .iter()
        .filter_map(|(d, b)| if dimension_of_interest.eq(d) { Some(*b) } else { None })
        .collect();

    bounds_in_dimension.sort();
    bounds_in_dimension.dedup();

    assert!(bounds_in_dimension.len() > 1);

    let partitioned_subspace: Vec<_> = space
        .into_iter()
        .filter(|(d, _)| dimension_of_interest.ne(d))
        .collect();

    let subspace_volumes = generate_volumes_from_partitioned_space(partitioned_subspace);

    let mut expanded_volumes: Vec<ExpandedVolume> = Vec::new();
    // iterating on sorted bounds_in_dimension pairwise
    for idx in 0..(bounds_in_dimension.len() - 1) {
        let lower = bounds_in_dimension[idx];
        let upper = bounds_in_dimension[idx + 1];

        let idx = idx + 1;

        if subspace_volumes.len() == 0 {
            // the deepest depths of the recursion where there are no dimensions left
            let id: VolumeIdentifier = match dimension_of_interest {
                Dimension::X => VolumeIdentifier { x: Some(idx as u16), m: None, a: None, s: None },
                Dimension::M => VolumeIdentifier { x: None, m: Some(idx as u16), a: None, s: None },
                Dimension::A => VolumeIdentifier { x: None, m: None, a: Some(idx as u16), s: None },
                Dimension::S => VolumeIdentifier { x: None, m: None, a: None, s: Some(idx as u16) },
            };

            let vol = ExpandedVolume { id, min: lower, max: upper, dimension: dimension_of_interest };
            expanded_volumes.push(vol);
        } else {
            // somewhere else in the recursion there are some dimensions left
            let mut unique_subspace_volume_ids: Vec<VolumeIdentifier> = subspace_volumes
                .iter()
                .map(|v|v.id.to_owned())
                .collect();

            unique_subspace_volume_ids.sort();
            unique_subspace_volume_ids.dedup();

            for mut id in unique_subspace_volume_ids {
                match dimension_of_interest {
                    Dimension::X => id.x = Some(idx as u16),
                    Dimension::M => id.m = Some(idx as u16),
                    Dimension::A => id.a = Some(idx as u16),
                    Dimension::S => id.s = Some(idx as u16),
                };
                expanded_volumes.push(ExpandedVolume { id, min: lower, max: upper, dimension: dimension_of_interest })
            };

            for v in &subspace_volumes {
                let mut v = v.to_owned();

                match dimension_of_interest {
                    Dimension::X => v.id.x = Some(idx as u16),
                    Dimension::M => v.id.m = Some(idx as u16),
                    Dimension::A => v.id.a = Some(idx as u16),
                    Dimension::S => v.id.s = Some(idx as u16),
                };

                expanded_volumes.push(v);
            };

        };

    }

    expanded_volumes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlapped_hypervolumes() {
        let input = vec![
            Bounds { x: Range { min: 1, max: 3 }, m: Range { min: 1, max: 5 }, a: Range { min: 1, max: 6 }, s: Range { min: 1, max: 7 } },
            Bounds { x: Range { min: 2, max: 5 }, m: Range { min: 2, max: 5 }, a: Range { min: 2, max: 5 }, s: Range { min: 2, max: 5 } },
            Bounds { x: Range { min: 2, max: 4 }, m: Range { min: 3, max: 7 }, a: Range { min: 4, max: 7 }, s: Range { min: 5, max: 7 } },
            ];
        
        let sum = overlapped_hyperrectangles(input);
        assert_eq!(sum, 1234);
    }

    #[test]
    fn test_end_to_end() {
        let input = "
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}
";
        
        let sum = parse_lines(input.par_lines());
        assert_eq!(sum, 167_409_079_868_000);
        
    }
}
