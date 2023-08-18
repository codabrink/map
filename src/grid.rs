use crate::way::{Highway, Node, Way};
use anyhow::Result;
use bincode::{Decode, Encode};
use osmpbf::{DenseNode, Element, Way as OSMWay};
use std::collections::HashMap;

const RESOLUTION: f64 = 100f64;
type Coord = (i16, i16);

#[derive(Encode, Decode, Default)]
struct Cell {
  nodes: HashMap<i64, Node>,
}

#[derive(Encode, Decode, Default)]
pub struct Grid {
  cells: HashMap<Coord, Cell>,
  ways: HashMap<i64, Way>,
}

impl Grid {
  pub fn insert(&mut self, el: &Element) -> Result<()> {
    match el {
      Element::DenseNode(node) => self.insert_dense_node(node)?,
      Element::Way(way) => self.insert_way(way)?,
      _ => {}
    }

    Ok(())
  }

  fn insert_dense_node(&mut self, node: &DenseNode) -> Result<()> {
    self
      .cell(node.lat(), node.lon())?
      .nodes
      .insert(node.id, node.into());
    Ok(())
  }

  fn cell(&mut self, lat: f64, lon: f64) -> Result<&mut Cell> {
    let coord = Self::lat_lon_to_coord(lat, lon)?;
    Ok(self.cells.entry(coord).or_insert(Cell::default()))
  }

  fn lat_lon_to_coord(lat: f64, lon: f64) -> Result<Coord> {
    Ok(((lat / RESOLUTION) as i16, (lon / RESOLUTION) as i16))
  }

  fn insert_way(&mut self, way: &OSMWay) -> Result<()> {
    let mut hw: Option<Highway> = None;
    for (k, v) in way.tags() {
      match k {
        "highway" => {
          let _hw: Highway = match serde_json::from_str(v) {
            Ok(hw) => hw,
            // if it fails, it's a type we don't care about
            _ => continue,
          };
          hw = Some(_hw);
        }
        _ => continue,
      }
    }

    Ok(())
  }
}
