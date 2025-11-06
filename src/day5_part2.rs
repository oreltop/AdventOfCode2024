use itertools::{Itertools, Update};
use std::collections::{HashMap, HashSet};
use std::fs;

const FILE_NAME: &'static str = "input_day5.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let (updates, rules) = parse_string(&input);
    let incorrect_updates: Vec<_> = updates
        .iter()
        .filter(|&update| !is_update_correct(&update, &rules))
        .map(|update| correct_update(&update, &rules))
        .collect();
    let result = sum_middles(&incorrect_updates);
    println!("{}", result);
}

fn filter_irrelevant_rules(
    rules_unfiltered: &[(i32, i32)],
    update: &[i32],
) -> Vec<(i32, i32)> {
    rules_unfiltered
        .iter()
        .filter(|(first, later)| update.contains(first) && update.contains(later))
        .cloned()
        .collect()
}

fn parse_string(input: &str) -> (Vec<Vec<i32>>, Vec<(i32, i32)>) {
    let Some((rules_raw, updates_raw)) = input.split_once("\r\n\r\n") else {
        panic!("this shouldn't happen!")
    };
    let updates: Vec<Vec<i32>> = updates_raw
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let rules: Vec<(i32, i32)> = rules_raw
        .lines()
        .map(|l| l.split_once('|').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    (updates, rules)
}

fn does_break_rule(update: &[i32], rule: &(i32, i32)) -> bool {
    let first_pos = update.iter().position(|&item| item == rule.0);
    if first_pos.is_none() {
        return false;
    };

    let later_pos = update.iter().position(|&item| item == rule.1);
    if later_pos.is_none() {
        return false;
    };

    first_pos > later_pos
}

fn is_update_correct(update: &[i32], rules: &[(i32, i32)]) -> bool {
    !rules.iter().any(|rule| does_break_rule(update, rule))
}

fn get_middle(update: &[i32]) -> i32 {
    update[update.len() / 2]
}

fn sum_middles(updates: &Vec<Vec<i32>>) -> i32 {
    updates.iter().map(|u| get_middle(u)).sum()
}

fn create_constraint_graph(rules: &[(i32, i32)]) -> HashMap<&i32, Vec<&i32>> {
    let mut result = HashMap::new();
    for (first, later) in rules {
        result.entry(first).or_insert(vec![]).push(later);
    }
    println!("create_constraint_graph: {:?} ", &result);
    result
}

fn visit<'a>(
    node: &'a i32,
    call_stack: &mut HashSet<&'a i32>,
    visited: &mut HashSet<&'a i32>,
    result: &mut Vec<&'a i32>,
    constraint_graph: &&HashMap<&i32, Vec<&'a i32>>,
) {
    println!(
        "visiting node: {:?}, neighbors: {:?}",
        node,
        constraint_graph.get(node)
    );
    let Some(neighbors) = constraint_graph.get(node) else {
        visited.insert(node);
        call_stack.remove(node);
        result.push(node);
        return;
    };

    for neighbor in neighbors {
        if call_stack.contains(neighbor) {
            panic!(
                "cyclic graph found! aborting.\nDebug data: node: {:?}\nneighbor: {:?}, call_stack: {:?}",
                node, neighbor, call_stack
            )
        }
        if !visited.contains(neighbor) {
            call_stack.insert(neighbor);
            visit(neighbor, call_stack, visited, result, constraint_graph)
        }
    }
    visited.insert(node);
    call_stack.remove(node);
    result.push(node);
}
fn topological_sort<'a>(constraint_graph: &HashMap<&'a i32, Vec<&'a i32>>) -> Vec<&'a i32> {
    let mut call_stack = HashSet::new();
    let mut visited = HashSet::new();
    let mut result = Vec::new();

    for (node, _) in constraint_graph {
        if !visited.contains(node) {
            visit(
                node,
                &mut call_stack,
                &mut visited,
                &mut result,
                &constraint_graph,
            );
        }
    }
    result.reverse();
    result
}

fn correct_update(update: &[i32], rules: &[(i32,i32)]) -> Vec<i32> {
    let rules_filtered = filter_irrelevant_rules(&rules,&update);
    let constraint_graph = create_constraint_graph(&rules_filtered);
    let sorted_constraints = topological_sort(&constraint_graph);

    sorted_constraints
        .iter()
        .filter(|item| update.contains(item))
        .map(|&&i| i)
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn rule_breaking() {
        let rule = (2, 1);
        let update = vec![1, 2, 3, 4, 5];
        assert!(does_break_rule(&update, &rule))
    }
    #[test]
    fn rule_not_breaking() {
        let rule = (1, 2);
        let update = vec![1, 2, 3, 4, 5];
        assert!(!does_break_rule(&update, &rule))
    }
    #[test]
    fn rule_not_apply() {
        let rule = (1, 6);
        let update = vec![1, 2, 3, 4, 5];
        assert!(!does_break_rule(&update, &rule))
    }
    #[test]
    fn rule_not_apply2() {
        let rule = (9, 6);
        let update = vec![1, 2, 3, 4, 5];
        assert!(!does_break_rule(&update, &rule))
    }
    #[test]
    fn rule_not_apply3() {
        let rule = (9, 2);
        let update = vec![1, 2, 3, 4, 5];
        assert!(!does_break_rule(&update, &rule))
    }

    #[test]
    fn doesnt_break_any_rule() {
        let rules = vec![(1, 2), (3, 5), (1, 9), (5, 6), (8, 4)];
        let update = vec![1, 2, 3, 4, 5];
        assert!(is_update_correct(&update, &rules))
    }
    #[test]
    fn break_one_rule() {
        let rules = vec![(1, 2), (5, 2), (1, 9), (5, 6), (8, 4)];
        let update = vec![1, 2, 3, 4, 5];
        assert!(!is_update_correct(&update, &rules))
    }

    #[test]
    fn test_get_middle() {
        let update = vec![1, 2, 3, 4, 5];
        assert_eq!(get_middle(&update), 3);
        let update2 = vec![1, 2, 3];
        assert_eq!(get_middle(&update2), 2);
        let update3 = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(get_middle(&update3), 4);
    }

    #[test]
    fn test_sum_middle() {
        let binding1 = vec![1, 2, 3, 4, 5, 6, 7];
        let binding2 = vec![1, 2, 3];
        let binding3 = vec![1, 2, 3, 4, 5];
        let updates = vec![binding1, binding2, binding3];
        assert_eq!(sum_middles(&updates), 9);
    }

    #[test]
    fn test_create_constraint_graph() {
        let rules = vec![(1, 2), (1, 3), (2, 4), (4, 5), (1, 5)];
        let graph = create_constraint_graph(&rules);
        assert_eq!(graph.get(&1), Some(&vec![&2, &3, &5]));
        assert_eq!(graph.get(&2), Some(&vec![&4]));
        assert_eq!(graph.get(&4), Some(&vec![&5]));
    }

    #[test]
    fn test_top_sort() {
        let rules = vec![(1, 2), (1, 3), (2, 4), (4, 5), (1, 5), (2, 3), (3, 4)];
        let graph = create_constraint_graph(&rules);
        let sorted_graph = topological_sort(&graph);
        assert_eq!(sorted_graph, vec![&1, &2, &3, &4, &5])
    }

    #[test]
    fn fix_incorrect_update() {
        let rules = vec![(1, 2), (1, 3), (2, 4), (4, 5), (1, 5), (2, 3), (3, 4)];
        let graph = create_constraint_graph(&rules);
        let sorted_graph = topological_sort(&graph);

        let incorrect_update = vec![3, 2, 4, 1, 5];
        assert!(!is_update_correct(&incorrect_update, &rules));

        let correct_update = correct_update(&incorrect_update, &rules);
        assert!(is_update_correct(&correct_update, &rules));
    }

    #[test]
    fn test_filter_irrelevant_rules(){
        let rules_unfiltered = vec![(1, 2), (1, 3), (2, 4), (4, 5), (1, 5), (2, 3), (3, 4)];
        let update = vec![1,2,3];
        let rules = filter_irrelevant_rules(&rules_unfiltered, &update);
        assert_eq!(rules, vec![(1, 2), (1, 3), (2, 3)])
    }
}
