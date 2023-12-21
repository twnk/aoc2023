use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u16},
    combinator::{value, map},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use rayon::{prelude::*, str::Lines};

#[derive(Debug)]
pub struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Label<'a> {
    Rejected,
    Accepted,
    Flow(&'a str)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Comparison {
    LessThan,
    GreaterThan
}

#[derive(PartialEq, Eq, Debug)]
pub struct Step<'a> {
    pub category: Category,
    pub comp: Comparison,
    pub val: u16,
    pub dest: Label<'a>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Workflow<'a> {
    pub steps: Vec<Step<'a>>,
    pub dest: Label<'a>,
}

fn parse_label(input: &str) -> IResult<&str, Label> {
    alt((
        value(Label::Rejected, tag("R")),
        value(Label::Accepted, tag("A")),
        map(alpha1, |flow| Label::Flow(flow))
    ))(input)
}

fn parse_category(input: &str) -> IResult<&str, Category> {
    alt((
        value(Category::X, tag("x")),
        value(Category::M, tag("m")),
        value(Category::A, tag("a")),
        value(Category::S, tag("s")),
    ))(input)
}

fn parse_step(input: &str) -> IResult<&str, Step> {
    let (r, (category, comp, val, dest)) = tuple((
        parse_category,
        alt((value(Comparison::LessThan, tag("<")), value(Comparison::GreaterThan, tag(">")))),
        u16,
        preceded(tag(":"), parse_label),
    ))(input)?;

    Ok((
        r,
        Step {
            category,
            comp,
            val,
            dest,
        },
    ))
}

pub fn parse_workflow(input: &str) -> IResult<&str, (Label, Workflow)> {
    let (r, (lbl, steps, dest)) = tuple((
        parse_label,
        preceded(tag("{"), separated_list1(tag(","), parse_step)),
        delimited(tag(","), parse_label, tag("}")),
    ))(input)?;

    Ok((r, (lbl, Workflow { steps, dest })))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (r, (x, m, a, s)) = tuple((
        preceded(tag("{x="), u16),
        preceded(tag(",m="), u16),
        preceded(tag(",a="), u16),
        delimited(tag(",s="), u16, tag("}")),
    ))(input)?;

    Ok((r, Part { x, m, a, s }))
}

pub fn parse_lines(lines: Lines) -> (HashMap<Label, Workflow>, Vec<Part>) {
    lines.into_par_iter().filter(|l| *l != "").partition_map(|line| {
        if line.starts_with("{") {
            rayon::iter::Either::Right(parse_part(line).unwrap().1)
        } else {
            rayon::iter::Either::Left(parse_workflow(line).unwrap().1)
        }
    })
}

fn process_part<'a>(start: &Workflow<'a>, workflows: &HashMap<Label<'a>, Workflow<'a>>, part: Part) -> usize {
    let mut flow = start;
    let (x, m, a, s) = (part.x, part.m, part.a, part.s);
    // println!("part {:?}", part);
    'flow: loop {
        for step in &flow.steps {
            let rating = match step.category {
                Category::X => x,
                Category::M => m,
                Category::A => a,
                Category::S => s,
            };

            let condition = match step.comp {
                Comparison::LessThan => rating < step.val,
                Comparison::GreaterThan => rating > step.val,
            };

            // println!(
            //     "comparing category {:?}. part val {} is {:?} {}? result: {}",
            //     step.category,
            //     rating,
            //     step.comp,
            //     step.val,
            //     condition
            // );

            if condition { match step.dest {
                Label::Rejected => return 0,
                Label::Accepted => return (x + m + a + s) as usize,
                Label::Flow(_) => {
                    // println!("goto flow {}", lbl);
                    flow = &workflows[&step.dest];
                    continue 'flow;
                },
            } } 
        }
        // println!("default:");
        match flow.dest {
            Label::Rejected => return 0,
            Label::Accepted => return (x + m + a + s) as usize,
            Label::Flow(_) => {
                // println!("goto flow {}", lbl);
                flow = &workflows[&flow.dest];
            },
        }
    }
}

pub fn process_parts(workflows: HashMap<Label, Workflow>, parts: Vec<Part>) -> usize {
    let start = &workflows[&Label::Flow("in")];
    parts
        .into_par_iter()
        .map(|part| process_part(start, &workflows, part))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_workflow() {
        let inputs = [
            (
                "px{a<2006:qkq,m>2090:A,rfg}",
                Workflow { 
                    steps: vec![
                        Step { category: Category::A, comp: Comparison::LessThan, val: 2006, dest: Label::Flow("qkq") },
                        Step { category: Category::M, comp: Comparison::GreaterThan, val: 2090, dest: Label::Accepted }
                    ], 
                    dest: Label::Flow("rfg")
                }
            ),
            (
                "pv{a>1716:R,A}",
                Workflow { 
                    steps: vec![
                        Step { category: Category::A, comp: Comparison::GreaterThan, val: 1716, dest: Label::Rejected }
                    ], 
                    dest: Label::Accepted
                }
        ),
            (
                "lnx{m>1548:A,A}",
                Workflow { 
                    steps: vec![
                        Step { category: Category::M, comp: Comparison::GreaterThan, val: 1548, dest: Label::Accepted }
                    ], 
                    dest: Label::Accepted
                }
            ),
            (
                "rfg{s<537:gd,x>2440:R,A}",
                Workflow { 
                    steps: vec![
                        Step { category: Category::S, comp: Comparison::LessThan, val: 537, dest: Label::Flow("gd") },
                        Step { category: Category::X, comp: Comparison::GreaterThan, val: 2440, dest: Label::Rejected }
                    ], 
                    dest: Label::Accepted
                }
            ),
        ];
        for (input, expected) in inputs {
            let (_, actual) = parse_workflow(input).unwrap().1;
            assert_eq!(actual, expected);
        }
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

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        
        let (workflows, parts) = parse_lines(input.par_lines());
        let actual = process_parts(workflows, parts);
        assert_eq!(actual, 19114);
        
    }
}
