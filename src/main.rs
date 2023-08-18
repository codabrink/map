use std::{collections::HashMap, fs::File, io::BufReader, time::Duration};

use anyhow::Result;
use osmpbf::{Element, ElementReader};

use crate::grid::Grid;

mod grid;
mod way;

fn main() -> Result<()> {
  let _ = std::fs::create_dir_all("grid");

  let mut grid = Grid::default();
  let reader = create_reader()?;
  reader.for_each(|el| {
    grid.insert(&el).unwrap();
  })?;

  println!("done");
  std::thread::sleep(Duration::from_secs(100));

  Ok(())
}

fn create_reader() -> Result<ElementReader<BufReader<File>>> {
  Ok(ElementReader::from_path("michigan.osm.pbf")?)
}

fn collect_nodes() -> Result<()> {
  let mut nodes = HashMap::new();
  let reader = create_reader()?;
  reader.for_each(|el| {
    if let Element::DenseNode(way) = el {
      let mut tags = vec![];
      for (k, v) in way.tags() {
        tags.push((k.to_owned(), v.to_owned()));
      }
      nodes.insert(way.id, (way.id, tags, (way.lat(), way.lon())));
    }
  })?;
  Ok(())
}
