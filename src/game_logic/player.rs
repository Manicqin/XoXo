use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub struct Player{
        player_id:u32
}

impl Player{
    pub fn new(player_id:u32)->Self{
        Player{player_id:player_id}
    }

    pub fn get_id(&self)->u32{
        self.player_id
    }
    
    pub fn get_id_ref(&self)->&u32{
        &self.player_id
    }
}

impl Ord for Player{
    fn cmp(&self, other: &Player) -> Ordering{
        self.player_id.cmp(&other.player_id)            
    }
}

impl PartialOrd for Player{
    fn partial_cmp(&self, other: &Player) -> Option<Ordering>{
        self.player_id.partial_cmp(&other.player_id)
    }
}


impl Eq for Player{
}

impl PartialEq for Player{
    fn eq(&self, other :&Player) -> bool{
        self.player_id.eq(&other.player_id)
    }
    
    fn ne(&self, other :&Player) -> bool{
        self.player_id.ne(&other.player_id)
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
