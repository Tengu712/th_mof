pub mod game;
pub mod game_ending;
pub mod game_shooting;
pub mod game_talking;
pub mod game_waiting;
pub mod title;

use game::*;
use title::*;

/// A eunm for scene that's matched at mainloop.
pub enum Scene {
    Title(TitleScene),
    Game(GameScene),
}
