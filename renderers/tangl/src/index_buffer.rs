


pub struct IndexBuffer { 
    pub data : Vec<i32>,

    //Example
    // stride[0] = 3 // 3 floats for position
    // stride[1] = 2 // 2 floats for UVs
    // stride[2] = 3 // 3 floats for normals
    // stride[3] = 4 // 4 floats for tangents

    //That's how I could send data to the CPU
}

impl IndexBuffer {

    //One day, figure out how to template all of this :P
    //Todo :: Template THIS V V  
    pub fn from_pointer(data_ptr: *const i32, data_len: usize) -> Self {
        
        // Convert the pointer to a typed pointer
        let typed_ptr: *const i32 = data_ptr as *const i32;
        //Will be used later.

        let buffer = unsafe {
            // Create a slice from the typed pointer and length
            let slice: &[i32] = std::slice::from_raw_parts(typed_ptr, data_len / std::mem::size_of::<i32>());
        
            // Copy the slice to a buffer
            let mut buffer = Vec::with_capacity(slice.len());
            buffer.copy_from_slice(slice);
            buffer
        };

        Self { data: buffer }

    }
    
}
    

