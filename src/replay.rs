//! Walk a talent build level by level and emit the spend order.

use std::collections::BTreeMap;

pub fn walk(points: &BTreeMap<u32, u8>, tree: &[(u32, String, u8, u8, u8, u8)]) -> Vec<(u8, String)> {
    let mut spent: BTreeMap<u32, u8> = BTreeMap::new();
    let mut out = Vec::new();
    for level in 10u8..=60 {
        for (id, name, row, _col, req, _max) in tree {
            let have = *spent.get(id).unwrap_or(&0);
            let want = points.get(id).copied().unwrap_or(0);
            if have >= want || *req > level {
                continue;
            }
            spent.insert(*id, have + 1);
            out.push((level, name.clone()));
            break;
        }
        let _ = row;
    }
    out
}
