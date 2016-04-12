extern crate rand;

// use board::*;
// use simple_game_board::{Board};
// use player;
use game_logic::simple_game_board::Board;
use game_logic::player;
use game_logic::board::{Direction, Cord, GameBoard, Drawable};
use self::rand::Rng;
use std::sync::Arc;
use std::collections::BTreeMap;

#[derive(Debug,Clone)]
pub struct GameSession{
    game_id:u32,
    board:Board,
    round:usize,
    players:BTreeMap<u32,Arc<player::Player>>,
}

impl GameSession{
    pub fn new(dim:usize) -> GameSession {
        GameSession{
            game_id:0,
            board:Board::new(dim),
            round:0,
            players:BTreeMap::new(),
        }
    }

    pub fn contains_player(&self,id:&u32)->bool{
        self.players.contains_key(id)
    }
    
    pub fn find_player(&self,id:&u32)->Option<&Arc<player::Player>>{
        self.players.get(id)
    }
    
    pub fn reset(&mut self){
        self.board.clear();
        self.round = 0;
    }
    pub fn get_dimension(&self)->usize{
            self.board.get_dimension()
    }

    pub fn check_winning(&self)-> Option<Direction>{
        self.board.check_winning()
    }
    pub fn attach_player(&mut self,player:Arc<player::Player>)->bool{
        let mut retval = false;
        if self.round==0 &&
        !self.players.contains_key(player.get_id_ref()){
            self.players.insert(player.get_id(),player);
            retval = true;
        }
        return retval;
    }

    pub fn play(&mut self)-> Option<Direction>{

        if self.players.len() < 2{
            panic!("need more players");
        }
        if (self.board.get_dimension() * self.board.get_dimension()) > self.round &&
            self.board.count_moves() < (self.board.get_dimension() * self.board.get_dimension())
        {

            let mut cord : Cord = Cord::new(0,0);

            let mut test= Ok(1);
            let mut iterations = 0;
            while let Ok(i) = test{
                if i > 0 {
                    cord = Cord::new(rand::thread_rng().gen_range(0, self.board.get_dimension()),
                                                 rand::thread_rng().gen_range(0, self.board.get_dimension()));
                    test = self.board.get(cord);
                    iterations = iterations+1;
                }
                else{
                    test= Err("");
                }
            }
            let id =self.get_player_by_round().get_id();
            self.board.insert(cord, id as usize);
            self.round = self.round + 1;
        }
        self.board.check_winning()
    }

    pub fn get_player_by_round(&self)->Arc<player::Player>{

        let playa = self.round % self.players.len();
        self.players.get(&(playa as u32)).unwrap().clone()

    }

    pub fn make_draw(&self)->String{
        self.board.make_draw()
    }
}
