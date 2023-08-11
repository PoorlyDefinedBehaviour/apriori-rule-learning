/// Assume that a large supermarket tracks sales data by stock-keeping unit (SKU) for each item:
/// each item, such as "butter" or "bread", is identified by a numerical SKU.
///
/// The supermarket has a database of transactions where each transaction is a set of SKUs that were bought together.
///
/// Let the database of transactions consist of following itemsets:
///
/// Itemsets (Sets of SKU)
/// {1,2,3,4}
/// {1,2,4}
/// {1,2}
/// {2,3,4}
/// {2,3}
/// {3,4}
/// {2,4}
use std::collections::{HashMap, HashSet};

fn apriori(itemsets: &[HashSet<i32>], support_threshold: usize) -> Vec<HashSet<i32>> {
    let items: HashSet<_> = itemsets.iter().flatten().cloned().collect();

    // The first step is to count up the number of occurrences, called the support, of each member item.
    let mut occurrences = HashMap::new();
    for item in items.iter().cloned() {
        let entry = occurrences.entry(item).or_insert(0);
        *entry += 1;
    }

    // TODO: remove itemsets that appeared less than the support?

    let mut n = 2;
    let mut associations = Vec::new();

    loop {
        // Generate every unique combination using the items.
        // TODO: There's no need to generate from the whole dataset since we know some combinations
        // have already been discarded.
        let combinations = gen_combinations(&items, n);

        // Count how many times each pair appears together in the dataset using the support threshold.
        let mut count = vec![0; combinations.len()];
        for (i, combination) in combinations.iter().enumerate() {
            for itemset in itemsets {
                let combination_in_itemset = combination.iter().all(|sku| itemset.contains(sku));
                if combination_in_itemset {
                    count[i] += 1;
                }
            }
        }

        // Keep combinations that appeared at least the support threshold amount of times.
        let combinations: Vec<_> = combinations
            .into_iter()
            .enumerate()
            .filter(|(i, _combination)| count[*i] >= support_threshold)
            .map(|(_, combination)| combination)
            .collect();

        let combinations_len = combinations.len();

        if combinations_len > 0 {
            associations.extend(combinations);
        }

        if combinations_len <= 1 {
            break;
        }

        n += 1;
    }

    associations
}

fn gen_combinations(items: &HashSet<i32>, n: usize) -> Vec<HashSet<i32>> {
    let mut combinations = Vec::new();

    for item in items.iter().cloned() {
        for i in 0..items.len() {
            let mut combination = HashSet::from([item]);

            for other_item in items.iter().skip(i).cloned() {
                if item != other_item {
                    combination.insert(other_item);
                }

                if combination.len() == n && !combinations.contains(&combination) {
                    combinations.push(combination);
                    break;
                }
            }
        }
    }

    combinations
}

fn main() {
    let itemsets = vec![
        HashSet::from([1, 2, 3, 4]),
        HashSet::from([1, 2, 4]),
        HashSet::from([1, 2]),
        HashSet::from([2, 3, 4]),
        HashSet::from([2, 3]),
        HashSet::from([3, 4]),
        HashSet::from([2, 4]),
    ];

    let mut associations = apriori(&itemsets, 3);
    associations.sort_unstable_by_key(|combination| combination.len());
    dbg!(&associations);
}
