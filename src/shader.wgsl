struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    
    // Hardcoded positions for a standard flat triangle (X, Y)
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),    // Top vertex
        vec2<f32>(-0.5, -0.5),  // Bottom-left vertex
        vec2<f32>(0.5, -0.5)    // Bottom-right vertex
    );
    
    // RGB color attributes corresponding to each vertex
    var colors = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.0, 0.0), // Red
        vec3<f32>(0.0, 1.0, 0.0), // Green
        vec3<f32>(0.0, 0.0, 1.0)  // Blue
    );

    out.clip_position = vec4<f32>(positions[in_vertex_index], 0.0, 1.0);
    out.color = colors[in_vertex_index];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Return final fragments interpolated over the triangle mesh surface
    return vec4<f32>(in.color, 1.0);
}
