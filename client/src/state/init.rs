use amethyst::controls::HideCursor;
use amethyst::prelude::*;

use crate::state::login::LoginState;
use crate::util::get_all_maps;
use amethyst::renderer::palette::*;
use amethyst::renderer::resources::AmbientColor;
use amethyst::utils::application_root_dir;
use amethyst::utils::removal::*;
use hoppinworld_runtime::{AllEvents, CustomTrans, ObjectType, PlayerSettings, RemovalId};
use tokio::runtime::Runtime;

#[derive(Default)]
pub struct InitState;

impl<'a, 'b> State<GameData<'a, 'b>, AllEvents> for InitState {
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.register::<ObjectType>();
        data.world.register::<Removal<RemovalId>>();
        data.world.insert(get_all_maps(
            &application_root_dir().unwrap().to_str().unwrap(),
        ));
        data.world
            .insert(AmbientColor(Srgba::new(0.1, 0.1, 0.1, 1.0)));
        let hide_cursor = HideCursor { hide: false };
        data.world.insert(hide_cursor);

        let mut player_settings_path = application_root_dir().unwrap();
        player_settings_path.push("assets/base/config/player.ron");
        let player_settings_path =
            String::from(player_settings_path.to_str().unwrap()).replace("\\", "/");
        let player_settings_data = std::fs::read_to_string(&player_settings_path).expect(&format!(
            "Failed to load player settings from file at {}",
            player_settings_path
        ));
        let player_settings: PlayerSettings =
            ron::de::from_str(&player_settings_data).expect(&format!(
                "Failed to load player settings from file at {}",
                player_settings_path
            ));

        //*data.world.write_resource::<Gravity>() = Gravity::new(0.0, player_settings.gravity, 0.0);

        data.world.insert(player_settings);

        //let mut runtime = Arc::new(Mutex::new(Runtime::new().expect("Failed to create tokio runtime")));
        let runtime = Runtime::new().expect("Failed to create tokio runtime");
        data.world.insert(runtime);
    }

    fn update(&mut self, data: StateData<GameData>) -> CustomTrans<'a, 'b> {
        data.data.update(&data.world);
        Trans::Switch(Box::new(LoginState))
        //Trans::Switch(Box::new(MainMenuState))
    }
}
