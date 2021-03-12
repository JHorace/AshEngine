use spirv_reflect::types::{ReflectDescriptorBinding, ReflectDescriptorSet, ReflectDescriptorType};

pub enum DescriptorType {
    Undefined,
    Sampler,
    CombinedImageSampler,
    SampledImage,
    StorageImage,
    UniformTexelBuffer,
    StorageTexelBuffer,
    UniformBuffer,
    StorageBuffer,
    UniformBufferDynamic,
    StorageBufferDynamic,
    InputAttachment,
    AccelerationStructureNV,
}

impl DescriptorType {
    pub fn from_reflect_type(reflect_type: &ReflectDescriptorType) -> DescriptorType {
        match reflect_type {
            ReflectDescriptorType::Undefined => DescriptorType::Undefined,
            ReflectDescriptorType::Sampler => DescriptorType::Sampler,
            ReflectDescriptorType::CombinedImageSampler => DescriptorType::CombinedImageSampler,
            ReflectDescriptorType::SampledImage => DescriptorType::SampledImage,
            ReflectDescriptorType::StorageImage => DescriptorType::StorageImage,
            ReflectDescriptorType::UniformTexelBuffer => DescriptorType::UniformTexelBuffer,
            ReflectDescriptorType::StorageTexelBuffer => DescriptorType::StorageTexelBuffer,
            ReflectDescriptorType::UniformBuffer => DescriptorType::UniformBuffer,
            ReflectDescriptorType::StorageBuffer => DescriptorType::StorageBuffer,
            ReflectDescriptorType::UniformBufferDynamic => DescriptorType::UniformBufferDynamic,
            ReflectDescriptorType::StorageBufferDynamic => DescriptorType::StorageBufferDynamic,
            ReflectDescriptorType::InputAttachment => DescriptorType::InputAttachment,
            ReflectDescriptorType::AccelerationStructureNV => {
                DescriptorType::AccelerationStructureNV
            }
        }
    }
}

pub struct DescriptorSetDescription {
    pub set_: u32,
    pub descriptors_: Vec<DescriptorDescription>,
}

impl DescriptorSetDescription {
    pub fn from_reflections(
        reflect_descriptor_sets: &Vec<ReflectDescriptorSet>,
    ) -> Vec<DescriptorSetDescription> {
        let mut descriptor_sets = vec![];

        for reflect_descriptor_set in reflect_descriptor_sets.iter() {
            descriptor_sets.push(DescriptorSetDescription::from_reflection(
                reflect_descriptor_set));
        }

        descriptor_sets
    }

    pub fn from_reflection(
        reflect_descriptor_set: &ReflectDescriptorSet,
    ) -> DescriptorSetDescription {
        let mut descriptors = vec![];

        for descriptor in reflect_descriptor_set.bindings.iter() {
            descriptors.push(DescriptorDescription::from_reflection(descriptor));
        }

        DescriptorSetDescription {
            set_: reflect_descriptor_set.set,
            descriptors_: descriptors,
        }
    }
}

pub struct DescriptorDescription {
    pub binding_: u32,
    pub descriptor_type_: DescriptorType,
    pub count_: u32,
    pub size_: u32,
    pub padded_size_: u32,
}

impl DescriptorDescription {
    /// TODO: Make this work with images and arrays
    pub fn from_reflection(descriptor: &ReflectDescriptorBinding) -> DescriptorDescription {
        DescriptorDescription {
            binding_: descriptor.binding,
            descriptor_type_: DescriptorType::from_reflect_type(&descriptor.descriptor_type),
            count_: descriptor.count,
            size_: descriptor.block.size,
            padded_size_: descriptor.block.padded_size,
        }
    }
}
