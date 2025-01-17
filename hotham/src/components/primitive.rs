use crate::{buffer::Buffer, resources::VulkanContext, Vertex};
use anyhow::{anyhow, Result};
use ash::vk;
use itertools::izip;
use nalgebra::vector;

use super::Material;

#[derive(Debug, Clone, PartialEq)]
pub struct Primitive {
    pub index_buffer: Buffer<u32>,
    pub vertex_buffer: Buffer<Vertex>,
    pub indicies_count: u32,
    pub material: Material,
    pub texture_descriptor_set: vk::DescriptorSet,
}

impl Primitive {
    pub fn load(
        textures_layout: vk::DescriptorSetLayout,
        mesh_name: &str,
        primitive_data: gltf::Primitive,
        buffer: &[u8],
        vulkan_context: &VulkanContext,
        images: &Vec<gltf::image::Data>,
    ) -> Result<Self> {
        let mut indices = Vec::new();
        let mut positions = Vec::new();
        let mut tex_coords_0 = Vec::new();
        let mut tex_coords_1 = Vec::new();
        let mut normals = Vec::new();
        let mut joint_indices = Vec::new();
        let mut joint_weights = Vec::new();

        let reader = primitive_data.reader(|_| Some(buffer));

        // Positions
        for v in reader
            .read_positions()
            .ok_or(anyhow!("Mesh {} has no positions!", mesh_name))?
        {
            positions.push(vector![v[0], v[1], v[2]]);
        }

        // Indices
        if let Some(iter) = reader.read_indices() {
            for i in iter.into_u32() {
                indices.push(i);
            }
        }

        // Normals
        if let Some(iter) = reader.read_normals() {
            for v in iter {
                normals.push(vector![v[0], v[1], v[2]]);
            }
        } else {
            for _ in 0..positions.len() {
                normals.push(vector![0., 0., 0.]);
            }
        }

        if let Some(iter) = reader.read_tex_coords(0) {
            for v in iter.into_f32() {
                tex_coords_0.push(vector![v[0], v[1]]);
            }
        } else {
            for _ in 0..positions.len() {
                tex_coords_0.push(vector![0., 0.]);
            }
        }

        if let Some(iter) = reader.read_tex_coords(1) {
            for v in iter.into_f32() {
                tex_coords_1.push(vector![v[0], v[1]]);
            }
        } else {
            for _ in 0..positions.len() {
                tex_coords_1.push(vector![0., 0.]);
            }
        }

        if let Some(iter) = reader.read_joints(0) {
            for t in iter.into_u16() {
                joint_indices.push(vector![t[0] as f32, t[1] as f32, t[2] as f32, t[3] as f32]);
            }
        } else {
            for _ in 0..positions.len() {
                joint_indices.push(vector![0., 0., 0., 0.]);
            }
        }

        if let Some(iter) = reader.read_weights(0) {
            for t in iter.into_f32() {
                joint_weights.push(vector![t[0] as f32, t[1] as f32, t[2] as f32, t[3] as f32]);
            }
        } else {
            for _ in 0..positions.len() {
                joint_weights.push(vector![0., 0., 0., 0.]);
            }
        }

        let (material, texture_descriptor_set) = Material::load(
            mesh_name,
            textures_layout,
            primitive_data.material(),
            vulkan_context,
            buffer,
            images,
        )?;

        let vertices: Vec<Vertex> = izip!(
            positions,
            normals,
            tex_coords_0,
            tex_coords_1,
            joint_indices,
            joint_weights
        )
        .into_iter()
        .map(Vertex::from_zip)
        .collect();

        // Create buffers
        let vertex_buffer = Buffer::new(
            &vulkan_context,
            &vertices,
            vk::BufferUsageFlags::VERTEX_BUFFER,
        )?;
        let index_buffer = Buffer::new(
            &vulkan_context,
            &indices,
            vk::BufferUsageFlags::INDEX_BUFFER,
        )?;

        Ok(Primitive {
            material,
            index_buffer,
            vertex_buffer,
            indicies_count: indices.len() as _,
            texture_descriptor_set,
        })
    }
}
