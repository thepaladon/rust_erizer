


pub struct VertexBuffer { 
    pub data : Vec<f32>,
    pub stride : Vec<i32>, //strides!  
    pub offset : i32 // offset until next vertex

    //Example
    // stride[0] = 3 // 3 floats for position
    // stride[1] = 2 // 2 floats for UVs
    // stride[2] = 3 // 3 floats for normals
    // stride[3] = 4 // 4 floats for tangents

    //That's how I could send data to the CPU
}

impl VertexBuffer {

    // Creates an empty array
    pub fn new() -> Self {
        Self { data: Vec::new(), stride: Vec::new(), offset: 0 }
    }

    //One day, figure out how to template all of this :P
    //Todo :: Template THIS V V  
    pub fn from_pointer(data_ptr: *const f32, data_len: usize, stride : &[i32]) -> Self {
        
        // Convert the pointer to a typed pointer
        let typed_ptr: *const f32 = data_ptr as *const f32;
        //Will be used later.

        let buffer = unsafe {
            // Create a slice from the typed pointer and length
            let slice: &[f32] = std::slice::from_raw_parts(typed_ptr, data_len / std::mem::size_of::<f32>());
        
            // Copy the slice to a buffer
            let mut buffer: Vec<f32> = Vec::with_capacity(slice.len());
            buffer.copy_from_slice(slice);
            buffer
        };

        let mut strides = Vec::new();
        let mut offset = 0;

        for value in stride {
            offset += *value;
            strides.push(*value);
        }

        Self { data: buffer, stride: strides, offset}

    }
    
}
    

