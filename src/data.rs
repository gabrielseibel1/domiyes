use std::{collections::HashMap, fmt};

/// Space is a struct representing some 2D-space or a point in it
#[derive(fmt::Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Space {
    pub ver: u16,
    pub hor: u16,
}
impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<v:{} x h:{}>", self.ver, self.hor)
    }
}

/// Domino is a special cell that contains some game mechanic
#[derive(Debug)]
pub enum Domino {
    Vertical,
    Horizontal,
    ForwardSlash,
    BackSlash,
}
impl fmt::Display for Domino {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vertical => write!(f, "|"),
            Self::Horizontal => write!(f, "_"),
            Self::ForwardSlash => write!(f, "/"),
            Self::BackSlash => write!(f, "\\"),
        }
    }
}

/// Field is a grid with some dominos
pub struct Field {
    pub size: Space,
    pub dominos: HashMap<Space, Domino>,
}
impl fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO show domino()
        for ver in 0..self.size.ver {
            for hor in 0..self.size.hor {
                match self.dominos.get(&Space { ver, hor }) {
                    None => write!(f, " ")?,
                    Some(dom) => write!(f, "{dom}")?,
                }
            }
        }
        Ok(())
    }
}
impl Field {
    pub fn new(size: Space) -> Self {
        Self {
            size,
            dominos: HashMap::from([]),
        }
    }
    pub fn put_domino(&mut self, position: Space, domino: Domino) {
        self.dominos.insert(position, domino);
    }
}
