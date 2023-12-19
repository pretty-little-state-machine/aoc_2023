use crate::day19::Action::{Accept, Reject, SendToWorkflow};
use crate::day19::PartCmp::{GreaterThan, LessThan};
use crate::day19::PartKind::{Aerodynamic, ExtremelyCoolLooking, Musical, Shiny};
use crate::DayResult;
use fxhash::FxHashMap;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let (workflows, parts) = parse_input(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&workflows, &parts).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&input).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

#[derive(Debug)]
enum PartKind {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl PartKind {
    fn new(input: &str) -> Self {
        match input.chars().collect::<Vec<char>>().first().unwrap() {
            'x' => ExtremelyCoolLooking,
            'm' => Musical,
            'a' => Aerodynamic,
            's' => Shiny,
            _ => unreachable!("Unknown part kind"),
        }
    }
}

#[derive(Debug)]
enum PartCmp {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone)]
enum Action {
    Accept,
    Reject,
    SendToWorkflow { workflow: String },
}

impl Action {
    fn new(input: &str) -> Self {
        if input.eq("A") {
            Accept
        } else if input.eq("R") {
            Reject
        } else {
            SendToWorkflow {
                workflow: input.to_string(),
            }
        }
    }
}

#[derive(Debug)]
struct RuleComparison {
    part_kind: PartKind,
    part_cmp: PartCmp,
    value: isize,
}

#[derive(Debug)]
struct Rule {
    rule_comparison: Option<RuleComparison>,
    action: Action,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Part {
    extremely_cool_looking: isize,
    musical: isize,
    aerodynamic: isize,
    shiny: isize,
}

impl Part {
    fn value(&self) -> isize {
        self.extremely_cool_looking + self.musical + self.aerodynamic + self.shiny
    }
}

fn parse_parts(input: &str) -> Vec<Part> {
    let mut parts = Vec::default();
    for part_text in input.lines() {
        let part_fields = part_text
            .replace(['{', '}', '=', 's', 'a', 'm', 'x'], "")
            .split(',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        parts.push(Part {
            extremely_cool_looking: *part_fields.first().unwrap(),
            musical: *part_fields.get(1).unwrap(),
            aerodynamic: *part_fields.get(2).unwrap(),
            shiny: *part_fields.get(3).unwrap(),
        });
    }
    parts
}

fn parse_workflows(input: &str) -> FxHashMap<String, Workflow> {
    let mut workflows = FxHashMap::default();
    for line in input.lines() {
        let mut fields = line.split('{');
        let name = fields.next().unwrap().to_string();
        let rules_text = fields.next().unwrap().replace('}', "");

        let mut rules = Vec::default();
        for rule_text in rules_text.split(',') {
            if rule_text.eq("A") {
                rules.push(Rule {
                    rule_comparison: None,
                    action: Accept,
                });
            } else if rule_text.eq("R") {
                rules.push(Rule {
                    rule_comparison: None,
                    action: Reject,
                });
            } else {
                if rule_text.contains('<') {
                    let mut rule_fields = rule_text.split(['<', ':']);
                    rules.push(Rule {
                        rule_comparison: Some(RuleComparison {
                            part_kind: PartKind::new(rule_fields.next().unwrap()),
                            part_cmp: PartCmp::LessThan,
                            value: rule_fields.next().unwrap().parse::<isize>().unwrap(),
                        }),
                        action: Action::new(rule_fields.next().unwrap()),
                    });
                } else if rule_text.contains('>') {
                    let mut rule_fields = rule_text.split(['>', ':']);
                    rules.push(Rule {
                        rule_comparison: Some(RuleComparison {
                            part_kind: PartKind::new(rule_fields.next().unwrap()),
                            part_cmp: PartCmp::GreaterThan,
                            value: rule_fields.next().unwrap().parse::<isize>().unwrap(),
                        }),
                        action: Action::new(rule_fields.next().unwrap()),
                    });
                } else {
                    rules.push(Rule {
                        rule_comparison: None,
                        action: Action::new(&rule_text),
                    });
                }
            }
        }
        workflows.insert(name.clone(), Workflow { name, rules });
    }
    workflows
}

fn parse_input(input: &str) -> (FxHashMap<String, Workflow>, Vec<Part>) {
    let mut components = input.split("\n\n");
    (
        parse_workflows(components.next().unwrap()),
        parse_parts(components.next().unwrap()),
    )
}

impl RuleComparison {
    fn matches(&self, part: &Part) -> bool {
        match (&self.part_kind, &self.part_cmp) {
            (ExtremelyCoolLooking, LessThan) => part.extremely_cool_looking < self.value,
            (ExtremelyCoolLooking, GreaterThan) => part.extremely_cool_looking > self.value,
            (Musical, LessThan) => part.musical < self.value,
            (Musical, GreaterThan) => part.musical > self.value,
            (Aerodynamic, LessThan) => part.aerodynamic < self.value,
            (Aerodynamic, GreaterThan) => part.aerodynamic > self.value,
            (Shiny, LessThan) => part.shiny < self.value,
            (Shiny, GreaterThan) => part.shiny > self.value,
        }
    }
}

impl Workflow {
    fn process_part(&self, part: &Part) -> Action {
        for rule in &self.rules {
            if let Some(comparison) = &rule.rule_comparison {
                if comparison.matches(part) {
                    return rule.action.clone();
                }
            } else {
                return rule.action.clone();
            }
        }
        panic!("Failed to match any rules...something is wrong in the parser.");
    }
}

fn part_1(workflows: &FxHashMap<String, Workflow>, parts: &Vec<Part>) -> isize {
    let mut accepted_parts = Vec::new();

    for part in parts {
        let mut current_workflow = "in".to_string();
        loop {
            let w = workflows
                .get(&current_workflow)
                .expect("Couldn't find workflow!");
            match w.process_part(part) {
                Accept => {
                    accepted_parts.push(part);
                    break;
                }
                Reject => break,
                SendToWorkflow { workflow: target } => {
                    current_workflow = target;
                }
            }
        }
    }
    accepted_parts.iter().map(|p| p.value()).sum()
}

fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}
";
        let (workflows, parts) = parse_input(input);
        assert_eq!(part_1(&workflows, &parts), 19114);
    }

    #[test]
    fn test_part_2() {}
}
