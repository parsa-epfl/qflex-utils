


// trait CircularBufferInterface<T> 
// {
//     fn new(capacity: usize) -> Self;
//     fn read(&mut self, read_ptr: &mut usize) -> Option<&T>;
// }

// trait CircularBufferWriterInterface<C>: CircularBufferInterface<T>
// {
//     fn new(buffer: C) -> Self;
//     fn write(&mut self, write_ptr: &mut usize, value: T);
// }

// ─────────────────────────────────────────────────────────────────────────────

// struct CircularBufferWriter<C>
// {
//     buffer: C,
// }

// impl<C, T> CircularBufferWriterInterface<T> for CircularBufferWriter<C>
// {

//     fn new(buffer: C) -> Self {
//         CircularBufferWriter { buffer }
//     }

//     fn write(&mut self, write_ptr: &mut usize, value: T)
//     {
//         self.storage[write_ptr % self.capacity] = value;
//         *write_ptr += 1;
//         *write_ptr %= self.capacity;
//     }   

// }

// ─────────────────────────────────────────────────────────────────────────────

pub struct CircularBuffer<T> {
    capacity    : usize,
    write_ptr   : usize,
    storage     : Vec<T>,
}

impl<T> /*CircularBufferInterface<T> for*/ CircularBuffer<T>
{

    pub fn new(capacity: usize) -> Self
    {
        Self {
            capacity,
            write_ptr: 0,
            storage: Vec::<T>::with_capacity(capacity),
        }
    }

    pub fn read(&mut self, read_ptr: &mut usize) -> Option<&T>
    {
        if *read_ptr == self.write_ptr { return None; }

        let old_ptr = read_ptr.clone();
        *read_ptr += 1;
        *read_ptr %= self.capacity;
        println!("old={}, actual={}", old_ptr, *read_ptr);
        self.storage.get(old_ptr % self.capacity)
    }

    pub fn push(&mut self, value: T)
    {
        self.storage.insert(self.write_ptr, value);
        self.write_ptr += 1;
        self.write_ptr %= self.capacity;
    }  

}


// fn main()
// {
//     let mut circ_buffer: CircularBuffer<u8> = CircularBuffer::new(16);
//     let mut my_pos : usize = 0;
//     circ_buffer.push(2);
//     circ_buffer.push(6);
//     circ_buffer.push(200);

//     println!("{}", circ_buffer.read(&mut my_pos).unwrap());


//     println!("{}", circ_buffer.read(&mut my_pos).unwrap());

// }