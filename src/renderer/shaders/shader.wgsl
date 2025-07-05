
struct CameraUniform {
    view_proj: mat4x4<f32>
}

struct PointLightUniform {
    position: vec3<f32>,
    intensity: f32,
    color: vec4<f32>
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> point_light: PointLightUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>
};

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
    
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    let model_mat = mat4x4<f32> (
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3
    );
    let normal_mat = transpose(inverse(mat3x3<f32> (
        model_mat[0].xyz,
        model_mat[1].xyz,
        model_mat[2].xyz
    )));
    
    var out: VertexOutput;
    out.color = model.color;
    let world_position = model_mat * vec4<f32>(model.position, 1.0);
    out.clip_position = camera.view_proj * world_position;
    out.world_position = world_position.xyz;
    out.world_normal = normalize(normal_mat * model.normal);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_dir = normalize(point_light.position - in.world_position);
    let ambient = 0.05;
    let diffuse = clamp(dot(in.world_normal, light_dir), 0.0, 1.0) * 1.5;
    let specular = 0.0;
    let color = in.color * (ambient + diffuse + specular);
    return vec4<f32>(color, 1.0);
}

fn _reflected(incident: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    return incident - 2 * normal * dot(incident, normal);
}

fn inverse(m: mat3x3<f32>) -> mat3x3<f32> {
    let a00 = m[0][0]; let a01 = m[0][1]; let a02 = m[0][2];
    let a10 = m[1][0]; let a11 = m[1][1]; let a12 = m[1][2];
    let a20 = m[2][0]; let a21 = m[2][1]; let a22 = m[2][2];

    let b01 = a22 * a11 - a12 * a21;
    let b11 = -a22 * a10 + a12 * a20;
    let b21 = a21 * a10 - a11 * a20;

    let det = a00 * b01 + a01 * b11 + a02 * b21;

    return mat3x3<f32>(
        vec3<f32>(b01, -a22 * a01 + a02 * a21, a12 * a01 - a02 * a11),
        vec3<f32>(b11, a22 * a00 - a02 * a20, -a12 * a00 + a02 * a10),
        vec3<f32>(b21, -a21 * a00 + a01 * a20, a11 * a00 - a01 * a10)
    ) * (1.0 / det);
}

