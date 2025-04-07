use std::sync::MutexGuard;
use bevy::{ ecs::{ entity, query }, prelude::*, utils::info };
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::parry::bounding_volume;
use obj::raw::object::Polygon;
#[derive(Component)]
pub struct AsyncMeshCollider {
    pub mesh_handle: Handle<Mesh>,
}
#[derive(Component)]
pub struct AttachColliderAfterLoad {
    mesh_handle: Handle<Mesh>,
}
// pub fn spwan_obj(mut commands: Commands, asset_server: Res<AssetServer>) {
//     // Load the .obj file (it should be in the assets folder)
//     let obj_handle = asset_server.load("models/chair.obj");
//     commands.spawn(SceneRoot(obj_handle));
// }
use std::fs::File;
use std::io::BufReader;

pub fn spwan_obj(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    // let input = BufReader::new(File::open("assets/models/mycobot_280_M5_230708.obj").unwrap());
    // if let Ok(model) = obj::raw::parse_obj(input) {
    //     let deltas = Isometry::identity();
    //     let mut vertices: Vec<_> = model.positions
    //         .iter()
    //         .map(|v| point![v.0, v.1, v.2])
    //         .collect();
    //     let indices: Vec<_> = model.polygons
    //         .into_iter()
    //         .flat_map(|p| {
    //             match p {
    //                 obj::raw::object::Polygon::P(idx) => idx.into_iter(),
    //                 obj::raw::object::Polygon::PT(idx) =>
    //                     Vec::from_iter(idx.into_iter().map(|i| i.0)).into_iter(),
    //                 obj::raw::object::Polygon::PN(idx) =>
    //                     Vec::from_iter(idx.into_iter().map(|i| i.0)).into_iter(),
    //                 obj::raw::object::Polygon::PTN(idx) =>
    //                     Vec::from_iter(idx.into_iter().map(|i| i.0)).into_iter(),
    //             }
    //         })
    //         .collect();
    //     // Compute the size of the model, to scale it and have similar size for everything.
    //     let aabb = bounding_volume::details::point_cloud_aabb(&deltas, &vertices);
    //     let center = aabb.center();
    //     let diag = (aabb.maxs - aabb.mins).norm();
    //     // let indices: Vec<_> = indices
    //     //     .chunks(3)
    //     //     .map(|idx| [idx[0] as u32, idx[1] as u32, idx[2] as u32])
    //     //     .collect();
    //     // let scene_handle = SceneRoot(asset_server.load("models/mycobot_280_M5_230708.obj"));
    //     // let mesh_handle = Mesh3d(asset_server.load("models/mycobot_280_M5_230708.obj"));
    //     // commands
    //     //     .spawn((
    //     //         scene_handle,
    //     //         Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.01)),
    //     //         RigidBody::Dynamic,
    //     //         Collider::convex_decomposition(&vertices, &indices),
    //     //     ))
    //     //     .insert(Name::new("mycobot"));
    // }
}
// fn extract_vertex_positions(mesh: &Mesh) -> Option<Vec<Vec3>> {
//     mesh.attribute(Mesh::ATTRIBUTE_POSITION).and_then(|attribute| {
//         match attribute {
//             bevy::render::mesh::VertexAttributeValues::Float32x3(positions) => {
//                 Some(
//                     positions
//                         .iter()
//                         .map(|&[x, y, z]| Vec3::new(x, y, z))
//                         .collect()
//                 )
//             }
//             _ => None,
//         }
//     })
// }
// A system to add the collider when the mesh is loaded
pub fn attach_collider_after_load(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    query: Query<(Entity, &AttachColliderAfterLoad)>
) {
    for (entity, attach_collider) in query.iter() {
        if let Some(mesh) = meshes.get(&attach_collider.mesh_handle) {
            if let Some(vertex_positions) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
                if let Some(positions) = vertex_positions.as_float3() {
                    let vertices: Vec<Vec3> = positions
                        .iter()
                        .map(|p| Vec3::from(*p))
                        .collect();
                    if let Some(collider) = Collider::convex_hull(&vertices) {
                        commands.entity(entity).insert(collider);
                    } else {
                        commands.entity(entity).insert(Collider::ball(0.5)); // Fallback
                    }
                }
            }
        }
    }
}

// Rotate only the "head" node
// pub fn rotate_head(
//     mut commands: Commands,
//     meshes: Res<Assets<Mesh>>,
//     query: Query<(Entity, &AsyncMeshCollider)>
//     // mut query: Query<(&Name, Entity, &mut Transform)>,
//     // time: Res<Time>
// ) {
//     for (entity, collider_data) in query.iter() {
//         if let Some(mesh) = meshes.get(&collider_data.mesh_handle) {
//             if let Some(collider) = Collider::convex_hull(mesh.positions().to_vec()) {
//                 commands.entity(entity).insert(collider);
//             }
//         }
//     }
//     // for (name, enitity, mut transform) in query.iter_mut() {
//     //     if
//     //         name.as_str() == "Node4" ||
//     //         name.as_str() == "Node5" ||
//     //         name.as_str() == "Node10" ||
//     //         name.as_str() == "Node11" ||
//     //         name.as_str() == "Node12" ||
//     //         name.as_str() == "Node13"
//     //     {
//     //         info!("mesh node name {}", name.as_str());
//     //         let half_extents = Vec3::new(0.5, 0.5, 0.5); // Adjust size as needed
//     //         let collider = Collider::cuboid(half_extents.x, half_extents.y, half_extents.z);
//     //         commands.entity(enitity).insert(collider);

//     //     }
//     // }
// }
