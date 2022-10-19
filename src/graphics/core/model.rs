use anyhow::*;
use wgpu::util::DeviceExt;

pub struct Model {
    meshes: Vec<(super::MeshBuffer, usize)>,
    materials: Vec<(super::Texture, wgpu::BindGroup)>,
}

impl Model {
    pub fn new(
        meshes: Vec<(super::MeshBuffer, usize)>,
        materials: Vec<(super::Texture, wgpu::BindGroup)>
    ) -> Self {
        Self {
            meshes,
            materials,
        }
    }

    // pub fn from_path(
    //     path: &str,
    //     bundle: &super::bundle::Bundle,
    //     layout: &wgpu::BindGroupLayout
    // ) -> Result<Self> {
    //     let obj_text = std::fs::read_to_string(path).unwrap();
    //     let obj_cursor = std::io::Cursor::new(obj_text);
    //     let mut obj_reader = std::io::BufReader::new(obj_cursor);

    //     let (models, obj_materials) = tobj::load_obj_buf(
    //         &mut obj_reader,
    //         &tobj::LoadOptions {
    //             triangulate: true,
    //             single_index: true,
    //             ..Default::default()
    //         },
    //         |p| {
    //             let mat_text = std::fs::read_to_string(&p).unwrap();
    //             tobj::load_mtl_buf(&mut std::io::BufReader::new(std::io::Cursor::new(mat_text)))
    //         },
    //     ).unwrap();

    //     let mut materials = Vec::new();
    //     for m in obj_materials.unwrap() {
    //         let diffuse_texture = super::Texture::from_path(
    //             bundle, &m.diffuse_texture, "model_texture").unwrap();
    //         let bind_group = bundle.device().create_bind_group(&wgpu::BindGroupDescriptor {
    //             layout,
    //             entries: &[
    //                 wgpu::BindGroupEntry {
    //                     binding: 0,
    //                     resource: wgpu::BindingResource::TextureView(&diffuse_texture.view()),
    //                 },
    //                 wgpu::BindGroupEntry {
    //                     binding: 1,
    //                     resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler()),
    //                 },
    //             ],
    //             label: None,
    //         });

    //         materials.push((diffuse_texture, bind_group));
    //     }

    //     let meshes = models.into_iter().map(|m| {
    //         let vertices = super::Vertices::new((0..m.mesh.positions.len() / 3).map(|i| super::ModelVertex::new(
    //             glam::vec3(
    //                 m.mesh.positions[i * 3],
    //                 m.mesh.positions[i * 3 + 1],
    //                 m.mesh.positions[i * 3 + 2]),
    //             glam::vec2(m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]),
    //             glam::vec3(
    //                 m.mesh.normals[i * 3],
    //                 m.mesh.normals[i * 3 + 1],
    //                 m.mesh.normals[i * 3 + 2],
    //             )
    //         )).collect::<Vec<_>>());

    //         let indices = super::Indices::new(m.mesh.indices);

    //         let mesh = super::Mesh::new(vertices, indices);

    //         (mesh.mesh_buffer(bundle), m.mesh.material_id.unwrap_or(0))
    //     }).collect::<Vec<_>>();

    //     Ok(Model::new(meshes, materials))
    // }

    // pub fn meshes(&self) -> &Vec<(super::MeshBuffer, usize)> {
    //     &self.meshes
    // }

    // pub fn materials(&self) -> &Vec<(super::Texture, wgpu::BindGroup)> {
    //     &self.materials
    // }
}