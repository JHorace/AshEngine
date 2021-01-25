use super::vertex::Vertex;
use cgmath::Vector3;
use cgmath::prelude::*;
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Mesh
{
    pub vertices_: Vec<Vertex>,
    pub indices_: Vec<u32>,
}

impl Mesh
{
    pub fn from_tobj_mesh(tobj_mesh: &tobj::Mesh) -> Mesh
    {
        let num_vertices = tobj_mesh.positions.len() / 3;

        let mut vertices = vec![];
        let mut indices = vec![];

        for i in 0..num_vertices
        {
            let vertex = Vertex{
                position_: [
                    tobj_mesh.positions[i * 3],
                    tobj_mesh.positions[i * 3 + 1],
                    tobj_mesh.positions[i * 3 + 2],
                ],
                normal_: if tobj_mesh.normals.is_empty(){[0.0, 0.0, 0.0]}
                else{ [tobj_mesh.normals[i * 3], tobj_mesh.normals[i * 3 + 1], tobj_mesh.normals[i * 3 + 2]]},
                color_: [1.0, 1.0, 1.0],
                uv_: if tobj_mesh.texcoords.is_empty() { [0.0, 0.0] }
                else { [tobj_mesh.texcoords[i * 2], tobj_mesh.texcoords[i * 2 + 1]] }
            };
            vertices.push(vertex);
        }

        indices = tobj_mesh.indices.clone();

        if tobj_mesh.normals.is_empty()
        {
            Mesh::generate_vertex_normals(& mut vertices, &indices);
        }


        Mesh{ vertices_: vertices, indices_: indices }
    }

    fn generate_vertex_normals(vertices: &mut Vec<Vertex>, indices: &Vec<u32>)
    {
        for face in indices.chunks(3)
        {
            let vert_a = vertices[face[0] as usize].position_;
            let vert_b = vertices[face[1] as usize].position_;
            let vert_c = vertices[face[2] as usize].position_;
            let vect_a = Vector3::new(vert_a[0], vert_a[1], vert_a[2]);
            let vect_b = Vector3::new(vert_b[0], vert_b[1], vert_b[2]);
            let vect_c = Vector3::new(vert_c[0], vert_c[1], vert_c[2]);

            let p = Vector3::cross(vect_b - vect_a, vect_c - vect_a);


            vertices[face[0] as usize].normal_[0] += p.x;
            vertices[face[0] as usize].normal_[1] += p.y;
            vertices[face[0] as usize].normal_[2] += p.z;

            vertices[face[1] as usize].normal_[0] += p.x;
            vertices[face[1] as usize].normal_[1] += p.y;
            vertices[face[1] as usize].normal_[2] += p.z;

            vertices[face[2] as usize].normal_[0] += p.x;
            vertices[face[2] as usize].normal_[1] += p.y;
            vertices[face[2] as usize].normal_[2] += p.z;
        }
    }
}