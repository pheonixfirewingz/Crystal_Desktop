use nalgebra::{Matrix4, Vector2, Vector3};

pub(crate) fn create_model_matrix(position: Vector3<f32>, scale: Vector2<f32>) -> Matrix4<f32> {
    let translation = Matrix4::new_translation(&position);
    let scaling = Matrix4::new_nonuniform_scaling(&Vector3::new(scale.x,scale.y, 1.0));
    translation * scaling
}

/// Creates an orthographic projection matrix
pub(crate) fn create_ortho_matrix(screen_width: f32, screen_height: f32) -> Matrix4<f32> {
    // This creates an orthographic projection matrix that maps screen space to normalized device coordinates
    Matrix4::new_orthographic(0.0, screen_width, screen_height, 0.0, -1.0, 1.0)
}