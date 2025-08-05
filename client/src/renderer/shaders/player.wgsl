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

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
    @location(9) normal_matrix_0: vec3<f32>,
    @location(10) normal_matrix_1: vec3<f32>,
    @location(11) normal_matrix_2: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
};

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
    var out: VertexOutput;
    let world_position = model_mat * vec4<f32>(model.position, 1.0);
    out.world_position = world_position.xyz;
    out.world_normal = normal_mat * model.normal;
    out.clip_position = camera.view_proj * world_position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
let material_color = vec3<f32>(1.0, 0.5, 0.5);
    let ambient_strength = 0.1;
    let specular_strength = 0.5;
    let shininess = 32.0;
    
    let normal = normalize(in.world_normal);
    let view_dir = normalize(camera.view_pos.xyz - in.world_position);
    
    var final_color = ambient_strength * material_color;
    
    for (var i = 0u; i < point_lights.count; i++) {
        let light_pos = point_lights.lights[i].position;
        let light_color = point_lights.lights[i].color;
        let light_intensity = point_lights.lights[i].intensity;
        
        let light_dir = normalize(light_pos - in.world_position);
        let light_dist = distance(light_pos, in.world_position);
        
        let attenuation = light_intensity / (1.0 + 0.09 * light_dist + 0.032 * light_dist * light_dist);
        
        let diffuse = max(dot(normal, light_dir), 0.0);
        let diffuse_contribution = diffuse * material_color * light_color;
        
        var specular_contribution = vec3<f32>(0.0);
        if (diffuse > 0.0) {
            let half_dir = normalize(light_dir + view_dir);
            let spec_angle = max(dot(normal, half_dir), 0.0);
            let spec = pow(spec_angle, shininess);
            specular_contribution = specular_strength * spec * light_color;
        }
        
        final_color += (diffuse_contribution + specular_contribution) * attenuation;
    }
    
    return vec4<f32>(final_color, 1.0);
}

