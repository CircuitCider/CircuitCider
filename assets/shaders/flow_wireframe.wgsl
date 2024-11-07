// #import bevy_pbr::forward_io::VertexOutput

struct WireframeMaterial {
    color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    // @location(0) world_position: vec4<f32>,
    // @location(1) world_normal: vec3<f32>,
    // @location(2) uv: vec2<f32>,
    // @location(3) uv_b: vec2<f32>,
    // @location(4) world_tangent: vec4<f32>,
    @location(0) color: vec4<f32>,
    // @location(6) @interpolate(flat) instance_index: u32
}


@group(2) @binding(0)
var<uniform> material: WireframeMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {

    var color = vec4(0.0, 0.0, 0.0, 1.0);
    var black = vec4(0.0, 0.0, 0.0, 1.0);
    
    color = vec4(in.color.xyz, 1.0);
    
    if in.color.y < 0.4 {
        
        //if in.color.y > 0.5 {
        color = black;

        //}
        //if in.color.y > 1.0 {
        //}
    }
    if in.color.y > 0.5 {
        color = black;
    }
    // if in.color.y < 1.0 {
    //     color = black;
    // }
    // if in.color.x > 1.0 {
    //     color = vec4(1.0, 0.0, 0.0, 1.0);
    // }
    // else if in.color.y > 1.0 {
    //     color = vec4(0.0, 1.0, 0.0, 1.0);
    // }
    // else if in.color.z > 1.0 {
    //     color = vec4(0.0, 0.0, 1.0, 1.0);
    // }
    return color;
}