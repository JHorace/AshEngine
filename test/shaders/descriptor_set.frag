#version 450

layout(location = 0) in vec3 fragColor;
layout(location = 0) out vec4 outColor;
layout(location = 1) out vec4 outPosition;

layout (input_attachment_index = 0, binding = 0) uniform subpassInput samplerposition;
layout (input_attachment_index = 1, binding = 1) uniform subpassInput samplerNormal;
layout (input_attachment_index = 2, binding = 2) uniform subpassInput samplerAlbedo;

layout(binding = 4) uniform UBO {
    mat4 transform;
};

void main() {
    outColor = vec4(fragColor, 1.0);
    outPosition = vec4(1.0, 1.0, 1.0, 1.0);
}
