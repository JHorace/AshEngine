#version 450

layout(location = 0) in vec3 vertPosition;
layout(location = 1) in vec3 vertNormal;
layout(location = 2) in vec3 vertColor;
layout(location = 3) in vec2 vertUV;

layout(binding = 0) uniform UBO {
    mat4 transform;
};

layout(push_constant) uniform ViewProjection {
    mat4 view;
    mat4 projection;
};

layout(location = 0) out vec3 fragColor;

void main() {
    gl_Position = projection * view * transform * vec4(vertPosition, 1.0);
    fragColor = vec3(1.0, 0.0, 0.0);
}
