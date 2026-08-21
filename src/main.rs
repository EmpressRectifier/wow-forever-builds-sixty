//! Sixty: replay a talent build level by level from 1 to 60.
//!
//! Reads a build file (JSON) and walks the talent tree in order,
//! spending one point per level from 10 to 60.

use std::collections::BTreeMap;
use std::env;
use std::fs;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct Talent {
    id: u32,
    name: String,
    row: u8,
    col: u8,
    req: u8,
    max: u8,
    #[serde(default)]
    prev: u32,
}

#[derive(serde::Deserialize)]
struct Build {
    #[allow(dead_code)]
    class: String,
    #[allow(dead_code)]
    name: String,
    points: BTreeMap<u32, u8>,
}

#[derive(serde::Serialize)]
struct LevelStep {
    level: u8,
    talent: String,
    total: u32,
}

fn replay(build: &Build, mut tree: Vec<Talent>) -> Vec<LevelStep> {
    let mut steps = Vec::new();
    let mut spent: BTreeMap<u32, u8> = BTreeMap::new();
    let mut total: u32 = 0;

    tree.sort_by(|a, b| (a.row, a.col).cmp(&(b.row, b.col)));

    for level in 10u8..=60 {
        for t in &tree {
            let have = *spent.get(&t.id).unwrap_or(&0);
            let want = build.points.get(&t.id).copied().unwrap_or(0);
            if have >= want || t.req > level {
                continue;
            }
            if t.prev != 0 {
                let need = tree.iter().find(|x| x.id == t.prev).map(|x| x.max).unwrap_or(0);
                if *spent.get(&t.prev).unwrap_or(&0) < need {
                    continue;
                }
            }
            spent.insert(t.id, have + 1);
            total += 1;
            steps.push(LevelStep {
                level,
                talent: t.name.clone(),
                total,
            });
            break; // one point per level
        }
    }
    steps
}

fn main() {
    let path = env::args().nth(1).expect("usage: sixty <build.json>");
    let raw = fs::read_to_string(&path).expect("read build file");
    let build: Build = serde_json::from_str(&raw).expect("parse build file");

    // The tree ships with the app; a mage sample is embedded for the demo route.
    let tree_json = r#"[
        {"id":1,"name":"Frostbolt","row":1,"col":1,"req":10,"max":5,"prev":0},
        {"id":2,"name":"Ice Shards","row":1,"col":2,"req":10,"max":5,"prev":0},
        {"id":3,"name":"Frostbite","row":2,"col":1,"req":15,"max":3,"prev":1},
        {"id":4,"name":"Improved Frostbolt","row":2,"col":2,"req":15,"max":2,"prev":1},
        {"id":5,"name":"Elemental Precision","row":3,"col":3,"req":20,"max":3,"prev":2}
    ]"#;
    let tree: Vec<Talent> = serde_json::from_str(tree_json).expect("parse tree");

    let steps = replay(&build, tree);
    println!("{}", serde_json::to_string_pretty(&steps).unwrap());
}
