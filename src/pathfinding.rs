use bevy::prelude::*;
use priority_queue::PriorityQueue;

use crate::grid::{GridPosition, SelectedPath, SelectedTile, Tile};
use crate::states::TurnPhase;
use crate::turns::ActiveUnit;
use crate::units::Unit;

use std::cmp::Reverse;
use std::collections::HashMap;

const EDGE_COST: i32 = 1;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OpenSet>()
            .init_resource::<ClosedSet>()
            .init_resource::<CurrentCosts>()
            .add_system_set(SystemSet::on_enter(TurnPhase::DoMove).with_system(a_star_setup))
            .add_system_set(
                SystemSet::on_enter(TurnPhase::DoMove)
                    .with_system(a_star_initializer.after(a_star_setup)),
            );
    }
}

#[derive(Default)]
struct OpenSet(PriorityQueue<(i32, i32), Reverse<i32>>);

#[derive(Default, Debug)]
struct ClosedSet(HashMap<(i32, i32), Option<(i32, i32)>>);

#[derive(Default)]
struct CurrentCosts(HashMap<(i32, i32), i32>);

fn a_star_initializer(
    units: Query<(Entity, &GridPosition), With<Unit>>,
    tiles: Query<(&GridPosition, &Tile)>,
    mut open_set: ResMut<OpenSet>,
    mut closed_set: ResMut<ClosedSet>,
    mut current_costs: ResMut<CurrentCosts>,
    mut selected_path: ResMut<SelectedPath>,
    selected_tile: Res<SelectedTile>,
    active: ResMut<ActiveUnit>,
) {
    let active = active.as_ref();
    if let Some((_e, grid)) = units.into_iter().find(|(e, _g)| e.id() == active.value) {
        open_set.0.clear();
        closed_set.0.clear();
        current_costs.0.clear();
        let unit_position = (grid.x, grid.y);
        open_set.0.push(unit_position, Reverse(0));
        closed_set.0.insert(unit_position, None);
        current_costs.0.insert(unit_position, 0);

        create_path(
            tiles,
            &mut open_set,
            &mut closed_set,
            &mut current_costs,
            &mut selected_path,
            unit_position,
            &selected_tile,
        )
    }
}

fn create_path(
    tiles: Query<(&GridPosition, &Tile)>,
    open_set: &mut ResMut<OpenSet>,
    closed_set: &mut ResMut<ClosedSet>,
    current_costs: &mut ResMut<CurrentCosts>,
    selected_path: &mut ResMut<SelectedPath>,
    source: (i32, i32),
    selected_tile: &Res<SelectedTile>,
) {
    while !open_set.0.is_empty() {
        let current = open_set.0.pop().unwrap().0;

        let (goal_x, goal_y) = (selected_tile.x, selected_tile.y);
        if current == (goal_x, goal_y) {
            select_path(closed_set, source, selected_path, current);
            open_set.0.clear();
            break;
        }

        for (x, y) in adjacents(current) {
            let new_cost = current_costs.0[&current] + EDGE_COST;
            if let Some((_grid, _tile)) = tiles
                .into_iter()
                .find(|(g, t)| g.x == x && g.y == y && !t.blocked)
            {
                if !current_costs.0.contains_key(&(x, y)) || new_cost < current_costs.0[&(x, y)] {
                    current_costs.0.insert((x, y), new_cost);
                    let priority = new_cost + heuristic((goal_x, goal_y), (x, y));
                    open_set.0.push((x, y), Reverse(priority));
                    closed_set.0.insert((x, y), Some(current));
                }
            }
        }
    }
}

fn select_path(
    came_from: &mut ResMut<ClosedSet>,
    source: (i32, i32),
    selected_path: &mut ResMut<SelectedPath>,
    goal: (i32, i32),
) {
    let mut current_tile = goal;
    selected_path.tiles.clear();

    while current_tile != source {
        selected_path.tiles.push(current_tile);

        if came_from.0[&current_tile] == None {
            println!("Did not find path");
            break;
        }

        current_tile = came_from.0[&current_tile].unwrap();
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
    // (((goal.0 - next_step.0).abs() + (goal.1 - next_step.1).abs()) as f32).sqrt() as i32
    // Change heurisitc specific for grid
    (goal.0 - next_step.0).abs() + (goal.1 - next_step.1).abs()
}

fn a_star_setup(
    mut frontier: ResMut<OpenSet>,
    mut came_from: ResMut<ClosedSet>,
    mut current_costs: ResMut<CurrentCosts>,
) {
    frontier.0.push((0, 0), Reverse(0));
    came_from.0.insert((0, 0), None);
    current_costs.0.insert((0, 0), 0);
}
