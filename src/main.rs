use bevy::{
     diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, pbr::CascadeShadowConfigBuilder, prelude::*, 
     render::{mesh::PlaneMeshBuilder, view::screenshot::{save_to_disk, Capturing, Screenshot}},
     window::SystemCursorIcon,
     winit::cursor::CursorIcon,
     text::FontSmoothing
};
use bevy_egui::EguiPlugin;
use bevy_dev_tools::fps_overlay::{
    FpsOverlayConfig,
    FpsOverlayPlugin
};

mod values;
mod procedural;
mod tree;

use procedural::ProceduralPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Random Plant Generator".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            EguiPlugin,
            ProceduralPlugin,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(
            FpsOverlayPlugin {
                config: FpsOverlayConfig { 
                    text_config: TextFont {
                        font_size: 14.0,
                        font: default(),
                        font_smoothing: FontSmoothing::default(),
                    }, 
                    text_color: Color::srgb(1.0, 1.0, 0.0), 
                    enabled: true 
                }
            }
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (screenshot_on_spacebar, screenshot_saving))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(
            PlaneMeshBuilder::from_length(20.0)
        )),
        MeshMaterial3d(materials.add(
            Color::srgb(0.3, 0.5, 0.3)
        )),
        Transform::from_xyz(0.0, -0.5, 0.0)
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.),
            ..default()
        },
        CascadeShadowConfigBuilder {
            ..default()
        }
        .build(),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(7.0, 3.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y)
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            right: Val::Px(12.0),
            ..default()
        },
    ));
}

fn screenshot_on_spacebar(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut counter: Local<u32>
) {
    if input.just_pressed(KeyCode::Space) {
        let path = format!("./screenshot-{}.png", *counter);
        *counter += 1;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
}

fn screenshot_saving(
    mut commands: Commands,
    screenshot_saving: Query<Entity, With<Capturing>>,
    window: Single<Entity, With<Window>>
) {
    match screenshot_saving.iter().count() {
        0=> {
            commands.entity(*window).remove::<CursorIcon>();
        }
        x if x > 0 => {
            commands
                .entity(*window)
                .insert(CursorIcon::from(SystemCursorIcon::Progress));
        }
        _ => {}
    }
}