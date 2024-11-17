// // #import bevy_pbr::forward_io::VertexOutput



// fn pristineGrid(uv: vec2<f32>, line_width: vec3<f32>) -> f32
// {
//     var ddx = dpdx(uv);
//     var ddy = dpdy(uv);
//     var uvDeriv = vec2(length(vec2(ddx.x, ddy.x)), length(vec2(ddx.y, ddy.y)));
//     var invertLine = vec2(line_width.x > 0.5, line_width.y > 0.5);
//     // var targetWidth = vec2(
//     //   invertLine.x ? 1.0 - lineWidth.x : lineWidth.x,
//     //   invertLine.y ? 1.0 - lineWidth.y : lineWidth.y
//     //   );
//     var targetWidth = vec2(
//         select(
//             line_width.x,
//             1.0 - line_width.x,
//             invertLine.x
//         ),
//         select(
//             line_width.y,
//             1.0 - line_width.y,
//             invertLine.y
//         ),
//     );
    
//     var drawWidth = clamp(targetWidth, uvDeriv, vec2(0.5));
//     var lineAA = uvDeriv * 1.5;
//     var gridUV = abs(fract(uv) * 2.0 - 1.0);
//     // gridUV.x = invertLine.x ? gridUV.x : 1.0 - gridUV.x;
//     gridUV.x = select(
//         1.0 - gridUV.x,
//         gridUV.x,
//         invertLine.x
//     );

//     // gridUV.y = invertLine.y ? gridUV.y : 1.0 - gridUV.y;
//     gridUV.y = select(
//         1.0 - gridUV.y,
//         gridUV.y,
//         invertLine.y
//     );
//     var grid2 = smoothstep(drawWidth + lineAA, drawWidth - lineAA, gridUV);

//     grid2 *= clamp(targetWidth / drawWidth, vec2(0.0, 0.0), vec2(1.0, 1.0));
//     grid2 = mix(grid2, targetWidth, clamp(uvDeriv * 2.0 - 1.0, vec2(0.0, 0.0), vec2(1.0, 1.0)));
//     grid2.x = select(
//         grid2.x,
//         1.0 - grid2.x,
//         invertLine.x
//     );
//     grid2.y = select(
//         grid2.y,
//         1.0 - grid2.y,
//         invertLine.y
//     );
//     // grid2.x = invertLine.x ? 1.0 - grid2.x : grid2.x;
//     // grid2.y = invertLine.y ? 1.0 - grid2.y : grid2.y;
//     return mix(grid2.x, 1.0, grid2.y);
// }

// struct Globals {
//     time: f32,
//     delta_time: f32,
//     frame_count: u32
// }
// @group(0) @binding(0)
// var<uniform> globals: Globals;

// struct GridMaterial {
//     color: vec4<f32>,
// };

// struct VertexOutput {
//     @builtin(position) position: vec4<f32>,
//     // @location(0) world_position: vec4<f32>,
//     // @location(1) world_normal: vec3<f32>,
//     @location(2) uv: vec2<f32>,
//     // @location(3) uv_b: vec2<f32>,
//     // @location(4) world_tangent: vec4<f32>,
//     @location(0) color: vec4<f32>,
//     // @location(6) @interpolate(flat) instance_index: u32
// }


// @group(2) @binding(0)
// var<uniform> material: GridMaterial;

// @fragment
// fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {

//     var color = vec4(0.0, 0.0, 0.0, 1.0);
//     var black = vec4(0.0, 0.0,   0.0, 1.0);
    
//     var mid=  0;

//     // let test = texture_2d(in.position.xy)
//     vec2 uvw = textureDimensions( texture_2d( in.position.xy), mid );

//     // calc texture sampling footprint		
//     vec2 ddx_uvw = dFdx( uvw ); 
//     vec2 ddy_uvw = dFdy( uvw );

//     //var mate = pristineGrid(in.uv, vec3(0.5, 0.5, 0.5));
//     var mate = vec3(0.0);

//     mate = vec3(1.0)*(1.0 - pristineGrid( in.uv - vec2(0.05), ddx_uv, ddy_uv, vec2(1.0/N) ));
//     color = vec4(mate, 1.0);
//     return color;
// }