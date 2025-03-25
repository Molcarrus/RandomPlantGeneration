use bevy::prelude::*;

use bevy_egui::{
    egui::{
        self, 
        Slider
    },
    EguiContexts,
};

use rand::Rng;

use crate::values::{
    Values, ValueVector, ANGLE_MINMAX, TRUNK_RADIUS_MINMAX, BRANCHES_MINMAX, LEAF_RADIUS_MINMAX, HEIGHT_MINMAX, SCALE_MINMAX, OFFSET_RATIO_MINMAX,
};

use crate::tree::generate;

pub struct ProceduralPlugin;

impl Plugin for ProceduralPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewValuesEvent>()
            .insert_resource(RedrawTimer(Timer::from_seconds(0.1, TimerMode::Once)))
            .init_resource::<Values>()
            .init_resource::<ValueVector>()
            .add_systems(
                Update, 
                (render_tree, rotator_system, keep_generating, ui_system),
            );
    }
}

#[derive(Resource)] 
struct RedrawTimer(Timer);

#[derive(Event)]
struct NewValuesEvent;

#[derive(Component)]
struct TreeRoot;

fn rotator_system(
    time: Res<Time>, 
    mut query: Query<&mut Transform, With<TreeRoot>>
) {
    for mut transform in &mut query {
        transform.rotate_y(0.1 * time.delta_secs());
    }
}

fn keep_generating(
    time: Res<Time>,
    mut timer: ResMut<RedrawTimer>,
    mut values: ResMut<Values>,
    mut values_vel: ResMut<ValueVector>,
    mut value_changed_event: EventWriter<NewValuesEvent>
) {
    if timer.0.tick(time.delta()).just_finished() {
        values_vel.nudge();
        let mut values_pos = ValueVector::from_values(&values);
        values_pos.add(&mut values_vel);
        *values = values_pos.to_values();
        value_changed_event.send(NewValuesEvent);
    }
}

fn render_tree(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<TreeRoot>>,
    values: Res<Values>,
    mut value_events: EventReader<NewValuesEvent>
) {
    for _ in value_events.read() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        let tree = generate(&values);
        let mut entity_parent_indices: Vec<(Entity, Option<usize>)> = Vec::new();

        let t = values.angle * 2.0 / std::f32::consts:: PI;
        let color_r = (1.0 - t * 2.0).max(0.0);
        let color_g = if t < 0.5 { 2.0 * t } else { 2.0 - 2.0 * t};
        let color_b = (t * 2.0 - 1.0).max(0.0);

        for branch in &tree {
            if branch.2 {
                let entity_id = commands
                    .spawn((
                        Mesh3d(meshes.add(
                            Sphere::new(values.leaf_radius).mesh().ico(2).unwrap()
                        )),
                        MeshMaterial3d(materials.add(
                            Color::srgb(color_r, color_g, color_b)
                        )),
                        branch.0
                    ))
                    .id();
                entity_parent_indices.push((entity_id, branch.1));
            } else {
                let entity_id = commands
                    .spawn((
                        Mesh3d(meshes.add(
                            Cylinder::new(values.trunk_radius, 1.0).mesh().segments(6).resolution(6)
                        )),
                        MeshMaterial3d(materials.add(
                            Color::srgb(0.8, 0.7, 0.6)
                        )),
                        branch.0
                    ))
                    .id();
                entity_parent_indices.push((entity_id, branch.1));
            }
        }

        for (child_id, par_id) in &entity_parent_indices {
            if par_id.is_some() {
                let parent_id = entity_parent_indices[par_id.unwrap()].0;
                commands.entity(parent_id).add_children(&[*child_id]);
            }
        }

        commands.entity(entity_parent_indices[0].0).insert(TreeRoot);
    }
}

fn ui_system(
    mut values: ResMut<Values>,
    mut timer: ResMut<RedrawTimer>,
    mut value_changed_event: EventWriter<NewValuesEvent>,
    mut contexts: EguiContexts
) {
    egui::Window::new("Tree Values").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Branches: ");
            if ui
                .add(Slider::new(
                    &mut values.branches,
                    BRANCHES_MINMAX[0]..=BRANCHES_MINMAX[1],
                ))
                .changed() 
            {
                value_changed_event.send(NewValuesEvent);
            }
        });

        ui.horizontal(|ui| {
            ui.label("Height: ");
            if ui
                .add(Slider::new(
                    &mut values.height,
                    HEIGHT_MINMAX[0]..=HEIGHT_MINMAX[1],
                ))
                .changed() 
            {
                value_changed_event.send(NewValuesEvent);
            }
        });

        ui.horizontal(|ui| {
            ui.label("Offset Ratio: ");
            if ui
                .add(Slider::new(
                    &mut values.offset_ratio,
                    OFFSET_RATIO_MINMAX[0]..=OFFSET_RATIO_MINMAX[1],
                ))
                .changed() 
            {
                value_changed_event.send(NewValuesEvent);
            }
        });

        ui.horizontal(|ui| {
            ui.label("Angle: ");
            if ui
                .add(Slider::new(
                    &mut values.angle,
                    ANGLE_MINMAX[0]..=ANGLE_MINMAX[1],
                ))
                .changed()
            {
                value_changed_event.send(NewValuesEvent);
            }
        });

        ui.horizontal(|ui| {
            ui.label("Scaling: ");
            if ui
                .add(Slider::new(
                    &mut values.scaling,
                    SCALE_MINMAX[0]..=SCALE_MINMAX[1]
                ))
                .changed()
            {
                value_changed_event.send(NewValuesEvent);
            }
        });

        ui.horizontal(|ui| {
            ui.label("Trunk Radius: ");
            if ui
                .add(Slider::new(
                    &mut values.trunk_radius,
                    TRUNK_RADIUS_MINMAX[0]..=TRUNK_RADIUS_MINMAX[1]
                ))
                .changed()
            {
                value_changed_event.send(NewValuesEvent);
            }
        });

        ui.horizontal(|ui| {
            ui.label("Leaf Radius: ");
            if ui
                .add(Slider::new(
                    &mut values.leaf_radius,
                    LEAF_RADIUS_MINMAX[0]..=LEAF_RADIUS_MINMAX[1]
                ))
                .changed() 
            {
                value_changed_event.send(NewValuesEvent);
            }
        });

        ui.horizontal(|ui| {
            if ui
                .add(egui::Button::new("Generate"))
                    .clicked() 
            {
                let mut rng = rand::rng();
                values.branches = rng.random_range(BRANCHES_MINMAX[0]..=BRANCHES_MINMAX[1]);
                values.height = rng.random_range(HEIGHT_MINMAX[0]..=HEIGHT_MINMAX[1]);
                values.offset_ratio = rng.random_range(OFFSET_RATIO_MINMAX[0]..=OFFSET_RATIO_MINMAX[1]);
                values.angle = rng.random_range(ANGLE_MINMAX[0]..=ANGLE_MINMAX[1]);
                values.scaling = rng.random_range(SCALE_MINMAX[0]..=SCALE_MINMAX[1]);
                values.trunk_radius = rng.random_range(TRUNK_RADIUS_MINMAX[0]..=TRUNK_RADIUS_MINMAX[1]);
                values.leaf_radius = rng.random_range(LEAF_RADIUS_MINMAX[0]..=LEAF_RADIUS_MINMAX[1]);

                value_changed_event.send(NewValuesEvent);
            }
        });

        ui.horizontal(|ui| {
            if ui
                .add(egui::Button::new("Keep Generating"))
                .clicked()
            {
                if timer.0.mode() == TimerMode::Once {
                    timer.0.set_mode(TimerMode::Repeating);
                } else {
                    timer.0.set_mode(TimerMode::Once);
                }
            }
        })
    });
}
