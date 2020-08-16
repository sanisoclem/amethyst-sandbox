use amethyst::{
  prelude::*,
  renderer::{
      plugins::{RenderShaded3D, RenderSkybox, RenderToWindow},
      types::DefaultBackend,
      RenderingBundle,
  },
  utils::{application_root_dir},
};

mod states;

fn main() -> amethyst::Result<()> {

  amethyst::start_logger(Default::default());

  let app_root = application_root_dir()?;
  let config_dir = app_root.join("config");
  let assets_dir = app_root.join("assets");

  let display_config_path = config_dir.join("display.ron");

  let game_data = GameDataBuilder::default()
      .with_bundle(
          RenderingBundle::<DefaultBackend>::new()
              .with_plugin(
                  RenderToWindow::from_config_path(display_config_path)?
                      .with_clear([0.0, 0.0, 0.0, 1.0]),
              )
              .with_plugin(RenderShaded3D::default())
              .with_plugin(RenderSkybox::default()),
      )?;

  let mut game = Application::build(assets_dir, states::GameplayState::default() )?
      .build(game_data)?;

  game.run();
  Ok(())
}
