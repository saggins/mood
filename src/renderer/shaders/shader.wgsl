struct CameraUniform {
    view_pos: vec4<f32>,
    view: mat4x4<f32>,
    view_proj: mat4x4<f32>,
    inv_proj: mat4x4<f32>,
    inv_view: mat4x4<f32>,
}

struct LightUniform {
    position: vec3<f32>,
    _padding: f32,
    color: vec3<f32>,
    intensity: f32,
}

struct Lights {
    lights: array<LightUniform, 32>,
    count: u32,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> point_lights: Lights;

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
    @location(2) tangent_view_position: vec3<f32>,
    @location(3) T: vec3<f32>,
    @location(4) B: vec3<f32>,
    @location(5) N: vec3<f32>
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
    var out: VertexOutput;
    out.T = world_tangent;
    out.B = world_bitangent;
    out.N = world_normal;

    let world_position = model_mat * vec4<f32>(model.position, 1.0);

    out.clip_position = camera.view_proj * world_position;
    out.tex_coords = model.tex_coords;
    out.tangent_position = tangent_matrix * world_position.xyz;
    out.tangent_view_position = tangent_matrix * camera.view_pos.xyz;
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
    var color = vec3<f32>(0.0);
    let normal = textureSample(t_normal, s_normal, in.tex_coords);
    let tangent_normal = normal.xyz * 2.0 - 1.0;
    let view_dir = normalize(in.tangent_view_position - in.tangent_position);
    let tangent_matrix = transpose(mat3x3<f32>(
        in.T,
        in.B,
        in.N
    ));
    
    for (var i = 0u; i < point_lights.count; i++) {
        let light_pos = tangent_matrix * point_lights.lights[i].position;
        let light_color = point_lights.lights[i].color;
        let light_intensity = point_lights.lights[i].intensity;
        let light_dir = normalize(light_pos - in.tangent_position);
        let dist = distance(light_pos, in.tangent_position);
        let attenuation = 1.0 / (dist * dist);
        let diffuse = max(dot(tangent_normal, light_dir), 0.0);
        var specular = 0.0;
        
        if (diffuse > 0.0) {
            let half_dir = normalize(light_dir + view_dir);
            let spec_angle = max(dot(tangent_normal, half_dir), 0.0);
            specular = pow(spec_angle, 16.0);
        }
        
        color += light_color * (specular + diffuse) * attenuation * light_intensity;
    }
    
    let texture_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let frag_color = texture_color.xyz * color;
    return vec4<f32>(frag_color, 1.0);
}
