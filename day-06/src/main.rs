use id_tree::*;
use id_tree::InsertBehavior::*;
use std::str::FromStr;
use common::Puzzle;
use std::collections::HashSet;

fn main() {
    let mut a = Puzzle1 { orbits: Vec::new() };
    a.run();
}

#[derive(Debug)]
struct Orbit {
    centre: String,
    satellite: String,
}

impl FromStr for Orbit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(")").collect();
        let centre = parts[0].to_string();
        let satellite = parts[1].to_string();

        Ok( Orbit { centre, satellite } )
    }
}

#[derive(Debug)]
struct Puzzle1 {
    orbits: Vec<Orbit>,
}

impl Puzzle1 {
    fn add_nodes(&self, tree: &mut Tree<String>, name: &String, parent: &NodeId) {
        for orbit in &self.orbits {
            if name == &orbit.centre {
                let id = tree.insert(Node::new(orbit.satellite.clone()), UnderNode(parent)).unwrap();
                self.add_nodes(tree, &orbit.satellite, &id);
            }
        }
    }
}

impl Puzzle for Puzzle1 {
    type ParsedLine = Orbit;

    fn process_item(&mut self, item: Self::ParsedLine) {
        self.orbits.push(item);
    }

    fn final_result(&mut self) -> String {
        let mut tree = Tree::new();
        let root_id = tree.insert(Node::new(String::from("COM")), AsRoot).unwrap();
        self.add_nodes(&mut tree, &String::from("COM"), &root_id);

        let mut checksum = 0;
        let mut you = None;
        let mut san = None;
        for nid in tree.traverse_pre_order_ids(&root_id).unwrap() {
            if you.is_none() {
                let n = tree.get(&nid).unwrap();
                if n.data() == &"YOU" {
                    you = Some(nid.clone());
                }
            }
            if san.is_none() {
                let n = tree.get(&nid).unwrap();
                if n.data() == &"SAN" {
                    san = Some(nid.clone());
                }
            }
            checksum += tree.ancestor_ids(&nid).unwrap().count();
        }

        let transfers;
        if you.is_some() && san.is_some() {
            let youcestors: HashSet<&NodeId> = tree.ancestor_ids(you.as_ref().unwrap()).unwrap().collect();
            let sancestors: HashSet<&NodeId> = tree.ancestor_ids(san.as_ref().unwrap()).unwrap().collect();
            transfers = youcestors.symmetric_difference(&sancestors).count();
        } else {
            transfers = 0;
        }

        format!("checksum: {}, transfers: {}", checksum, transfers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let orbits = vec![
            "COM)B".parse().unwrap(),
            "B)C".parse().unwrap(),
            "C)D".parse().unwrap(),
            "D)E".parse().unwrap(),
            "E)F".parse().unwrap(),
            "B)G".parse().unwrap(),
            "G)H".parse().unwrap(),
            "D)I".parse().unwrap(),
            "E)J".parse().unwrap(),
            "J)K".parse().unwrap(),
            "K)L".parse().unwrap(),
        ];
        let mut subject = Puzzle1 { orbits };
        let result = subject.final_result();
        assert!(result.contains("checksum: 42"));
    }


    #[test]
    fn test_b() {
        let orbits = vec![
            "COM)B".parse().unwrap(),
            "B)C".parse().unwrap(),
            "C)D".parse().unwrap(),
            "D)E".parse().unwrap(),
            "E)F".parse().unwrap(),
            "B)G".parse().unwrap(),
            "G)H".parse().unwrap(),
            "D)I".parse().unwrap(),
            "E)J".parse().unwrap(),
            "J)K".parse().unwrap(),
            "K)L".parse().unwrap(),
            "K)YOU".parse().unwrap(),
            "I)SAN".parse().unwrap(),
        ];
        let mut subject = Puzzle1 { orbits };
        let result = subject.final_result();
        assert!(result.contains("transfers: 4"));
    }

}
