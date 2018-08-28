#[macro_use]
extern crate itertools;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use std::env;
use std::f64;
use std::fs::File;
use std::io::BufReader;

use itertools::{Itertools, MinMaxResult};

#[derive(Deserialize)]
struct ItemEntry(f64, Option<Vec<String>>);

#[derive(Deserialize)]
struct ItemList {
    items: Vec<f64>,
}

const ROUND_TO: f64 = 0.25;

impl ItemList {
    pub fn new() -> ItemList {
        ItemList { items: Vec::new() }
    }

    pub fn sum(&self) -> f64 {
        self.items.iter().sum()
    }

    pub fn add(&mut self, v: f64) {
        self.items.push(v);
    }

    pub fn it_str(&self) -> String {
        format!("{:.2}", self.items.iter().format(", "))
    }
}

// 30 -> 20, 40 -> 25, 70 -> 45, 120 -> 75
fn price_cut(tot: f64) -> f64 {
    match tot {
        x if x < 30.0 => 0.0,
        x if x < 40.0 => 30.0 - 20.0,
        x if x < 70.0 => 40.0 - 25.0,
        x if x < 120.0 => 70.0 - 45.0,
        _ => 120.0 - 75.0,
    }
}

fn round_scaled(x: f64) -> f64 {
    (x * (1.0 / ROUND_TO)).round() / (1.0 / ROUND_TO)
}

fn ceil_scaled(x: f64) -> f64 {
    (x * (1.0 / ROUND_TO)).ceil() / (1.0 / ROUND_TO)
}

fn yscalc(dataset: &HashMap<String, ItemList>) {
    let (names, entries): (Vec<_>, Vec<_>) = dataset.iter().unzip();

    let sum_before: f64 = entries.iter().fold(0.0, |acc, x| acc + x.sum());
    let sum_after = sum_before - price_cut(sum_before);
    let ratio = sum_after / sum_before;

    let pay: Vec<f64> = entries.iter().map(|e| e.sum() * ratio).collect();
    let mut pay_rounded: Vec<f64> = pay.iter().map(|c| round_scaled(*c)).collect();
    let rounded_sum: f64 = pay_rounded.iter().sum();

    let mut remainder = ceil_scaled(sum_after) - rounded_sum;

    loop {
        let minmax = izip!(&mut pay_rounded, &pay)
            .map(|(pr, p)| (*pr - *p, pr))
            .minmax_by(|&(a, _), &(b, _)| a.partial_cmp(&b).unwrap());

        if let MinMaxResult::MinMax((_, min), (_, max)) = minmax {
            let tgt = if remainder > 0.0 { min } else { max };

            let adj = if remainder >= ROUND_TO {
                ROUND_TO
            } else if remainder <= -ROUND_TO {
                -ROUND_TO
            } else if remainder > 0.0 {
                remainder
            } else {
                break;
            };

            *tgt += adj;
            remainder -= adj;
        } else {
            panic!("Need more than one input");
        }
    }

    println!("Total: {:.2}", sum_after);
    println!(
        "Ratio = {:.2} / {:.2} = {:.2}\n",
        sum_after, sum_before, ratio
    );

    for (idx, (n, e, r, p)) in izip!(&names, &entries, &pay_rounded, &pay).enumerate() {
        println!("{} {}: {:.2} ({})", idx + 1, n, e.sum(), e.it_str());
        println!("  {:.2} ({:.2})\n", r, p);
    }

    let paid_sum = pay_rounded.iter().sum::<f64>();
    println!("Sum: {:.2}", paid_sum);
    println!("Remainder: {:.2}", sum_after - paid_sum);
}

fn items_to_hmap(items: &[ItemEntry]) -> HashMap<String, ItemList> {
    let mut dataset: HashMap<String, ItemList> = HashMap::new();

    // Init HashMap with unique names
    for &ItemEntry(_, ref n) in items {
        if let Some(ref names) = *n {
            for name in names {
                dataset.entry(name.to_owned()).or_insert_with(ItemList::new);
            }
        }
    }

    // Distribute items
    for &ItemEntry(v, ref n) in items {
        if let Some(ref names) = *n {
            let v_div = v / names.len() as f64;

            for n in names {
                dataset.get_mut(n).unwrap().add(v_div);
            }
        } else {
            let v_div = v / dataset.len() as f64;

            for e in dataset.values_mut() {
                e.add(v_div);
            }
        }
    }

    dataset
}

fn main() {
    let filename = env::args().nth(1).expect("No argument given");

    let items: Vec<ItemEntry> = File::open(filename)
        .map(|f| serde_json::from_reader(BufReader::new(f)).unwrap())
        .unwrap();

    let dataset = items_to_hmap(&items);

    yscalc(&dataset);
}
