////////////////////////////////////////////////////////////////
// StaticFIFO
//
// This module implements a simple static fifo
// This is a fifo with fixed length at compile time.
//
// For us in static / embedded enviroments where std is not
// available and no dynamic memory.
//
// Peter Fetterer (kb3gtn@gmail.com) 
//
///////////////////////////////////////////////////////////////

use core::result::Result;
use core::result::Result::{Ok,Err};

// note length specified is raw storage container.
// fifo full pointers take up 1 element. so you will need N+1
pub struct StaticFifoU8<const N : usize> {
    buf: [ u8; N],
    read_ptr: usize,
    write_ptr: usize,
    capacity: usize,
}

pub enum StaticFifoError {
    Empty,
    Full,
}

impl<const N : usize> StaticFifoU8<N> {

    // create new StaticFifoU8
    pub fn new() -> Self {
        Self {
            buf : [0; N],
            read_ptr : 0,
            write_ptr : 0,
            capacity : N,
        }
    }

    #[inline]
    fn increment_readptr(&mut self) {
        self.read_ptr = (self.read_ptr + 1) % self.capacity;
    }

    #[inline]
    fn increment_writeptr(&mut self) {
        self.write_ptr = (self.write_ptr + 1) % self.capacity;
    }
    
    #[inline]
    pub fn is_empty(&self) -> bool {
        if self.read_ptr == self.write_ptr {
            return true;
        }
        return false;
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        let rp1 : usize = (self.write_ptr+1) % self.capacity;
        if rp1 == self.read_ptr {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn get(&mut self) -> Result<u8, StaticFifoError> {
        if self.is_empty() {
            return Err(StaticFifoError::Empty);
        }
        let rv :u8 = self.buf[self.read_ptr];
        self.increment_readptr();
        return Ok(rv)
    }

    #[inline]
    pub fn put(&mut self, data : u8) -> Result<(), StaticFifoError> {
        if self.is_full() {
            return Err(StaticFifoError::Full);
        }
        self.buf[self.write_ptr] = data; 
        self.increment_writeptr();
        return Ok(())
    }

    pub fn len(&self) -> usize {
        if self.read_ptr > self.write_ptr {
            (self.capacity - self.read_ptr) + self.write_ptr
        } else {
            self.write_ptr - self.read_ptr
        }
    }

    pub fn max_len(&self) -> usize {
        self.capacity
    }
}



// note length specified is raw storage container.
// fifo full pointers take up 1 element. so you will need N+1
pub struct StaticFifoU32<const N : usize> {
    buf: [ u32; N],
    read_ptr: usize,
    write_ptr: usize,
    capacity: usize,
}

impl<const N : usize> StaticFifoU32<N> {

    // create new StaticFifoU8
    pub fn new() -> Self {
        Self {
            buf : [0; N],
            read_ptr : 0,
            write_ptr : 0,
            capacity : N,
        }
    }

    #[inline]
    fn increment_readptr(&mut self) {
        self.read_ptr = (self.read_ptr + 1) % self.capacity;
    }

    #[inline]
    fn increment_writeptr(&mut self) {
        self.write_ptr = (self.write_ptr + 1) % self.capacity;
    }
    
    #[inline]
    pub fn is_empty(&self) -> bool {
        if self.read_ptr == self.write_ptr {
            return true;
        }
        return false;
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        let rp1 : usize = (self.write_ptr+1) % self.capacity;
        if rp1 == self.read_ptr {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn get(&mut self) -> Result<u32, StaticFifoError> {
        if self.is_empty() {
            return Err(StaticFifoError::Empty);
        }
        let rv :u32 = self.buf[self.read_ptr];
        self.increment_readptr();
        return Ok(rv)
    }

    #[inline]
    pub fn put(&mut self, data : u32) -> Result<(), StaticFifoError> {
        if self.is_full() {
            return Err(StaticFifoError::Full);
        }
        self.buf[self.write_ptr] = data; 
        self.increment_writeptr();
        return Ok(())
    }

    pub fn len(&self) -> usize {
        if self.read_ptr > self.write_ptr {
            (self.capacity - self.read_ptr) + self.write_ptr
        } else {
            self.write_ptr - self.read_ptr
        }
    }

    pub fn max_len(&self) -> usize {
        self.capacity
    }
}



//////////////////////////////////////
// Run Test Command:
// cargo test -- --nocapture
/////////////////////////////////////

#[cfg(test)]
mod tests {

    use std::println;
    use super::*; 

    #[test]
    fn fifo_functional_testi_u8() -> Result<(), &'static str> {

        println!("##################### FIFO FUNCTIONAL TEST U8 ######################################");
        
        // create static fifo of 16 bytes
        let mut byte_fifo : StaticFifoU8<16> = StaticFifoU8::<16>::new();


        assert!( byte_fifo.is_empty(), "FIFO not empty at startup..");
        assert!( !byte_fifo.is_full(), "FIFO reported as full at initialization.."); 
        println!("Buffer Len reported: {}", byte_fifo.len() );
        assert!( byte_fifo.len() == 0, "Buffer Length not zero when started" );
        println!("byte_fifo capacity reported: {}", byte_fifo.max_len());
        assert!( byte_fifo.max_len() == 16, "Buffer capacity reported was not 16.");

        // Try to get an item from an empty fifo
        match byte_fifo.get() {
            Ok(v) => { println!("empty fifo return value: {}", v); assert!(false, "get return value on empty fifo.."); () },
            Err(_s) => { println!("Got Err() when performing get() on empty buffer.. (OK)"); () }
        };

        // Try pushing 1 item to the fifo..
        let mut value : u8 = 65;
        match byte_fifo.put( value ) {
            Ok(_) => { println!("Put 1 item (65) to fifo ok.."); () },
            Err(_) => { println!("Put returned error when fifo was empty.. (BAD)"); assert!(false, "put failed on empty fifo.."); },
        }

        println!("byte_fifo len after add 1 element: {}", byte_fifo.len());
        assert!( byte_fifo.len() == 1, "Buffer Length should be 1 after adding 1 element." );

        assert!( ! byte_fifo.is_empty(), "FIFO reports empty after putting 1 value in fifo.. (BAD)");
        assert!( ! byte_fifo.is_full(), "FIFO reports full after putting 1 value in fifo.. (BAD)");

         // Try to get the 1 item in the fifo.
        match byte_fifo.get() {
            Ok(v) => { println!("got value {} from fifo.", v); assert!(v==65,"get did not return the expected value 65.."); },
            Err(_) => { println!("Get returned error on not empty fifo.. (BAD)"); assert!(false, "get failed on non-empty fifo.") },
        }

        // fifo should be empty again..
        assert!( byte_fifo.is_empty(), "FIFO not empty at startup..");
        assert!( !byte_fifo.is_full(), "FIFO reported as full at initialization.."); 

        // now fill fifo
        println!("Filling Fifo full..\n");
        for i in 0..15 {
            value = i as u8;
            match byte_fifo.put(value) {
                Ok(_) => { println!("Put item {} to fifo ok..", value); () },
                Err(_) => { println!("Put returned error on index {i} (BAD)"); assert!(false, "put failed when filling fifo."); },
            } 
        }
 
        // fifo should be full..
        assert!( !byte_fifo.is_empty(), "FIFO reports empty when full?");
        assert!( byte_fifo.is_full(), "FIFO did not report full when it should be.");

        // read one item from fifo, shoud go not full..
        // Try to get the 1 item in the fifo.
        match byte_fifo.get() {
            Ok(v) => { println!("got value {} from fifo.", v); assert!(v==0,"get did not return the expected value 0 form the fill operation."); },
            Err(_) => { println!("Get returned error on not empty fifo.. (BAD)"); assert!(false, "get failed on non-empty fifo.") },
        }

        // FIFO should not be full or empty.
        assert!( !byte_fifo.is_empty(), "FIFO not empty at startup..");
        assert!( !byte_fifo.is_full(), "FIFO reported as full at initialization.."); 

        // Add new item, should roll over read/write pointers.
        println!("Adding item to rollover read/write pointers..");

        value = 100;
        match byte_fifo.put( value ) {
            Ok(_) => { println!("Put 1 item (100) to fifo ok.."); () },
            Err(_) => { println!("Put returned error when fifo was not empty.. (BAD)"); assert!(false, "put failed on not full fifo.."); },
        }

        println!("byte_fifo len after add 1 element: {}", byte_fifo.len());
        assert!( byte_fifo.len() == 15, "Buffer Length should be 15 after adding 1 element (should be full)." );

        // fifo should be full..
        assert!( !byte_fifo.is_empty(), "FIFO reports empty when full?");
        assert!( byte_fifo.is_full(), "FIFO did not report full when it should be.");

        // remove 8 elements
        println!("removing some elements from the fifo..");
        for _i in 0..8 {
            match byte_fifo.get() {
                Ok(v) => { println!("got value {} from fifo.", v);  },
                Err(_) => { println!("Get returned error on not empty fifo.. (BAD)"); assert!(false, "get failed on non-empty fifo.") },
            }
        }

        println!("Adding 6 more elements back into fifo");
        for i in 0..6 {
            value = i+20 as u8;
            match byte_fifo.put(value) {
                Ok(_) => { println!("Put item {} to fifo ok..", value); () },
                Err(_) => { println!("Put returned error on index {i} (BAD)"); assert!(false, "put failed when filling fifo."); },
            } 
        }
 
        println!("byte_fifo len after adding element: {}", byte_fifo.len());
        assert!( byte_fifo.len() == 13, "Buffer Length should be 13 after adding elements." );

        // remove 13 elements from fifo to make it empty.
        println!("removing 13 elements from fifo..");
        for _i in 0..13 {
            match byte_fifo.get() {
                Ok(v) => { println!("got value {} from fifo.", v);  },
                Err(_) => { println!("Get returned error on not empty fifo.. (BAD)"); assert!(false, "get failed on non-empty fifo.") },
            }
        }

        // fifo should be empty again..
        assert!( byte_fifo.is_empty(), "FIFO not empty at after emptying..");
        assert!( !byte_fifo.is_full(), "FIFO reported as full when it should be empty.."); 



        Ok(())
    }


    #[test]
    fn fifo_functional_testi_u32() -> Result<(), &'static str> {
        
        println!("##################### FIFO FUNCTIONAL TEST U32 ######################################");

        // create static fifo of 16 bytes
        let mut byte_fifo : StaticFifoU32<16> = StaticFifoU32::<16>::new();


        assert!( byte_fifo.is_empty(), "FIFO not empty at startup..");
        assert!( !byte_fifo.is_full(), "FIFO reported as full at initialization.."); 
        println!("Buffer Len reported: {}", byte_fifo.len() );
        assert!( byte_fifo.len() == 0, "Buffer Length not zero when started" );
        println!("byte_fifo capacity reported: {}", byte_fifo.max_len());
        assert!( byte_fifo.max_len() == 16, "Buffer capacity reported was not 16.");

        // Try to get an item from an empty fifo
        match byte_fifo.get() {
            Ok(v) => { println!("empty fifo return value: {}", v); assert!(false, "get return value on empty fifo.."); () },
            Err(_s) => { println!("Got Err() when performing get() on empty buffer.. (OK)"); () }
        };

        // Try pushing 1 item to the fifo..
        let mut value : u32 = 0x1acffc1d;
        match byte_fifo.put( value ) {
            Ok(_) => { println!("Put 1 item to fifo ok.."); () },
            Err(_) => { println!("Put returned error when fifo was empty.. (BAD)"); assert!(false, "put failed on empty fifo.."); },
        }

        println!("byte_fifo len after add 1 element: {}", byte_fifo.len());
        assert!( byte_fifo.len() == 1, "Buffer Length should be 1 after adding 1 element." );

        assert!( ! byte_fifo.is_empty(), "FIFO reports empty after putting 1 value in fifo.. (BAD)");
        assert!( ! byte_fifo.is_full(), "FIFO reports full after putting 1 value in fifo.. (BAD)");

         // Try to get the 1 item in the fifo.
        match byte_fifo.get() {
            Ok(v) => { println!("got value {} from fifo.", v); },
            Err(_) => { println!("Get returned error on not empty fifo.. (BAD)"); assert!(false, "get failed on non-empty fifo.") },
        }

        // fifo should be empty again..
        assert!( byte_fifo.is_empty(), "FIFO not empty at startup..");
        assert!( !byte_fifo.is_full(), "FIFO reported as full at initialization.."); 

        // now fill fifo
        println!("Filling Fifo full..\n");
        for i in 0..15 {
            value = i*1000;
            match byte_fifo.put(value) {
                Ok(_) => { println!("Put item {} to fifo ok..", value); () },
                Err(_) => { println!("Put returned error on index {i} (BAD)"); assert!(false, "put failed when filling fifo."); },
            } 
        }
 
        // fifo should be full..
        assert!( !byte_fifo.is_empty(), "FIFO reports empty when full?");
        assert!( byte_fifo.is_full(), "FIFO did not report full when it should be.");

        // read one item from fifo, shoud go not full..
        // Try to get the 1 item in the fifo.
        match byte_fifo.get() {
            Ok(v) => { println!("got value {} from fifo.", v); },
            Err(_) => { println!("Get returned error on not empty fifo.. (BAD)"); assert!(false, "get failed on non-empty fifo.") },
        }

        // FIFO should not be full or empty.
        assert!( !byte_fifo.is_empty(), "FIFO not empty at startup..");
        assert!( !byte_fifo.is_full(), "FIFO reported as full at initialization.."); 

        // Add new item, should roll over read/write pointers.
        println!("Adding item to rollover read/write pointers..");

        value = 154939000;
        match byte_fifo.put( value ) {
            Ok(_) => { println!("Put 1 item to fifo ok.."); () },
            Err(_) => { println!("Put returned error when fifo was not empty.. (BAD)"); assert!(false, "put failed on not full fifo.."); },
        }

        println!("byte_fifo len after add 1 element: {}", byte_fifo.len());
        assert!( byte_fifo.len() == 15, "Buffer Length should be 15 after adding 1 element (should be full)." );

        // fifo should be full..
        assert!( !byte_fifo.is_empty(), "FIFO reports empty when full?");
        assert!( byte_fifo.is_full(), "FIFO did not report full when it should be.");

        // remove 8 elements
        println!("removing some elements from the fifo..");
        for _i in 0..8 {
            match byte_fifo.get() {
                Ok(v) => { println!("got value {} from fifo.", v);  },
                Err(_) => { println!("Get returned error on not empty fifo.. (BAD)"); assert!(false, "get failed on non-empty fifo.") },
            }
        }

        println!("Adding 6 more elements back into fifo");
        for i in 0..6 {
            value = i+20 as u32;
            match byte_fifo.put(value) {
                Ok(_) => { println!("Put item {} to fifo ok..", value); () },
                Err(_) => { println!("Put returned error on index {i} (BAD)"); assert!(false, "put failed when filling fifo."); },
            } 
        }
 
        println!("byte_fifo len after adding element: {}", byte_fifo.len());
        assert!( byte_fifo.len() == 13, "Buffer Length should be 13 after adding elements." );

        // remove 13 elements from fifo to make it empty.
        println!("removing 13 elements from fifo..");
        for _i in 0..13 {
            match byte_fifo.get() {
                Ok(v) => { println!("got value {} from fifo.", v);  },
                Err(_) => { println!("Get returned error on not empty fifo.. (BAD)"); assert!(false, "get failed on non-empty fifo.") },
            }
        }

        // fifo should be empty again..
        assert!( byte_fifo.is_empty(), "FIFO not empty at after emptying..");
        assert!( !byte_fifo.is_full(), "FIFO reported as full when it should be empty.."); 



        Ok(())
    }
}

