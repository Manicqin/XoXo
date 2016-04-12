use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub struct Player{
        id:u32
}

impl Player{
    pub fn new(player_id:u32)->Self{
        Player{id:player_id}
    }

    pub fn get_id(&self)->u32{
        self.id
    }
    
    pub fn get_id_ref(&self)->&u32{
        &self.id
    }
}

impl Ord for Player{
    fn cmp(&self, other: &Player) -> Ordering{
        self.id.cmp(&other.id)            
    }
}

impl PartialOrd for Player{
    fn partial_cmp(&self, other: &Player) -> Option<Ordering>{
        self.id.partial_cmp(&other.id)
    }
}


impl Eq for Player{
}

impl PartialEq for Player{
    fn eq(&self, other :&Player) -> bool{
        self.id.eq(&other.id)
    }
    
    fn ne(&self, other :&Player) -> bool{
        self.id.ne(&other.id)
    }
}
// pub struct PlayerRegistration{
//     players_hub:Vec<Player>
// }
//
// impl PlayerRegistration{
//     //add
//     //remove
//
// }
