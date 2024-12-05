use std::{collections::{HashMap, HashSet}, io, usize};

use anyhow::Result;

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut orderings: Vec<String> = vec![];
    let mut updates: Vec<String> = vec![];

    let mut oou = true;
    for line in stdin.lines() {
        let line = line?;

        if line.is_empty() {
            oou = false;
            continue;
        }

        if oou {
            orderings.push(line);
        } else {
            updates.push(line)
        }
    }

    let orderings: Vec<(usize, usize)> = orderings
        .into_iter()
        .map(|s| {
            s.split('|')
                .map(|sp| sp.parse::<usize>().unwrap())
                .collect()
        })
        .map(|i: Vec<usize>| (i[0], i[1]))
        .collect();

    let updates: Vec<Vec<usize>> = updates
        .into_iter()
        .map(|s| {
            s.split(',')
                .map(|sp| sp.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut orderings_map: HashMap<usize, PageOrdering> = HashMap::new();

    for ordering in &orderings {
        let page_before = ordering.0;
        let page_after = ordering.1;

        orderings_map.entry(page_before).or_default().after.insert(page_after);
        orderings_map.entry(page_after).or_default().before.insert(page_before);
    }

    let result: usize = updates.iter().filter(|update| check_update(&update, &orderings_map)).map(|update| update[update.len()/2]).sum();

    println!("{}", result);
    Ok(())
}

#[derive(Default, Debug)]
struct PageOrdering {
    before: HashSet<usize>,
    after: HashSet<usize>,
}

fn check_update(update: &Vec<usize>, orderings_map: &HashMap<usize, PageOrdering>) -> bool {
    let mut seen = HashSet::new();

    for page in update.iter() {
        let Some(page_ordering) = orderings_map.get(page) else {
            return false;
        };

        for value in seen.iter() {
            if page_ordering.after.contains(value) {
                return false;
            }
        }

        seen.insert(*page);
    }

    seen.clear();

    for page in update.iter().rev() {
        let Some(page_ordering) = orderings_map.get(&page) else {
            return false;
        };

        for value in seen.iter() {
            if page_ordering.before.contains(value) {
                return false;
            }
        }

        seen.insert(*page);
    }

    true
}
