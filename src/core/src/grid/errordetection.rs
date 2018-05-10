use grid::parse::Blob;
use super::{Droplet, DropletId};

use std::collections::{HashMap};


pub fn match_views(
    mut exec_view: HashMap<DropletId, Droplet>,
    mut chip_view: Vec<Blob>,
) -> HashMap<DropletId, Blob> {
    HashMap::new()
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

        let mut expected: HashMap<DropletId, Blob> = HashMap::new();
        for id in exec_view.keys() {
            expected.insert(*id, chip_blobs[&char_to_id[id.id]].clone());
        }

        let result: HashMap<DropletId, Blob> = super::match_views(exec_view, chip_blobs.into_iter().map(|(_, blob)| blob).collect());
        for id in expected.keys() {
            assert_eq!(result.get(id), expected.get(id));
        }
    }
}
