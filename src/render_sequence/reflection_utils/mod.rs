use spirv_reflect::types::traits::ReflectNumericTraits;

pub mod format;

/// Returns the size of a ReflectNumericTrait
///
/// # Arguments
///
/// * 'reflect_numeric_trait' - A structure describing the numeric attributes of a shader variable
///
pub fn size_of_reflect_numeric_trait(reflect_numeric_trait: &ReflectNumericTraits) -> u32
{
    let scalar_size = reflect_numeric_trait.scalar.width / 8;
    // TODO: Check this matrix size calculation
    if reflect_numeric_trait.matrix.column_count != 0
    {
        reflect_numeric_trait.matrix.stride * reflect_numeric_trait.matrix.row_count
    }
    else if reflect_numeric_trait.vector.component_count != 0
    {
        scalar_size * reflect_numeric_trait.vector.component_count
    }
    else
    {
        scalar_size
    }
}
