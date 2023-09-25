mod data;
mod game;
mod terminal;

use color_eyre::eyre::Result;
use game::Game;
use terminal::Terminal;
use termion::raw::IntoRawMode;

fn main() -> Result<()> {
    // prepare errors
    color_eyre::install()?;

    // prepare termion
    let _stdout = std::io::stdout().into_raw_mode();

    let frontend = Terminal::new();
    let mut game = Game::new(frontend);

    game.play();

    Ok(())
}
