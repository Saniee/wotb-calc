use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

use crate::data_types::tank_data_types::Tank;

#[derive(Clone, Debug)]
pub struct ModuleStep {
    pub module_name: String,
    pub xp_cost: i64,
}

#[derive(Clone, Debug)]
pub struct ResearchStep {
    pub tank_name: String,
    pub modules_to_research: Vec<ModuleStep>,
    pub next_tank_name: String,
    pub next_tank_xp: i64,
}

#[derive(Clone, Debug)]
pub struct ResearchPath {
    pub total_xp: i64,
    pub steps: Vec<ResearchStep>,
}

/// Returns the minimum XP and the ordered list of module IDs to research
/// (excluding default/stock modules) to unlock `target_module_id` from the
/// tank's stock state. Returns None if the module is not in this tank's tree.
fn min_module_xp(tank: &Tank, target_module_id: i64) -> Option<(i64, Vec<i64>)> {
    let target_mod = tank.modules_tree.get(&target_module_id.to_string())?;

    if target_mod.is_default {
        return Some((0, vec![]));
    }

    let mut dist: HashMap<i64, i64> = HashMap::new();
    let mut prev: HashMap<i64, i64> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(i64, i64)>> = BinaryHeap::new();

    for module in tank.modules_tree.values() {
        if module.is_default {
            dist.insert(module.module_id, 0);
            heap.push(Reverse((0, module.module_id)));
        }
    }

    while let Some(Reverse((cost, mid))) = heap.pop() {
        if cost > *dist.get(&mid).unwrap_or(&i64::MAX) {
            continue;
        }

        if let Some(module) = tank.modules_tree.get(&mid.to_string()) {
            for &next_id in module.next_modules.iter().flatten() {
                if let Some(next_mod) = tank.modules_tree.get(&next_id.to_string()) {
                    let new_cost = cost + next_mod.price_xp;
                    let entry = dist.entry(next_id).or_insert(i64::MAX);
                    if new_cost < *entry {
                        *entry = new_cost;
                        prev.insert(next_id, mid);
                        heap.push(Reverse((new_cost, next_id)));
                    }
                }
            }
        }
    }

    let &total_xp = dist.get(&target_module_id)?;
    if total_xp == i64::MAX {
        return None;
    }

    // Reconstruct path (non-default modules only; defaults are free starting points)
    let mut path = vec![];
    let mut cur = target_module_id;
    while let Some(&p) = prev.get(&cur) {
        path.push(cur);
        cur = p;
    }
    // `cur` is now a default module (no predecessor)  - include target and all
    // non-default modules along the way, already collected above
    path.reverse();

    Some((total_xp, path))
}

/// Find the minimum-XP research path from `start_id` to `end_id` through the
/// tech tree. Returns None if no path exists (different tech trees, invalid IDs).
pub fn find_research_path(
    tanks: &HashMap<String, Tank>,
    start_id: i64,
    end_id: i64,
) -> Option<ResearchPath> {
    if start_id == end_id {
        return Some(ResearchPath { total_xp: 0, steps: vec![] });
    }

    let mut dist: HashMap<i64, i64> = HashMap::new();
    let mut prev: HashMap<i64, i64> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(i64, i64)>> = BinaryHeap::new();

    dist.insert(start_id, 0);
    heap.push(Reverse((0, start_id)));

    while let Some(Reverse((cost, tid))) = heap.pop() {
        if tid == end_id {
            break;
        }
        if cost > *dist.get(&tid).unwrap_or(&i64::MAX) {
            continue;
        }

        let tank = match tanks.get(&tid.to_string()) {
            Some(t) => t,
            None => continue,
        };

        for module in tank.modules_tree.values() {
            let next_tank_ids = match &module.next_tanks {
                Some(v) => v,
                None => continue,
            };

            let module_xp = match min_module_xp(tank, module.module_id) {
                Some((xp, _)) => xp,
                None => continue,
            };

            for &next_tank_id in next_tank_ids {
                let tank_xp = tank
                    .next_tanks
                    .as_ref()
                    .and_then(|nt| nt.get(&next_tank_id.to_string()).copied())
                    .unwrap_or(0);

                let new_cost = cost + module_xp + tank_xp;
                let entry = dist.entry(next_tank_id).or_insert(i64::MAX);
                if new_cost < *entry {
                    *entry = new_cost;
                    prev.insert(next_tank_id, tid);
                    heap.push(Reverse((new_cost, next_tank_id)));
                }
            }
        }
    }

    let &total_xp = dist.get(&end_id)?;
    if total_xp == i64::MAX {
        return None;
    }

    // Reconstruct the tank path from start to end
    let mut path = vec![end_id];
    let mut cur = end_id;
    while let Some(&p) = prev.get(&cur) {
        path.push(p);
        if p == start_id {
            break;
        }
        cur = p;
    }
    path.reverse();

    if path.first() != Some(&start_id) {
        return None;
    }

    // Build human-readable steps
    let mut steps = Vec::new();
    for w in path.windows(2) {
        let (from_id, to_id) = (w[0], w[1]);
        let from_tank = tanks.get(&from_id.to_string())?;
        let to_tank = tanks.get(&to_id.to_string())?;

        // Find the cheapest gate module that unlocks to_id on from_tank
        let mut best: Option<(i64, Vec<i64>)> = None;
        for module in from_tank.modules_tree.values() {
            if !module.next_tanks.iter().flatten().any(|&id| id == to_id) {
                continue;
            }
            if let Some((xp, path)) = min_module_xp(from_tank, module.module_id) {
                if best.as_ref().map_or(true, |(bxp, _)| xp < *bxp) {
                    best = Some((xp, path));
                }
            }
        }

        let mod_path = best.map(|(_, p)| p).unwrap_or_default();
        let modules_to_research: Vec<ModuleStep> = mod_path
            .iter()
            .filter_map(|&mid| {
                let m = from_tank.modules_tree.get(&mid.to_string())?;
                Some(ModuleStep { module_name: m.name.clone(), xp_cost: m.price_xp })
            })
            .collect();

        let next_tank_xp = from_tank
            .next_tanks
            .as_ref()
            .and_then(|nt| nt.get(&to_id.to_string()).copied())
            .unwrap_or(0);

        steps.push(ResearchStep {
            tank_name: from_tank.name.clone(),
            modules_to_research,
            next_tank_name: to_tank.name.clone(),
            next_tank_xp,
        });
    }

    Some(ResearchPath { total_xp, steps })
}
