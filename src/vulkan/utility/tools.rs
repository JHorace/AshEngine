use std::ffi::CStr;
use std::os::raw::c_char;


pub fn c_char_array_to_string(char_array: &[c_char]) -> String
{
    let raw_string = unsafe {
        let pointer = char_array.as_ptr();
        CStr::from_ptr(pointer)
    };

    raw_string.to_str()
        .expect("Failed to convert char array to string")
        .to_owned()
}

pub fn truncate_optional_usize_to_optional_u32(source: Option<usize>) -> Option<u32>
{
    match source
    {
        Some(x) => Some(x as u32),
        None => None,
    }
}

/*
pub fn c_string_array_to_vec(num_c_strings: &u32, c_strings: &* const * const c_char) -> Vec<*const i8>
{

}
 */