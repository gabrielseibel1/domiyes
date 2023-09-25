use crate::data::{self, Field, Space};

pub trait Frontend {
    fn show(&self, field: &Field, player: &Space) -> Result<(), std::io::Error>;
    fn input(&mut self) -> Option<Event>;
    fn size(&self) -> Space;
}
pub enum Event {
    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,
    PutDominoVer,
    PutDominoHor,
    PutDominoLeft,
    PutDominoRight,
    Quit,
}

pub struct Game<F: Frontend> {
    frontend: F,
    field: data::Field,
    player: data::Space,
    over: bool,
}

impl<F> Game<F>
where
    F: Frontend,
{
    pub fn new(frontend: F) -> Self {
        let size = frontend.size();
        let field = data::Field::new(size);
        let player = data::Space {
            ver: size.ver / 2,
            hor: size.hor / 2,
        };
        Self {
            frontend,
            field,
            player,
            over: false,
        }
    }
    pub fn play(&mut self) {
        self.frontend.show(&self.field, &self.player).unwrap();
        while !self.over {
            let e_r = self.frontend.input();
            match e_r {
                None => todo!(),
                Some(event) => match event {
                    Event::MoveUp => self.player.ver -= 1,
                    Event::MoveDown => self.player.ver += 1,
                    Event::MoveRight => self.player.hor += 1,
                    Event::MoveLeft => self.player.hor -= 1,
                    Event::PutDominoVer => {
                        self.field.put_domino(self.player, data::Domino::Vertical)
                    }
                    Event::PutDominoHor => {
                        self.field.put_domino(self.player, data::Domino::Horizontal)
                    }
                    Event::PutDominoLeft => {
                        self.field.put_domino(self.player, data::Domino::BackSlash)
                    }
                    Event::PutDominoRight => self
                        .field
                        .put_domino(self.player, data::Domino::ForwardSlash),
                    Event::Quit => self.over = true,
                },
            }
            self.frontend.show(&self.field, &self.player).unwrap();
        }
    }
}
