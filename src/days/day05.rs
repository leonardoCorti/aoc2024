use std::collections::HashMap;

use super::Day;

pub struct Day05;

impl Day for Day05 {
    fn part1(&self, input: &str) -> String {
        let mut parts = input.split("\n\n");
        let input_rules = parts.next().unwrap();
        let input_pages = parts.next().unwrap();
        let mut rules: HashMap<u32,Vec<u32>> = HashMap::new();

        for line in input_rules.lines() {
            let mut parts = line.split("|");
            let first_number: u32 = parts.next().unwrap().parse().unwrap();
            let second_number: u32 = parts.next().unwrap().parse().unwrap();
            match rules.get_mut(&first_number) {
                Some(vector) => {
                    vector.push(second_number);
                },
                None => {
                    rules.insert(first_number, vec![second_number]);
                },
            }
        }
        
        let result: i32 = input_pages.lines()
            .filter(|l| is_correct_print_order(&l,&rules))
            .map(find_middle_page_number)
            .sum();

        return result.to_string();
    }

    fn part2(&self, input: &str) -> String {
        let mut parts = input.split("\n\n");
        let input_rules = parts.next().unwrap();
        let input_pages = parts.next().unwrap();
        let mut rules: HashMap<u32,Vec<u32>> = HashMap::new();

        for line in input_rules.lines() {
            let mut parts = line.split("|");
            let first_number: u32 = parts.next().unwrap().parse().unwrap();
            let second_number: u32 = parts.next().unwrap().parse().unwrap();
            match rules.get_mut(&first_number) {
                Some(vector) => {
                    vector.push(second_number);
                },
                None => {
                    rules.insert(first_number, vec![second_number]);
                },
            }
        }

        let updates: Vec<Vec<u32>> = input_pages.lines()
            .map(|l| l.split(",")
                .map(|e| e.parse::<u32>().unwrap())
                .collect())
            .collect();
        // println!("rules are {rules:?}");
        // println!("updates are {updates:?}");

        let mut accumulator = 0;

        for mut update in updates {
            // println!("checking {update:?}");
            if is_correct_print_order_vec(&update, &rules) {
                // println!("is correct order, passing on");
                continue;
            }
            let mut limit =0;
            while !is_correct_print_order_vec(&update, &rules) {
                // println!("reordering {update:?}");
                reorder(&mut update, &rules);
                // println!("reordered {update:?}");
                // break;
                limit +=1;
                if limit == 10 {
                    // break;
                }
            }
            // println!("finished reordering");

            accumulator += update[update.len()/2];
            // println!("accumulator is now {accumulator}");
        }
        return accumulator.to_string();
    }
}

fn reorder(update: &mut Vec<u32>, rules: &HashMap<u32, Vec<u32>>) {
    // println!("\t reordering!");
    for i in (0..update.len()).rev() {
        // println!("searching the {i}, it is {:?}", update[i]);
        if let Some(rule) = rules.get(&update[i]) {
            // println!("found rule {:?}", rule);
            for j in 0..i {
                if rule.contains(&update[j]) {
                    // println!("\t FOUND IT at {j}, {} and {} will be replaced", update[j], update[i]);
                    let number = update[i];
                    update.remove(i);
                    update.insert(j, number);
                    return;
                }
            }
        }
    }
    // println!("\t finished reordering!");
}

fn is_correct_print_order_vec(l: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    let reverse_l: Vec<u32> = l.iter().rev().map(|e| *e).collect();
    for (i,page) in reverse_l.iter().enumerate() {
        if let Some(rule) = rules.get(page){
            for r in rule {
                if reverse_l[i..].contains(r) {
                    return false;
                }
            }
        }
    }
    return true;
}

fn find_middle_page_number(input: &str) -> i32 {
    let list: Vec<&str> = input.split(",").collect();
    let lenght = list.len();
    return list[lenght/2].parse().unwrap();
}

fn is_correct_print_order(l: &str, rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut list: Vec<&str> = l.split(",").collect();
    list.reverse();

    for (i,page) in list.iter().enumerate() {
        // println!("inspecting {page:?} at index {i:?}");
        if let Some(rule) = rules.get(&page.parse::<u32>().unwrap()) {
            // println!("found some rule: {rule:?}");
            // println!("searching in the slice {:?}",&list[i..]);
            for r in rule {
                let var_name = r.to_string();
                if list[i..].contains(&var_name.as_str()) {
                    return false;
                }
            }

        }        
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_test() {
        let day = Day05;
        assert_eq!(day.part1(INPUT), "143");
    }

    #[test]
    fn part2_test() {
        let day = Day05;
        assert_eq!(day.part2(INPUT), "123");
    }
}
