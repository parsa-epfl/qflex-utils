// ─────────────────────────────────────────────────────────────────────────────

pub struct CircularBuffer<T> {
    capacity    : usize,
    write_ptr   : usize,
    storage     : Vec<T>,
    access_counter : Vec<u16>,
    nb_accessor: u16,
}

impl<T> CircularBuffer<T>
{

    pub fn new(capacity: usize, nb_accessor: u16) -> Self
    {
        Self {
            capacity,
            write_ptr: 0,
            storage: Vec::<T>::with_capacity(capacity),
            access_counter : vec![0; capacity],
            nb_accessor,
        }
    }
    
    pub fn read(&mut self, read_ptr: &mut usize) -> Result<&T, &'static str>
    {
        if self.write_ptr == *read_ptr { return Err("Nothing to read"); }
        if 0 >= self.access_counter[*read_ptr] { return Err("Already Read"); }

        // Decrement the number of access to this cell (unsafe)
        self.access_counter[*read_ptr] -= 1;

        // Clone the pointer, to use it in an owning function
        let present_ptr = read_ptr.to_owned();
        
        // Update read_ptr position
        *read_ptr += 1;
        *read_ptr %= self.capacity;

        // return value        
        self.storage.get(present_ptr).ok_or("Out of bound")
    }

 
    pub fn push(&mut self, value: T) -> Result<usize, &'static str>
    {

        let nb_access = match self.access_counter.get(self.write_ptr) {
            Some(v) => v,
            None => return Err("Out of bound§")
            
        };

        // If the number of time the cell was accessed is lower than the number of thread splawn (=guest)
        // that means the RingBuffer is full and should wait for the socket to keep up
        if *nb_access > 0 { return Err("Ring buffer is full"); }

        self.storage.insert(self.write_ptr, value);
        self.access_counter.insert(self.write_ptr, self.nb_accessor);
        self.write_ptr += 1;
        self.write_ptr %= self.capacity;

        Ok(self.write_ptr)
    }  

}