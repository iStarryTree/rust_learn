use anyhow::Result;
use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Player {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    #[serde(rename = "Nationality")]
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub number: u8
}

fn main()->Result<()> {
    let file = File::open("C:/Users/Starr/Documents/hao/my_github/rust_learn/rust_live/rcli/assets/juventus.csv")?;
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize(){
        let player:Player = result?;
        println!("Name:{:}, Position:{:?}, Number:{:?}", player.name, player.position, player.number);
    }
    Ok(())
}

impl Player{
    pub fn to_json(&self)->Result<String>{
        let json = serde_json::to_string(self)?;
        Ok(json)
    }
}

impl Player{
    pub fn from_json(json:&str)->Result<Player>{
        let player = serde_json::from_str(json)?;
        Ok(player)
    }
}