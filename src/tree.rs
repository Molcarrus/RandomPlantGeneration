use bevy::prelude::{
    Transform, 
    Vec3
};

use crate::values::Values;

pub struct Branch(
    pub Transform, 
    pub Option<usize>, 
    pub bool
);

fn generate_leaves(parent_idx: usize, all: &mut Vec<Branch>) {
    let mut branch_transform = Transform::IDENTITY;
    branch_transform = branch_transform.with_translation(branch_transform.local_y().as_vec3());
    all.push(Branch(branch_transform, Some(parent_idx), true));
}

fn generate_branches(values: &Values, height: u8, parent_idx: usize, all: &mut Vec<Branch>) {
    assert!(height >= 1);
    assert!(values.branches > 1);

    for i in 0..values.branches {
        let angle_from_root_branch = values.angle;
        let child_gap_f32 = f32::from(i) / f32::from(values.branches);
        let angle_around_root_branch = 2.0 * std::f32::consts::PI * child_gap_f32;
        let child_idx_f32 = f32::from(i) / f32::from(values.branches - 1);

        let translation_along_root = (1.0 - child_idx_f32) * values.offset_ratio + child_idx_f32;

        let mut child_transform = Transform::IDENTITY;
        child_transform.rotate_local_y(angle_around_root_branch);
        child_transform = child_transform.with_translation(
            child_transform.local_z() 
            * (values.trunk_radius + values.scaling * 0.5 * angle_from_root_branch.sin())
            + child_transform.local_y()
            * ((translation_along_root - 0.5)
                + values.scaling * 0.5 * angle_from_root_branch.cos()
            ),
        );
        child_transform = child_transform.with_scale(Vec3::splat(values.scaling));
        child_transform.rotate_local_x(angle_from_root_branch);

        let child_idx = all.len();
        all.push(Branch(child_transform, Some(parent_idx), false));

        if height < values.height {
            generate_branches(values, height + 1, child_idx, all);
        } else {
            generate_leaves(child_idx, all);
        }
    }
}

pub fn generate(values: &Values) -> Vec<Branch> {
    let base = Transform::default();
    let mut ret: Vec<Branch> = Vec::new();
    ret.push(Branch(base, None, false));
    generate_branches(values, 1, 0, &mut ret);

    ret
}