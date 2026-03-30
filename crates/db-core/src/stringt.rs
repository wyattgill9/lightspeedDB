use std::collections::LinkedList;

#[repr(C, align(16))]
struct StringRef {
    length: u32,

    // union {
    //     inline: [u8; 12];
    //     struct {
    //         prefix: [u8; 4]
    //         arena_idx: u32
    //         offset: u32
    //     }
    // }
    payload: [u8; 12],
}

impl StringRef {
    fn is_inline(&self) -> bool {
        self.length <= 12
    }
}

struct StringBuffer {
    buffer: LinkedList<ArenaBuffer>, // 1 MB
}

// Per TablePartition
impl StringBuffer {
    fn alloc(data: &[u8]) -> (u32, u32) {
        // (start, offset)
        todo!()
    }

    fn resolve<'a>(start: u32, offset: u32, length: u32) -> &'a [u8] {
        todo!()
    }
}

struct ArenaBuffer {
    data: [u8; 1_048_576],
    used: u32,
}

impl ArenaBuffer {
    fn try_alloc(size: u32) -> u32 {
        todo!()
    }
}
