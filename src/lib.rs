use godot::{
    classes::{ArrayMesh, mesh::PrimitiveType},
    prelude::*,
};

use sprinting_cubes::marching_cubes::{Mesh, marching_cubes};

// Godot bindings do not export these consts for the indexes on 3DMeshArray
const MESH_ARRAY_MAX: usize = 13;
const ARRAY_VERTEX: usize = 0;
const ARRAY_NORMAL: usize = 1;
const ARRAY_INDEX: usize = 12;

struct MarchingCubesExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MarchingCubesExtension {}

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct MarchingCubesGenerator {
    #[base]
    base: Base<RefCounted>,
}

#[godot_api]
impl MarchingCubesGenerator {
    #[func]
    pub fn new() -> Gd<Self> {
        Gd::from_init_fn(|base| Self { base })
    }

    /// Generate a mesh from a 3D scalar field
    ///
    /// # Arguments
    /// * `data` - Flattened 3D array of scalar values
    /// * `width` - Width of the 3D grid
    /// * `height` - Height of the 3D grid  
    /// * `depth` - Depth of the 3D grid
    /// * `isolevel` - Surface threshold value
    /// * `scale` - Scale factor for vertex positions (default 1.0)
    #[func]
    pub fn generate_mesh(
        &self,
        data: PackedFloat32Array,
        width: i32,
        height: i32,
        depth: i32,
        isolevel: f32,
        scale: f32,
    ) -> Option<Gd<ArrayMesh>> {
        // Convert Godot types to Rust types
        let data_slice: &[f32] = data.as_slice();
        let dimensions = (width as usize, height as usize, depth as usize);

        // Validate input
        if data_slice.len() != width as usize * height as usize * depth as usize {
            godot_error!("Data array size doesn't match dimensions");
            return None;
        }

        // Generate marching cubes mesh
        let mesh = marching_cubes(data_slice, dimensions, isolevel);

        if mesh.vertices.is_empty() {
            godot_warn!("No vertices generated - check your isolevel and data");
            return None;
        }

        // Convert to Godot mesh
        Some(self.create_godot_mesh(&mesh, scale))
    }

    /// Generate a simple test sphere mesh
    #[func]
    pub fn generate_sphere(&self, size: i32, radius: f32, scale: f32) -> Gd<ArrayMesh> {
        let dimensions = (size as usize, size as usize, size as usize);
        let mut data = vec![0.0; size as usize * size as usize * size as usize];

        let center = size as f32 / 2.0;

        // Generate sphere data
        for z in 0..size {
            for y in 0..size {
                for x in 0..size {
                    let dx = x as f32 - center;
                    let dy = y as f32 - center;
                    let dz = z as f32 - center;
                    let distance = (dx * dx + dy * dy + dz * dz).sqrt();

                    let index = (z * size * size + y * size + x) as usize;
                    data[index] = radius - distance;
                }
            }
        }

        let mesh = marching_cubes(&data, dimensions, 0.0);
        self.create_godot_mesh(&mesh, scale)
    }

    /// Generate terrain from a height function
    #[func]
    pub fn generate_terrain(
        &self,
        width: i32,
        height: i32,
        depth: i32,
        scale: f32,
        noise_scale: f32,
        height_scale: f32,
    ) -> Gd<ArrayMesh> {
        let dimensions = (width as usize, height as usize, depth as usize);
        let mut data = vec![0.0; width as usize * height as usize * depth as usize];

        // Simple terrain generation (you could integrate with Godot's noise here)
        for z in 0..depth {
            for y in 0..height {
                for x in 0..width {
                    let nx = x as f32 * noise_scale;
                    let nz = z as f32 * noise_scale;

                    // Simple height function (replace with proper noise)
                    let terrain_height = ((nx * 0.1).sin() + (nz * 0.1).cos()) * height_scale;
                    let world_y = y as f32;

                    let density = terrain_height - world_y;

                    let index = (z * height * width + y * width + x) as usize;
                    data[index] = density;
                }
            }
        }

        let mesh = marching_cubes(&data, dimensions, 0.0);
        self.create_godot_mesh(&mesh, scale)
    }

    fn create_godot_mesh(&self, mesh: &Mesh, scale: f32) -> Gd<ArrayMesh> {
        let mut array_mesh = ArrayMesh::new_gd();
        let mut arrays = VariantArray::new();
        arrays.resize(MESH_ARRAY_MAX, &Variant::nil());

        // Convert vertices
        let mut vertices = PackedVector3Array::new();
        for vertex in &mesh.vertices {
            vertices.push(Vector3::new(
                vertex[0] * scale,
                vertex[1] * scale,
                vertex[2] * scale,
            ));
        }

        // Convert triangle indices
        let mut indices = PackedInt32Array::new();
        for triangle in &mesh.triangles {
            indices.push(triangle[0] as i32);
            indices.push(triangle[1] as i32);
            indices.push(triangle[2] as i32);
        }

        // Calculate normals (proper vertex normal accumulation)
        let mut vertex_normals = vec![Vector3::ZERO; mesh.vertices.len()];

        // Calculate face normals and accumulate to vertices
        for triangle in &mesh.triangles {
            let v0 = Vector3::new(
                mesh.vertices[triangle[0]][0],
                mesh.vertices[triangle[0]][1],
                mesh.vertices[triangle[0]][2],
            );
            let v1 = Vector3::new(
                mesh.vertices[triangle[1]][0],
                mesh.vertices[triangle[1]][1],
                mesh.vertices[triangle[1]][2],
            );
            let v2 = Vector3::new(
                mesh.vertices[triangle[2]][0],
                mesh.vertices[triangle[2]][1],
                mesh.vertices[triangle[2]][2],
            );

            let face_normal = (v1 - v0).cross(v2 - v0);

            // Accumulate face normal to each vertex
            vertex_normals[triangle[0]] += face_normal;
            vertex_normals[triangle[1]] += face_normal;
            vertex_normals[triangle[2]] += face_normal;
        }

        // Normalize accumulated normals and convert to PackedVector3Array
        let mut normals = PackedVector3Array::new();
        for normal in vertex_normals {
            normals.push(normal.normalized());
        }

        // Set arrays
        arrays.set(ARRAY_VERTEX, &vertices.to_variant());
        arrays.set(ARRAY_NORMAL, &normals.to_variant());
        arrays.set(ARRAY_INDEX, &indices.to_variant());

        // Create the mesh surface
        array_mesh.add_surface_from_arrays(PrimitiveType::TRIANGLES, &arrays);

        array_mesh
    }
}

#[godot_api]
impl IRefCounted for MarchingCubesGenerator {
    fn init(base: Base<RefCounted>) -> Self {
        Self { base }
    }
}
