pub struct simple_struct {
    pub a: u32
}


fn main() {
    println!("pok");

    unsafe {
        let lib = libloading::Library::new(
            "C:/Users/xavie/workspace/testdylib/plug0/target/debug/plug0.dll",
        )
        .unwrap();

        let func: libloading::Symbol<fn() -> u32> = lib.get(b"get_number").unwrap();
        println!("get_number got {} == expect 5", func());

        let func2: libloading::Symbol<fn() -> *mut u32> = lib.get(b"get_number_pointer").unwrap();
        println!("get_number_pointer got {} == expect 5", *func2());

        let func3: libloading::Symbol<fn() -> *mut simple_struct> = lib.get(b"get_simple_struct_ptr").unwrap();
        println!("get_simple_struct_ptr got {} == expect 6", (*func3()).a);


        
    }
}
