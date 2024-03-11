use ambient_api::{
    core::transform::components::translation, element::{use_entity_component, use_state}, prelude::*,
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
fn App(_hooks: &mut Hooks) -> Element {
    FlowColumn::el(
        [FlowRow::el([Restart.el(), ChCam.el(), PlayerPosition.el()]),
            FlowRow::el([
                PlSpawn.el()
            ])]
    ).with(width(), 200.)
    .with(space_between_items(), STREET)
    .with_padding_even(STREET)
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
fn PlSpawn(hooks: &mut Hooks) -> Element {
    Button::new("Spawn", |_| PlayerSpawn.send_server_reliable())
        .hotkey(VirtualKeyCode::Space)
        .el()
}
