use serde_json;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::f64;
use std::fs::File;

use itertools::{izip, Itertools, MinMaxResult};
use serde_derive::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
enum ItemVal {
    Single(f64),
    List(Vec<f64>),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum NameVal {
    Single(String),
    List(Vec<String>),
}

#[derive(Deserialize)]
struct ItemEntry(ItemVal, Option<NameVal>);

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
    // Try to mitigate some float errors by substracting 0.00001
    (x * (1.0 / ROUND_TO) - 0.00001).ceil() / (1.0 / ROUND_TO)
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

    let paid_sum: f64 = pay_rounded.iter().sum();
    println!("Sum: {:.2}", paid_sum);
    println!("Remainder: {:.2}", sum_after - paid_sum);
}

fn items_to_hmap(items: &[ItemEntry]) -> HashMap<String, ItemList> {
    let mut dataset: HashMap<String, ItemList> = HashMap::new();

    let names = items.iter().filter_map(|ItemEntry(_, n)| n.as_ref());

    for name in names {
        match name {
            NameVal::Single(n) => {
                dataset.entry(n.to_owned()).or_insert_with(ItemList::new);
            }
            NameVal::List(ns) => {
                for n in ns {
                    dataset.entry(n.to_owned()).or_insert_with(ItemList::new);
                }
            }
        }
    }

    // Distribute items
    for &ItemEntry(ref vs, ref n) in items {
        if let Some(names) = n {
            let mut tmp = vec![];
            let names = match names {
                NameVal::Single(n) => {
                    tmp.push(n.clone());
                    &tmp
                }
                NameVal::List(ns) => ns,
            };

            match vs {
                ItemVal::Single(v) => {
                    let v_div = v / names.len() as f64;

                    for n in names {
                        dataset.get_mut(n).unwrap().add(v_div);
                    }
                }
                ItemVal::List(vs) => {
                    for v in vs {
                        let v_div = v / names.len() as f64;

                        for n in names {
                            dataset.get_mut(n).unwrap().add(v_div);
                        }
                    }
                }
            }
        } else {
            match vs {
                ItemVal::Single(v) => {
                    let v_div = v / dataset.len() as f64;

                    for e in dataset.values_mut() {
                        e.add(v_div);
                    }
                }
                ItemVal::List(vs) => {
                    for v in vs {
                        let v_div = v / dataset.len() as f64;

                        for e in dataset.values_mut() {
                            e.add(v_div);
                        }
                    }
                }
            }
        }
    }

    dataset
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().nth(1).expect("No argument given");
    let file = File::open(filename)?;
    let items: Vec<ItemEntry> = serde_json::from_reader(file)?;
    let dataset = items_to_hmap(&items);

    yscalc(&dataset);

    Ok(())
}
