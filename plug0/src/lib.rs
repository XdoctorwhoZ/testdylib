#[no_mangle]
pub fn get_number() -> u32 {
    5
}

#[no_mangle]
pub fn get_number_pointer() -> *mut u32 {
    Box::into_raw(Box::new(5))
}


pub struct simple_struct {
    a: u32
}


#[no_mangle]
pub fn get_simple_struct_ptr() -> *mut simple_struct {
    Box::into_raw(Box::new(simple_struct{a:6}))
}