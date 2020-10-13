//! Mainly taken from bevy_input_prototype
use bevy::{
    prelude::*,
    render::mesh::{Indices, VertexAttribute},
};
use lyon::tessellation::{
    BuffersBuilder, StrokeOptions, StrokeTessellator, StrokeVertex, VertexBuffers,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ParseError;

struct Geometry(pub VertexBuffers<[f32; 3], u32>);

impl From<Geometry> for Mesh {
    fn from(geometry: Geometry) -> Self {
        let num_vertices = geometry.0.vertices.len();
        let mut mesh = Self::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
        mesh.indices = Some(Indices::U32(geometry.0.indices));
        mesh.attributes
            .push(VertexAttribute::position(geometry.0.vertices));

        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        for _ in 0..num_vertices {
            normals.push([0.0, 0.0, 0.0]);
            uvs.push([0.0, 0.0]);
        }

        mesh.attributes.push(VertexAttribute::normal(normals));
        mesh.attributes.push(VertexAttribute::uv(uvs));

        mesh
    }
}

/// Returns a `SpriteComponents` bundle with the given [`Geometry`](Geometry)
/// and `ColorMaterial`.
fn create_sprite(
    material: Handle<ColorMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
    geometry: Geometry,
    translation: Vec3,
) -> SpriteComponents {
    SpriteComponents {
        material,
        mesh: meshes.add(geometry.into()),
        sprite: Sprite {
            size: Vec2::new(1.0, 1.0),
            ..Default::default()
        },
        transform: Transform::from_translation(translation),
        ..Default::default()
    }
}

pub fn stroke(
    path: lyon::path::Path,
    material: Handle<ColorMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
    translation: Vec3,
    options: &StrokeOptions,
) -> SpriteComponents {
    let mut tessellator = StrokeTessellator::new();
    let mut geometry = Geometry(VertexBuffers::new());
    tessellator
        .tessellate_path(
            path.as_slice(),
            options,
            &mut BuffersBuilder::new(&mut geometry.0, |pos: StrokeVertex| {
                [pos.position().x, pos.position().y, 0.0]
            }),
        )
        .unwrap();

    create_sprite(material, meshes, geometry, translation)
}