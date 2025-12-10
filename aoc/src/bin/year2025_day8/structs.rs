use std::cmp::{max, min};
use std::collections::HashSet;

use itertools::Itertools;

type Circuit = HashSet<JunctionBox>;
type Circuits = Vec<Circuit>;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct JunctionBox {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

impl JunctionBox {
    pub fn distance_squared(a: &Self, b: &Self) -> usize { 
        let dx = max(a.x,b.x) - min(a.x,b.x);
        let dy = max(a.y,b.y) - min(a.y,b.y);
        let dz = max(a.z,b.z) - min(a.z,b.z);

        dx*dx + dy*dy + dz*dz
    }
    pub fn parse_text(text: &str) -> Circuits {
        text.lines()
            .map(Self::from)
            .map(|coord| HashSet::from([coord]))
            .collect()
    }
}

impl From<&str> for JunctionBox {
    fn from(text: &str) -> Self {
        let mut split = text.split(',');
        let x: usize = split.next().unwrap().parse().unwrap();
        let y: usize = split.next().unwrap().parse().unwrap();
        let z: usize = split.next().unwrap().parse().unwrap();
        Self { x, y, z}
    }
}

pub fn find_circuit(circuits: &Circuits, junktion_box: &JunctionBox) -> Circuit {
    circuits.iter()
        .find(|circuit| circuit.contains(junktion_box))
        .unwrap()
        .clone()
}

pub fn calculate_distances(circuits: &Circuits) -> Vec<(JunctionBox, JunctionBox)> {
    let junction_boxes: Vec<&JunctionBox> = circuits.iter().flatten().collect();
    let count = junction_boxes.len();

    let mut distance_map: Vec<(usize, (JunctionBox, JunctionBox ))> = vec![];
    for a in 0..count {
        let circuit_a = junction_boxes[a];
        for circuit_b in junction_boxes.iter().take(count).skip(a+1) {
            // let circuit_b = junction_boxes[b];
            let distance = JunctionBox::distance_squared(circuit_a, circuit_b);
            let pair: (JunctionBox, JunctionBox) = (*circuit_a, **circuit_b);
            distance_map.push((distance, pair));
        }
    } 

    distance_map.into_iter()
        .sorted_by(|(a, _),(b, _)| a.cmp(b) )
        .map(|(_, pair)| pair)
        .collect()
}
