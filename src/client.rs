use ambient_api::{
    core::transform::components::translation,
    element::{use_entity_component, use_query, use_state, use_state_with},
    prelude::*,
};
use packages::this::{components::screen_item, messages::{ChangeCam, DeleteItem, NewItem, Paint, PlayerSpawn, TeleportToSpawn}};
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
    
    ScreenItems.el().spawn_interactive();
    NewItem::new().send_server_reliable();
}

#[element_component]
fn Interf(_hooks: &mut Hooks) -> Element {
    FlowColumn::el([
        FlowRow::el([Restart.el(), ChCam.el(), PlayerPosition.el()]),
    ])
    .with(width(), 200.)
    .with(space_between_items(), STREET)
    .with_padding_even(STREET)
}

#[element_component]
fn ScreenItems(hooks: &mut Hooks) -> Element {
    let items = use_query(hooks, screen_item());
    let (screen, _) = use_state(hooks, None);
    FlowColumn::el(items.into_iter().map(|(id, _)| {
        PageScreen::el([
                ScreenContainer(screen.clone()).el(),
                Text::el(format!("StartGame")),     
                Button::new("Play", move |_| {
                    DeleteItem::new(id).send_server_reliable();
                    PlayerSpawn.send_server_reliable();
                    Interf.el().spawn_interactive();
                }).el(),
            ])
    }))
    .with(space_between_items(), 10.)
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
