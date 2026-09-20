use thiserror::Error;

#[derive(Debug, Error)]
pub enum RendererError {
    #[error("No suitable adapter found")]
    NoAdapter,
    #[error("Failed to create GPU device: {0}")]
    Device(#[source] wgpu::RequestDeviceError),
    #[error("failed to create surface: {0}")]
    Surface(#[source] wgpu::CreateSurfaceError),
}
