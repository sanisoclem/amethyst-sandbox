use amethyst::prelude::*;

pub struct GameplayState();

impl SimpleState for GameplayState {

}

impl Default for GameplayState {
  fn default() -> Self { GameplayState() }
}
