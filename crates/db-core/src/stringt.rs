use std::collections::LinkedList;

#[repr(C, align(16))]
struct StringT {
    length: u32,
    data  : StringTData,
}

#[repr(C)]
union StringTData {
    inlined: [u8; 12],   // strings ≤ 12 bytes: fully inline
    ptr    : StringTPtrData
}

impl StringTData {
    fn new(length: u32) -> Self {
        if length <= 12 {
            todo!();
        } else {
            todo!();
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct StringTPtrData {
    prefix    : [u8; 4], // first 4 bytes for fast comparison
    buffer_idx: u32,     // index into string buffer array
    offset    : u32      // offset within buffer
}

impl StringT {
    fn new(length: u32) -> Self {
        Self {
            length: length,
            data: StringTData::new(length)
        }
    }
}

struct StringBuffer {
    buffer: LinkedList<ArenaBuffer> // 1 MB
}

// Per TablePartition
impl StringBuffer {
    fn alloc(data: &[u8]) -> (u32, u32) { // (start, offset)
        todo!()
    }

    fn resolve<'a>(start: u32, offset: u32, length: u32) -> &'a[u8] {
        todo!()
    }
}

struct ArenaBuffer {
    data: [u8; 1_048_576],
    used: u32
}

impl ArenaBuffer {
    fn try_alloc(size: u32) -> u32 {
        todo!()
    }
}
