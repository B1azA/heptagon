pub struct RenderPipeline {
    render_pipeline: wgpu::RenderPipeline,
}

impl RenderPipeline {
    // pub fn new(device: &wgpu::Device, shader: &str) -> Self {
    //     let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    //         label: Some("Shader"),
    //         source: wgpu::ShaderSource::Wgsl(shader.into()),
    //     });

    //     let texture_bind_group_layout = Texture::bind_group_layout(&device);
 
    //     let mvp_bind_group_layout = Mat4Uniform::bind_group_layout(&device);

    //     let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    //         label: Some("Render Pipeline Layout"),
    //         bind_group_layouts: &[
    //             &texture_bind_group_layout,
    //             &mvp_bind_group_layout,
    //         ],
    //         push_constant_ranges: &[],
    //     });
    // }
}