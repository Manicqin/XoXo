use std::fmt;

type CordType = i16;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy,Debug)]
pub struct Cord(CordType, CordType);

impl Cord{
    pub fn new(x: usize, y: usize) -> Cord {
        Cord(x as CordType, y as CordType)
    }
}

impl fmt::Display for Cord{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

pub fn major(init: Cord, step: i16) -> Cord {
    let mut retval = init.clone();
    retval.0 = init.0 + step;
    retval.1 = init.1 + step;
    retval
}

pub fn minor(init: Cord, step: i16) -> Cord {
    let mut retval = init.clone();
    retval.0 = init.0 - step;
    retval.1 = init.1 + step;
    retval
}

pub fn horizontal(init: Cord, step: i16) -> Cord {
    let mut retval = init.clone();
    retval.0 = init.0 + step;
    retval
}

pub fn vertical(init: Cord, step: i16) -> Cord {
    let mut retval = init.clone();
    retval.1 = init.1 + step;
    retval
}

pub trait Drawable{
    fn draw(&self);
    fn make_draw(&self)->String;
}

//#[derive(Debug)]
pub enum Direction {
    Horizontal{data:Vec<Cord>},
    Vertical{data:Vec<Cord>},
    Major{data:Vec<Cord>},
    Minor{data:Vec<Cord>},
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match *self{
            Direction::Horizontal{ref data} =>    write!(f, "| {:?}", data),
            Direction::Vertical{ref data}   =>    write!(f, "- {:?}", data),
            Direction::Minor{ref data}      =>    write!(f, "/ {:?}", data),
            Direction::Major{ref data}      =>    write!(f, "\\ {:?}", data),
        }

    }
}

pub enum HotSpots {
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

pub trait GameBoard{
    fn insert(&mut self, coordinate: Cord, character: usize) -> bool;
    fn get(&self, coordinate: Cord) -> Result<usize, &'static str>;
    fn remove(&mut self, coordinate: Cord) -> bool {
        self.insert(coordinate, 0)
    }
    fn check_winning(&self) -> Option<Direction>;
    fn clear(&mut self);

    fn hotspot(&self,spot:HotSpots)->Cord;
    fn count_moves(&self)->usize;
    fn get_dimension(&self)->usize;
    fn convert_index_2_cord(&self,index:usize) -> Result<Cord, &'static str>{
        let dim = self.get_dimension();
        if dim*dim > index {
            let val_x = index / dim;
            let val_y = index - dim*val_x;

            Ok(Cord::new(val_x, val_y))
        }
        else
        {
            Err("out of bounds")
        }
    }

}
