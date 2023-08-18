use bincode::{Decode, Encode};
use osmpbf::DenseNode;
use serde::Deserialize;
use std::cell::RefCell;

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct Way {
  id: i64,
  tags: Vec<(WayTag, String)>,
  nodes: Vec<Node>,
  max_speed: Option<u8>,
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct Node {
  id: i64,
  lat: f64,
  lon: f64,
  tags: Vec<(String, String)>,
  refs: RefCell<Vec<i64>>,
}

impl<'a> From<&DenseNode<'a>> for Node {
  fn from(node: &DenseNode) -> Self {
    Node {
      id: node.id,
      lat: node.lat(),
      lon: node.lon(),
      // TODO: populate this
      tags: vec![],
      refs: RefCell::default(),
    }
  }
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum WayTag {
  Highway(Highway),
}

#[derive(Encode, Decode, PartialEq, Debug, Deserialize)]
pub enum Highway {
  // A restricted access major divided highway, normally with 2 or
  // more running lanes plus emergency hard shoulder. Equivalent to
  // the Freeway, Autobahn, etc..
  #[serde(rename = "motorway")]
  Motorway, // maxspeed: 98.7%
  // The most important roads in a country's system that aren't motorways.
  // (Need not necessarily be a divided highway.)
  #[serde(rename = "trunk")]
  Trunk, // maxspeed: 82.5%
  // The next most important roads in a country's system.
  // (Often link larger towns.)
  #[serde(rename = "primary")]
  Primary, // maxspeed: 77.8%
  // The next most important roads in a country's system.
  // (Often link towns.)
  #[serde(rename = "secondary")]
  Secondary, // maxspeed: 78.2%
  // The next most important roads in a country's system.
  // (Often link smaller towns and villages)
  #[serde(rename = "tertiary")]
  Tertiary, // maxspeed: 46%
  // The least important through roads in a country's system –
  // i.e. minor roads of a lower classification than tertiary,
  // but which serve a purpose other than access to properties.
  // (Often link villages and hamlets.)

  // The word 'unclassified' is a historical artefact of the UK
  // road system and does not mean that the classification is unknown;
  // you can use highway=road for that.
  #[serde(rename = "unclassified")]
  Unclassified, // maxspeed: 9%
  // Roads which serve as an access to housing, without function of connecting settlements.
  // Often lined with housing.
  #[serde(rename = "residential")]
  Residential, // maxspeed: 4%
  // The link roads (sliproads/ramps) leading to/from a motorway
  // from/to a motorway or lower class highway.
  // Normally with the same motorway restrictions.
  #[serde(rename = "motorway_link")]
  MotorwayLink,
  // The link roads (sliproads/ramps) leading to/from a trunk road from/to
  // a trunk road or lower class highway.
  #[serde(rename = "trunk_link")]
  TrunkLink,
  // The link roads (sliproads/ramps) leading to/from a primary road from/to
  // a primary road or lower class highway.
  #[serde(rename = "primary_link")]
  PrimaryLink,
  // The link roads (sliproads/ramps) leading to/from a secondary road from/to
  // a secondary road or lower class highway.
  #[serde(rename = "secondary_link")]
  SecondaryLink,
  // The link roads (sliproads/ramps) leading to/from a tertiary road from/to
  // a tertiary road or lower class highway.
  #[serde(rename = "tertiary_link")]
  TertiaryLink,
  // For living streets, which are residential streets where pedestrians have
  // legal priority over cars, speeds are kept very low and this is can use
  // for narrow roads that usually using for motorcycle roads.
  #[serde(rename = "living_street")]
  LivingStreet, // maxspeed: 7%
  // A road/way/street/motorway/etc. of unknown type.
  // It can stand for anything ranging from a footpath to a motorway.
  // This tag should only be used temporarily until the road/way/etc.
  // has been properly surveyed. If you do know the road type, do not use this value,
  // instead use one of the more specific highway=* values.
  #[serde(rename = "road")]
  Road,
}
