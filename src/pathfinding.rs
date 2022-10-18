use bevy::prelude::*;
use priority_queue::PriorityQueue;

use crate::grid::{BlockedTiles, GridPosition, SelectedPath, SelectedTile};
use crate::states::TurnPhase;
use crate::units::{ActiveUnit, Unit};

use std::cmp::Reverse;
use std::collections::HashMap;

const EDGE_COST: i32 = 1;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AllUnitsActed>()
            .add_system_set(SystemSet::on_enter(TurnPhase::DoMove).with_system(a_star_initializer))
            .add_system_set(SystemSet::on_enter(TurnPhase::AIDoMove))
            .add_system_set(
                SystemSet::on_enter(TurnPhase::AIDoMove).with_system(a_star_initializer),
            );
    }
}

#[derive(Default)]
pub struct AllUnitsActed {
    pub value: bool,
}

pub fn calculate_a_star_path(
    from: (i32, i32),
    to: (i32, i32),
    blocked: &Res<BlockedTiles>,
) -> Vec<(i32, i32)> {
    let mut open_set: PriorityQueue<(i32, i32), Reverse<i32>> = PriorityQueue::new();
    let mut closed_set: HashMap<(i32, i32), Option<(i32, i32)>> = HashMap::new();
    let mut current_costs: HashMap<(i32, i32), i32> = HashMap::new();

    open_set.push(from, Reverse(0));
    closed_set.insert(from, None);
    current_costs.insert(from, 0);

    let mut a_star_path: Vec<(i32, i32)> = Vec::new();
    while !open_set.is_empty() {
        let current = open_set.pop().unwrap().0;

        if current == (to.0, to.1) {
            a_star_path = get_path(closed_set, from, current);
            open_set.clear();
            break;
        }

        for (x, y) in adjacents(current) {
            if let Some(is_blocked) = blocked.0.get(&(x, y)) {
                if !*is_blocked || (x == from.0 && y == from.1) {
                    let new_cost = current_costs[&current] + EDGE_COST;
                    if !current_costs.contains_key(&(x, y)) || new_cost < current_costs[&(x, y)] {
                        current_costs.insert((x, y), new_cost);
                        let priority = new_cost + heuristic((to.0, to.1), (x, y));
                        open_set.push((x, y), Reverse(priority));
                        closed_set.insert((x, y), Some(current));
                    }
                }
            } else {
                continue;
            }
        }
    }
    return a_star_path;
}
fn get_path(
    closed_set: HashMap<(i32, i32), Option<(i32, i32)>>,
    from: (i32, i32),
    to: (i32, i32),
) -> Vec<(i32, i32)> {
    let mut selected_path: Vec<(i32, i32)> = Vec::new();
    selected_path.clear();

    let mut current_tile = to;
    while current_tile != from {
        selected_path.push(current_tile);

        if closed_set[&current_tile] == None {
            println!("Did not find path");
            break;
        }

        current_tile = closed_set[&current_tile].unwrap();
    }
    return selected_path;
}

fn a_star_initializer(
    units: Query<(Entity, &GridPosition), With<Unit>>,
    mut selected_path: ResMut<SelectedPath>,
    selected_tile: Res<SelectedTile>,
    active_res: ResMut<ActiveUnit>,
    blocked: Res<BlockedTiles>,
) {
    match active_res.value {
        Some(active) => match units.get(active) {
            Ok((_e, grid)) => {
                selected_path.tiles = calculate_a_star_path(
                    (grid.x, grid.y),
                    (selected_tile.x, selected_tile.y),
                    &blocked,
                );
            }
            Err(_) => {}
        },
        None => {}
    }
}

fn adjacents(tile: (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (tile.0, tile.1 + 1),
        (tile.0, tile.1 - 1),
        (tile.0 + 1, tile.1),
        (tile.0 - 1, tile.1),
    ]
}

fn heuristic(goal: (i32, i32), next_step: (i32, i32)) -> i32 {
    (goal.0 - next_step.0).abs() + (goal.1 - next_step.1).abs()
}
