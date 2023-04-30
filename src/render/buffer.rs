use wgpu::util::DeviceExt;

use crate::systems::{GameState, GRID_SIZE};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2]
}



const TILE_VERTS: [Vertex; 4] =  [
    Vertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], }, // Top right
    Vertex { position: [-1.0, 1.0, 0.0], tex_coords: [0.0, 0.0], }, // Top left
    Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [0.0, 1.0], }, // Bottom left
    Vertex { position: [1.0, -1.0, 0.0], tex_coords: [1.0, 1.0], }, // Bottom right
];

const TILE_INDIS: [u16; 6] = [
    0, 1, 2,
    0, 2, 3
];

impl Vertex {

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2
                }
            ]
        }
    }
}

pub fn create_buffers(device: &wgpu::Device, state: &GameState) -> (Option<wgpu::Buffer>, Option<wgpu::Buffer>, usize) {

    let mut verts : Vec<Vertex> = vec![];
    let mut indis : Vec<u16> = vec![];

    for x in 0..GRID_SIZE[0] {
        for y in 0..GRID_SIZE[1] {
            if state.board[x as usize][y as usize] {
                create_tile([x, y], &mut verts, &mut indis);
            }
        }
    }

    for (y, row) in state.tetrimino.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val { create_tile([x as i32 + state.pos[0], y as i32+ state.pos[1]], &mut verts, &mut indis); }
        }
    }

    let vertex_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&verts),
            usage: wgpu::BufferUsages::VERTEX
        }
    );

    let index_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indis),
            usage: wgpu::BufferUsages::INDEX
        }
    );

    (Some(vertex_buffer), Some(index_buffer), indis.len())
}

fn create_tile(pos: [i32; 2], verts: &mut Vec<Vertex>, indis: &mut Vec<u16>) {
    let mut tile_verts : Vec<Vertex> = TILE_VERTS.iter()
        .map(|v| Vertex {
            position: {
                [(v.position[0] + 1.0 - GRID_SIZE[0] as f32 + (pos[0] * 2) as f32) / GRID_SIZE[0] as f32, 
                (v.position[1] + 1.0 - GRID_SIZE[1] as f32 + (pos[1] * 2) as f32) / GRID_SIZE[1] as f32, 
                v.position[2]]
            },
            tex_coords: v.tex_coords
        })
        .collect();

    let mut tile_indis : Vec<u16> = TILE_INDIS.iter()
        .map(|i| i + verts.len() as u16)
        .collect();

    verts.append(&mut tile_verts);
    indis.append(&mut tile_indis);
}