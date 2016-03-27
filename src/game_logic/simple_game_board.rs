use game_logic::board::{Cord, GameBoard, Drawable, Direction, HotSpots, vertical, horizontal, major, minor};

use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Write;
#[derive(Debug,Clone)]
pub struct Board {
    board: BTreeMap<Cord, usize>,
    dimension: usize,
}

impl Board{
    pub fn new(dim: usize) -> Board {
        Board {
            board: BTreeMap::new(),
            dimension: dim,
        }
    }

    fn check_winning_ver(&self) -> Option<Direction> {
        let mut retval : Option<Direction> = None;

        for idx in 0..self.dimension{
            let start_cord = Cord::new(idx, 0);

            let checked_vertical = (0..self.dimension as i16)
                                    .map(|x| vertical(start_cord, x));

            let vertical_cord_value = self.get(start_cord).unwrap();

            let vertical_vec = checked_vertical.collect::<Vec<Cord>>();

            match self.get_vec(vertical_vec.clone()).unwrap().iter()
                                        .all(|x| *x == vertical_cord_value && *x != 0) {
                true => {
                    retval = Some(Direction::Vertical{data:vertical_vec.clone()});
                    break;
                },
                false => retval = None
            }
        }
        retval
    }

    fn check_winning_hor(&self) -> Option<Direction> {
        let mut retval : Option<Direction> = None;

        for idx in 0..self.dimension{
            let start_cord = Cord::new(0, idx);

            let checked_vertical = (0..self.dimension as i16)
                                    .map(|x| horizontal(start_cord, x));

            let vertical_cord_value = self.get(start_cord).unwrap();

            let vertical_vec = checked_vertical.collect::<Vec<Cord>>();
            match self.get_vec(vertical_vec.clone()).unwrap().iter().all(|x| *x == vertical_cord_value && *x != 0) {
                true => {
                    retval = Some(Direction::Horizontal{data:vertical_vec.clone()});
                    break;
                },
                false => retval = None
            }
        }
        retval
    }

    fn get_vec(&self, coordinates: Vec<Cord>) -> Result<Vec<usize>, &'static str> {
        let mut retval = Vec::with_capacity(coordinates.len());

        for (_idx, coord) in coordinates.into_iter().enumerate() {
            match self.get(coord){
                Ok(x)=> {retval.push(x);},
                Err(err) => {return Err(err);}
            }
            retval.push(self.get(coord).unwrap());
        }
        Ok(retval)
    }

    fn check_winning_cross(&self) -> Option<Direction> {

        self.check_winning_ver().or(self.check_winning_hor())
    }

    fn check_winning_diag(&self) -> Option<Direction> {
        let start_major_cord = self.hotspot(HotSpots::LeftUp);
        let start_minor_cord = self.hotspot(HotSpots::RightUp);

        let checked_major = (0..self.dimension as i16)
                                .map(|x| major(start_major_cord, x));
        let checked_minor = (0..self.dimension as i16)
                                .map(|x| minor(start_minor_cord, x));


        let major_cord_value = self.get(start_major_cord).unwrap();
        let minor_cord_value = self.get(start_minor_cord).unwrap();

        let major_vec = checked_major.collect::<Vec<_>>();
        let minor_vec = checked_minor.collect::<Vec<_>>();
        match self.get_vec(major_vec.clone()).unwrap().iter().all(|x| *x == major_cord_value && *x != 0) {
            true => Some(Direction::Major{data:major_vec.clone()}),
            false => {
                match self.get_vec(minor_vec.clone()).unwrap()
                          .iter()
                          .all(|x| *x == minor_cord_value && *x != 0) {
                    true => Some(Direction::Minor{data:minor_vec}),
                    false => None,
                }
            }
        }
    }
}

impl GameBoard for Board{
    fn clear(&mut self){
        self.board.clear();
    }

    fn get_dimension(&self)->usize{
        self.dimension
    }

    fn insert(&mut self, coordinate: Cord, character: usize) -> bool {

        self.board.insert(coordinate, character);
        return true;
    }

    fn get(&self, coordinate: Cord) -> Result<usize, &'static str> {

        if coordinate < Cord::new(self.dimension,self.dimension) {
            match self.board.get(&coordinate) {
                None => Ok(0),
                Some(x) => Ok(x.clone()),
            }
        } else {
            Err("xoxo: out of bounds")
        }
    }


    fn check_winning(&self) -> Option<Direction> {
        self.check_winning_diag().or(self.check_winning_cross())
    }

    fn hotspot(&self,spot:HotSpots)->Cord
    {
        match spot{
            HotSpots::LeftUp    => Cord::new(0, 0),
            HotSpots::LeftDown  => Cord::new(0, self.dimension-1),
            HotSpots::RightUp   => Cord::new(self.dimension-1, 0),
            HotSpots::RightDown => Cord::new(self.dimension-1, self.dimension-1),
        }
    }

    fn count_moves(&self)->usize{
        self.board.len()
    }
}

impl fmt::Display for Board{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for (k, v) in &self.board {
            try!(write!(f, "({}) => {}\n", k, v))
        }

        Ok(())
    }
}

impl Drawable for Board{
    fn draw(&self) {

        for x in 0..self.dimension {
            for y in 0..self.dimension {
                match self.get(Cord::new(x as usize, y as usize)) {
                    Ok(character) => print!("\t({},{}){}", x, y, character),
                    _ => print!("\t"),
                }
            }

            println!("");
        }
    }

    fn make_draw(&self)->String{
        let mut retval = String::default();
        for x in 0..self.dimension {
            for y in 0..self.dimension {
                match self.get(Cord::new(x as usize, y as usize)) {
                    Ok(character) => {
                                        retval = format!("{}    ({})",retval, character);
                                    },
                    _ => {retval = format!("{}  ",retval);},
                }
            }
            retval = format!("{}</br>",retval);
        }
        retval
    }
}
