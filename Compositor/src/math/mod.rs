use nalgebra::{Matrix4, Vector2, Vector3};
pub fn create_model_matrix(position: Vector3<f32>, scale: Vector2<f32>) -> Matrix4<f32> {
    let translation = Matrix4::new_translation(&position);
    let scaling = Matrix4::new_nonuniform_scaling(&Vector3::new(scale.x,scale.y, 1.0));
    translation * scaling
}