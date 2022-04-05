pub mod game;
pub mod title;

use game::*;
use title::*;

/// A eunm for scene that's matched at mainloop.
pub enum Scene {
    Title(TitleScene),
    Game(GameScene),
}
