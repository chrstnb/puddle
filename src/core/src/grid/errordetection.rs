extern crate pathfinding;

use pathfinding::kuhn_munkres::*;
use pathfinding::matrix::*;

use grid::parse::Blob;
use super::{Droplet, DropletId};

use std::collections::{HashMap};

pub fn match_views(
    exec_view: HashMap<DropletId, Droplet>,
    chip_view: Vec<Blob>,
) -> HashMap<DropletId, Blob> {
    if exec_view.len() != chip_view.len() {
        panic!("Expected and actual droplets are of different lengths");
    }
    let mut hm = HashMap::new();
    let mut ids = vec![];
    let mut matches = vec![];
    for (id, droplet) in &exec_view {
        ids.push(id);
        for blob in chip_view.clone().into_iter() {
            matches.push(get_similarity(&blob, droplet));
        }
    }
    let m: Matrix<i32> = Matrix::from_vec(ids.len(), ids.len(), matches);
    let (_c, result) = kuhn_munkres_min(&m);
    for i in 0..result.len() {
        hm.insert(*ids[i], chip_view[result[i]].clone());
    }
    hm
}

pub fn get_similarity(blob: &Blob, droplet: &Droplet) -> i32 {
    blob.location.distance_to(&droplet.location) as i32
    + blob.dimensions.distance_to(&droplet.dimensions) as i32
    + ((blob.volume - droplet.volume) as i32).abs()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use super::super::parse;

    fn blob_map_to_droplet_map(blobs: HashMap<char, Blob>) -> (HashMap<DropletId, Droplet>, Vec<char>) {
        let mut droplet_vec: HashMap<DropletId, Droplet> = HashMap::new();
        let mut char_to_id: Vec<char> = vec![];

        for c in blobs.keys() {
            let droplet_id =
                DropletId {
                    id: char_to_id.len(),
                    process_id: 0,
            };
            println!("{:?}", droplet_id);
            droplet_vec.insert(droplet_id, Droplet::new(
                droplet_id,
                1.0,
                blobs.get(c).unwrap().location,
                blobs.get(c).unwrap().dimensions,
            ));
            char_to_id.push(c.clone());
        }
        (droplet_vec, char_to_id)
    }

    #[test]
    fn test_no_diff() {
        let strs = vec!["aa..........c",
                        ".....bb......",
                        "............."];

        let (_, exec_blobs) = parse::tests::parse_strings(&strs);
        let (_, chip_blobs) = parse::tests::parse_strings(&strs);

        let (exec_view, char_to_id) = blob_map_to_droplet_map(exec_blobs);

        let mut expected: HashMap<DropletId, Blob> = HashMap::new();
        for id in exec_view.keys() {
            expected.insert(*id, chip_blobs[&char_to_id[id.id]].clone());
        }

        let result: HashMap<DropletId, Blob> = super::match_views(exec_view, chip_blobs.into_iter().map(|(_, blob)| blob).collect());
        for id in expected.keys() {
            assert_eq!(result.get(id), expected.get(id));
        }
    }

    #[test]
    fn test_location_diff() {
        let exec_strs = vec!["aa..........c",
                             ".....bb......",
                             "............."];

        let chip_strs = vec!["aa...........",
                             "............c",
                             ".....bb......"];

        let (_, exec_blobs) = parse::tests::parse_strings(&exec_strs);
        let (_, chip_blobs) = parse::tests::parse_strings(&chip_strs);

        let (exec_view, char_to_id) = blob_map_to_droplet_map(exec_blobs);

        let mut expected: HashMap<DropletId, Blob> = HashMap::new();
        for id in exec_view.keys() {
            expected.insert(*id, chip_blobs[&char_to_id[id.id]].clone());
        }

        let result: HashMap<DropletId, Blob> = super::match_views(exec_view, chip_blobs.into_iter().map(|(_, blob)| blob).collect());
        for id in expected.keys() {
            assert_eq!(result.get(id), expected.get(id));
        }
    }

    #[test]
    fn test_dimension_diff() {
        let exec_strs = vec!["aa..........c",
                             ".....bb......",
                             "............."];
        let chip_strs = vec!["aa.........cc",
                             ".....b.......",
                             ".....b......."];

        let (_, exec_blobs) = parse::tests::parse_strings(&exec_strs);
        let (_, chip_blobs) = parse::tests::parse_strings(&chip_strs);

        let (exec_view, char_to_id) = blob_map_to_droplet_map(exec_blobs);

        let mut expected: HashMap<DropletId, Blob> = HashMap::new();
        for id in exec_view.keys() {
            expected.insert(*id, chip_blobs[&char_to_id[id.id]].clone());
        }

        let result: HashMap<DropletId, Blob> = super::match_views(exec_view, chip_blobs.into_iter().map(|(_, blob)| blob).collect());
        for id in expected.keys() {
            assert_eq!(result.get(id), expected.get(id));
        }
    }

    #[test]
    #[should_panic(expected = "Expected and actual droplets are of different lengths")]
    fn test_mix_split_diff() {
        let exec_strs = vec!["aa...........",
                             ".....bb..c...",
                             "............."];
        let chip_strs = vec!["aa...........",
                             ".....bbb.....",
                             "............."];

        let (_, exec_blobs) = parse::tests::parse_strings(&exec_strs);
        let (_, chip_blobs) = parse::tests::parse_strings(&chip_strs);

        let (exec_view, char_to_id) = blob_map_to_droplet_map(exec_blobs);

        let result: HashMap<DropletId, Blob> = super::match_views(exec_view, chip_blobs.into_iter().map(|(_, blob)| blob).collect());
    }
}
