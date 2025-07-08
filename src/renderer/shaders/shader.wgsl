
struct CameraUniform {
    view_proj: mat4x4<f32>,
    position: vec3<f32>,
    _padding: f32
}

struct PointLightUniform {
    position: vec3<f32>,
    intensity: f32,
    color: vec3<f32>,
    _padding: f32
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> point_light: PointLightUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tangent: vec3<f32>,
    @location(4) bitangent: vec3<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) tangent_position: vec3<f32>,
    @location(2) tangent_light_position: vec3<f32>,
    @location(3) tangent_view_position: vec3<f32>,
};

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
    @location(9) normal_matrix_0: vec3<f32>,
    @location(10) normal_matrix_1: vec3<f32>,
    @location(11) normal_matrix_2: vec3<f32>,
    
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
    let normal_mat = mat3x3<f32> (
        instance.normal_matrix_0,
        instance.normal_matrix_1,
        instance.normal_matrix_2
    );
    let world_normal = normalize(normal_mat * model.normal);
    let world_tangent = normalize(normal_mat * model.tangent);
    let world_bitangent = normalize(normal_mat * model.bitangent);
    let tangent_matrix = transpose(mat3x3<f32>(
        world_tangent,
        world_bitangent,
        world_normal,
    ));

    let world_position = model_mat * vec4<f32>(model.position, 1.0);

    var out: VertexOutput;
    out.clip_position = camera.view_proj * world_position;
    out.tex_coords = model.tex_coords;
    out.tangent_position = tangent_matrix * world_position.xyz;
    out.tangent_view_position = tangent_matrix * camera.position.xyz;
    out.tangent_light_position = tangent_matrix * point_light.position;
    return out;
}

@group(2) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(2) @binding(1)
var s_diffuse: sampler;
@group(2) @binding(2)
var t_normal: texture_2d<f32>;
@group(2) @binding(3)
var s_normal: sampler;


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texture_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let normal = textureSample(t_normal, s_normal, in.tex_coords);
    let tangent_normal = normal.xyz * 2.0 - 1.0;
    let light_dir = normalize(in.tangent_light_position - in.tangent_position);
    let view_dir = normalize(in.tangent_view_position - in.tangent_position);
    let dist = distance(in.tangent_light_position, in.tangent_position);
    let attenuation = 1.0 / (dist * dist);
    let ambient = 0.05;
    let diffuse = max(dot(tangent_normal, light_dir), 0.0);
    var specular = 0.0;
    if (diffuse > 0.0) {
        let half_dir = normalize(light_dir + view_dir);
        let spec_angle = max(dot(tangent_normal, half_dir), 0.0);
        specular = pow(spec_angle, 16.0);
    }
    let specular_color = point_light.color * specular;
    let diffuse_color = point_light.color * diffuse;
    let ambient_color = point_light.color * ambient;
    let color = texture_color.xyz * (ambient_color + diffuse_color + specular_color) * attenuation;
    return vec4<f32>(color, 1.0);
}
