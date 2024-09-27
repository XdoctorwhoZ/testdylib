#[no_mangle]
pub fn get_number() -> u32 {
    5
}

#[no_mangle]
pub fn get_number_pointer() -> *mut u32 {
    Box::into_raw(Box::new(5))
}


// can be plugin props
pub struct simple_struct {
    a: u32
}


#[no_mangle]
pub fn get_simple_struct_ptr() -> *mut simple_struct {
    Box::into_raw(Box::new(simple_struct{a:6}))
}


// trait wahou {
//     fn test() -> u32;
// }

// struct wahouimpl {}

// impl wahou for wahouimpl{
//     fn test() -> u32 {
//         10
//     }
// }

// #[no_mangle]
// pub fn get_wahou_ptr() -> *mut wahou {
//     Box::into_raw(Box::new(wahouimpl{}))
// }

