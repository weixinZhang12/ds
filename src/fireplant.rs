use crate::{plant::{self, }, traits::{ TBoom, TIsPlant, TIsPlantAny}};

pub struct FirePlant{
  
}
impl TIsPlant for FirePlant {
    
    fn attack(&mut self) {
        println!("attack")
    }
    
    fn on_die(&mut self) {
        std::println!("This plant has dead");
    }
}
impl TIsPlantAny for FirePlant {
}
impl TBoom for FirePlant {
    fn boom(&mut self) {
        println!("fire boom")
    }
}
