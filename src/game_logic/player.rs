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
