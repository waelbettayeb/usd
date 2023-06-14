type Size = usize;

struct Vec2<T = Size> {
    x: T,
    y: T,
}

type Vec3<T = Size> = [T; 3];
type Vec4<T = Size> = [T; 4];

pub type Vec3f = Vec3<f32>;
pub type Vec2f = Vec2<f32>;
pub type Vec4f = Vec4<f32>;

type Point = Vec3;

struct Vertex {
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
}

struct Edge {
    points: [Point; 2],
}

struct Face {
    points: Vec<Point>,
}

pub struct Mesh {
    points: Vec<Point>,
    edges: Vec<Edge>,
    faces: Vec<Face>,
}

impl Mesh {
    pub fn new() -> Mesh {
        return Mesh {
            points: vec![[0, 0, 0]],
            edges: Vec::new(),
            faces: Vec::new(),
        };
    }
}
