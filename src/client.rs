use ambient_api::{
    core::transform::components::translation,
    element::{use_entity_component, use_query, use_state, use_state_with},
    prelude::*,
};
use packages::this::messages::{ChangeCam, Paint, PlayerSpawn, TeleportToSpawn};
#[main]
pub fn main() {
    fixed_rate_tick(Duration::from_millis(20), move |_| {
        let Some(camera_id) = camera::get_active() else {
            return;
        };

        let input = input::get();
        if input.keys.contains(&KeyCode::Q) {
            let ray = camera::clip_position_to_world_ray(camera_id, Vec2::ZERO);

            Paint {
                ray_origin: ray.origin,
                ray_dir: ray.dir,
            }
            .send_server_unreliable();
        }
    });
    App.el().spawn_interactive();
}

#[element_component]
fn App(hooks: &mut Hooks) -> Element {
    FlowColumn::el([
        FlowRow::el([Restart.el(), ChCam.el(), PlayerPosition.el()]),
        FlowRow::el([PlSpawn.el()]),
    ])
    .with(width(), 200.)
    .with(space_between_items(), STREET)
    .with_padding_even(STREET);

    let (screen, set_screen) = use_state(hooks, None);
    PageScreen::el([
        ScreenContainer(screen).el(),
        Text::el("Начало игры"), // Play game
        Button::new("Start", move |_| { // Spawn
            set_screen(Some(SubScreen::el(cb({
                let set_screen = set_screen.clone();
                move || {
                    set_screen(None);
                }
            }))))
        })
        .el(),
    ])
}

#[element_component]
fn SubScreen(hooks: &mut Hooks, on_back: Cb<dyn Fn() + Sync + Send>) -> Element {
    let (screen, set_screen) = use_state(hooks, None);
    // let (id, _) = use_state_with(hooks, |_| friendly_id());
    PageScreen::el([
        ScreenContainer(screen).el(),
        Text::el(format!("SubScreen")),
        Button::new("Back", move |_| on_back()).el(),
        Button::new("Open sub screen", {
            let set_screen = set_screen.clone();
            move |_| {
                set_screen(Some(SubScreen::el(cb({
                    let set_screen = set_screen.clone();
                    move || {
                        set_screen(None);
                    }
                }))))
            }
        },).el(),
        PlSpawn.el(),
    ])
}

#[element_component]
fn PlayerPosition(hooks: &mut Hooks) -> Element {
    let pos = use_entity_component(hooks, player::get_local(), translation());
    Text::el(format!("Достигнутая высота: {}", pos.unwrap_or_default().z))
}

#[element_component]
fn Restart(_hooks: &mut Hooks) -> Element {
    Button::new("Restart", |_| TeleportToSpawn.send_server_reliable())
        .hotkey(VirtualKeyCode::R)
        .el()
}

#[element_component]
fn ChCam(_hooks: &mut Hooks) -> Element {
    Button::new("Cam", |_| ChangeCam.send_server_reliable())
        .hotkey(VirtualKeyCode::V)
        .el()
}

#[element_component]
fn PlSpawn(_hooks: &mut Hooks) -> Element {
    Button::new("Spawn", |_| {
        PlayerSpawn.send_server_reliable();
    })
    .el()
}
