
use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Entry::{Occupied, Vacant};

use minheap::MinHeap;
use arch::{Location, Grid, Droplet, Architecture};


type Path = Vec<Location>;

fn build_path(mut came_from: HashMap<Node, Node>, end_node: Node) -> Path {
    let mut path = Vec::new();
    let mut current = end_node;
    while let Some(prev) = came_from.remove(&current) {
        path.push(current.location);
        current = prev;
    }
    path.push(current.location);
    path.reverse();
    path
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct Node {
    location: Location,
    time: u32,
}

type Cost = u32;

impl Node {
    fn expand(&self, grid: &Grid) -> Vec<(Cost, Node)> {
        let mut vec: Vec<(u32, Node)> = grid.neighbors(&self.location)
            .iter()
            .map(|&location| {
                (1,
                 Node {
                    location: location,
                    time: self.time + 1,
                })
            })
            .collect();

        vec.push((100,
                  Node {
            location: self.location,
            time: self.time + 1,
        }));

        vec
    }
}


pub fn route_one(droplet: &Droplet, grid: &Grid) -> Option<Path> {
    let mut todo: MinHeap<Cost, Node> = MinHeap::new();
    let mut best_so_far: HashMap<Node, Cost> = HashMap::new();
    let mut came_from: HashMap<Node, Node> = HashMap::new();
    // TODO remove done in favor of came_from
    let mut done: HashSet<Node> = HashSet::new();

    let start_node = Node {
        location: droplet.location,
        time: 0,
    };
    todo.push(0, start_node);
    best_so_far.insert(start_node, 0);

    // use manhattan distance from goal as the heuristic
    let heuristic = |node: Node| -> u32 { droplet.destination.distance_to(&node.location) };

    while let Some((est_cost, node)) = todo.pop() {

        if node.location == droplet.destination {
            let path = build_path(came_from, node);
            return Some(path);
        }

        // insert returns false if value was already there
        if !done.insert(node) {
            continue;
        }

        // node must be in best_so_far because it was inserted when we put it in
        // the minheap
        let node_cost: Cost = *best_so_far.get(&node).unwrap();

        for (edge_cost, next) in node.expand(&grid) {

            if done.contains(&next) {
                continue;
            }

            let mut next_cost = node_cost + edge_cost;

            match best_so_far.entry(next) {
                Occupied(entry) => {
                    let old_cost = *entry.get();
                    if next_cost < old_cost {
                        *entry.into_mut() = next_cost;
                        came_from.insert(next, node);
                    } else {
                        next_cost = old_cost;
                    }
                }
                Vacant(entry) => {
                    entry.insert(next_cost);
                    came_from.insert(next, node);
                }
            };

            let next_cost_est = next_cost + heuristic(next);
            todo.push(next_cost_est, next)
        }

    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    use proptest::prelude::*;
    use proptest::sample::select;
    use proptest::collection::vec;
    use proptest::option::weighted;

    use arch::tests::{arb_grid, arb_location};

    fn locs_from_grid(grid: Grid) -> BoxedStrategy<(Grid, Location, Location)> {
        let locs: Vec<Location> = grid.locations_with_cells()
            .map(|(loc, _)| loc)
            .collect();
        let strat = (Just(grid), select(locs.clone()), select(locs));
        strat.boxed()
    }


    /// assumes grid is routable
    fn check_contiguous_route(grid: Grid, start: Location, end: Location) {
        let droplet = Droplet {
            location: start,
            destination: Some(end),
        };

        let path = route_one(&droplet, &grid).unwrap();

        for win in path.windows(2) {
            assert!(win[0].distance_to(&win[1]) == 1)
        }
    }

    proptest! {

        #[test]
        fn route_on_connected(ref input in arb_grid(5, 10, 0.95)
                                           .prop_flat_map(locs_from_grid)) {

            let (grid, start, end) = input.clone();

            prop_assume!(grid.is_connected());
            check_contiguous_route(grid.clone(), start, end)

        }
    }
}
