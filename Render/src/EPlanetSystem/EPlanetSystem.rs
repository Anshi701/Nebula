

use super::EPlanet::*;
use super::EStar::*;
use crate::{EGlobals::*, 
    EWinChan::win_chan::*,
};

pub struct EPlanetSystem{
    Star: EStar,
    Planets: Vec<EPlanet>,
}


impl EPlanetSystem{
    pub fn new() -> Self{

        let def_star = EStar::new();
        let mut def_planetVec: Vec<EPlanet> = Vec::<EPlanet>::new();

        let def_size_1: f32 = 1000.0; 
        let def_size_2: f32 = 100.0; 
        let def_size_3: f32 = 500.0; 

        let def_pos_1: EPos = EPos::new(0.0, 0.0, 0.0);
        let def_pos_2: EPos = EPos::new(1000.0, 1000.0, 0.0);
        let def_pos_3: EPos = EPos::new(-1500.0, -1500.0, -1500.0);

        def_planetVec.push(EPlanet::new(def_size_1, def_pos_1));
        def_planetVec.push(EPlanet::new(def_size_2, def_pos_2));
        def_planetVec.push(EPlanet::new(def_size_3, def_pos_3));

        Self { 
            Star: def_star,
            Planets: def_planetVec,
        }
    }

    pub fn Draw(&self, channel: &EChannel){
        for planet in &self.Planets{
            planet.Draw(channel);
        }
    }


}