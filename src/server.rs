use ambient_api::{
    core::{
        hierarchy::components::parent,
        model::components::model_from_url,
        physics::components::{cube_collider, plane_collider},
        player::components::user_id,
        primitives::components::{cube, quad},
        rendering::components::color,
        transform::components::{rotation, scale, translation},
    }, entity::despawn, prelude::*
};
use packages::{
    character_animation::components::basic_character_animations,
    character_controller::components::{camera_distance, use_character_controller}, this::messages::{ChangeCam, Paint, PlayerSpawn, TeleportToSpawn},
};

use crate::packages::this::components::{max_h, will_destroyed};

#[main]
pub async fn main() {
    Entity::new()
        .with(quad(), ())
        .with(scale(), Vec3::ONE * 10.0)
        .with(color(), vec4(1.0, 1.0, 1.0, 1.0))
        .with(plane_collider(), ())
        .spawn();

    // spawn_query(is_player()).bind(move |players| {
    //     for (id, _) in players {
    //         entity::add_components(
    //             id,
    //             Entity::new()
    //                 .with(use_character_controller(), ())
    //                 .with(
    //                     model_from_url(),
    //                     packages::base_assets::assets::url("Y Bot.fbx"),
    //                 )
    //                 .with(basic_character_animations(), id)
    //                 .with(color(), random::<Vec4>())
    //                 .with(max_h(), 0.0)
    //                 .with(camera_distance(), -1.0),
    //         );
    //     }
    // });

    Paint::subscribe(|ctx, msg| {
        if ctx.client_user_id().is_none() {
            return;
        }

        let Some(hit) = physics::raycast_first(msg.ray_origin, msg.ray_dir) else {
            return;
        };

        Entity::new()
            .with(cube(), ())
            .with(translation(), hit.position)
            .with(scale(), Vec3::ONE * 0.1)
            .with(color(), vec4(0., 1., 0., 1.))
            .spawn();
    });

    PlayerSpawn::subscribe(move |ctx, _| {
        let Some(player_id) = ctx.client_entity_id() else {
            return;
        };

        entity::add_components(
            player_id,
            Entity::new()
                .with(use_character_controller(), ())
                .with(
                    model_from_url(),
                    packages::base_assets::assets::url("Y Bot.fbx"),
                )
                .with(basic_character_animations(), player_id)
                .with(color(), random::<Vec4>())
                .with(max_h(), 0.0)
                .with(camera_distance(), -1.0),
        );
    });

    TeleportToSpawn::subscribe(move |ctx, _| {
        let Some(player_id) = ctx.client_entity_id() else {
            return;
        };

        entity::mutate_component(player_id, translation(), |t| {
            *t = Vec3::Z;
        });
    });

    ChangeCam::subscribe(move |ctx, _| {
        let Some(player_id) = ctx.client_entity_id() else {
            return;
        };

        entity::mutate_component(player_id, camera_distance(), |t| {
            *t = if *t == 2.0 { -1.0 } else { 2.0 };
        });
    });

    for i in 0..10 {
        let fi = i as f32;
        Entity::new()
            .with(cube(), ())
            .with(
                translation(),
                (Vec2::X * fi).extend(fi) + vec3(2.0, 0.0, 0.5),
            )
            .with(cube_collider(), Vec3::ONE)
            .with(color(), vec4(0., 0., 1., 1.))
            .spawn();
    }

    // let mut h = 0.0;
    change_query((user_id(), translation(), rotation(), color(), max_h()))
        .track_change(translation())
        .bind(move |players| {
            for (ent_id, (_, trans, rot, colr, mut h)) in players {
                if trans.z - h > 0.5 {
                    entity::set_component(ent_id, max_h(), trans.z);
                    h = trans.z;
                    for _ in 0..(h as u32) {
                        Entity::new()
                            .with(cube(), ())
                            .with(parent(), ent_id)
                            .with(rotation(), random::<Quat>())
                            .with(
                                translation(),
                                trans
                                    + Mat3::from_quat(rot).mul_vec3(
                                        ((random::<Vec2>() - 0.5) * h * h).extend(0.0)
                                            + vec3(5.0, 0.0, 2.0),
                                    ),
                            )
                            .with(cube_collider(), Vec3::ONE)
                            .with(will_destroyed(), true)
                            .with(scale(), vec3(3.0, 3.0, 0.1))
                            .with(scale(), random::<Vec3>() * h)
                            .with(color(), colr)
                            .spawn();
                    }
                    println!("Новая высота достигнута: {h}");
                } else if trans.z < 0.1 && h > 1.0 {
                    println!("Вы дошли до {h} метров");
                    let platforms = query(will_destroyed()).build().evaluate();
                    for (ent_id, _) in platforms {
                        despawn(ent_id);
                    }
                    entity::set_component(ent_id, max_h(), 0.0);
                };
            }
        });
}
