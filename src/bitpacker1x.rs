use super::BitPacker;

const BLOCK_LEN: usize = 32;

mod scalar {

    use super::BLOCK_LEN;

    type DataType = u32;

    fn set1(el: i32) -> DataType {
        el as u32
    }

    fn right_shift_32(el: DataType, shift: i32) -> DataType {
        el >> shift
    }

    fn left_shift_32(el: DataType, shift: i32) -> DataType {
        el << shift
    }

    fn op_or(left: DataType, right: DataType) -> DataType {
        left | right
    }

    fn op_and(left: DataType, right: DataType) -> DataType {
        left & right
    }

    unsafe fn load_unaligned(addr: *const DataType) -> DataType {
        *addr
    }

    unsafe fn store_unaligned(addr: *mut DataType, data: DataType) {
        *addr = data;
    }

    fn or_collapse_to_u32(accumulator: DataType) -> u32 {
        accumulator
    }

    fn compute_delta(curr: DataType, prev: DataType) -> DataType { curr.wrapping_sub(prev) }

    fn integrate_delta(offset: DataType, delta: DataType) -> DataType {
        offset.wrapping_add(delta)
    }


    // The `cfg(any(debug, not(debug)))` is here to put an attribute that has no effect.
    //
    // For other bitpacker, we enable specific CPU instruction set, but for the
    // scalar bitpacker none is required.
    declare_bitpacker!( cfg(any(debug, not(debug))) );
}


pub struct BitPacker1x;

impl BitPacker for BitPacker1x {

    const BLOCK_LEN: usize = BLOCK_LEN;

    fn compress(decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize {
        unsafe {
            scalar::compress(decompressed, compressed, num_bits)
        }

    }

    fn compress_sorted(initial: u32, decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize {
        unsafe {
            scalar::compress_sorted(initial, decompressed, compressed, num_bits)
        }

    }

    fn decompress(compressed: &[u8], decompressed: &mut [u32], num_bits: u8) -> usize {
        unsafe {
            scalar::decompress(compressed, decompressed, num_bits)
        }
    }

    fn decompress_sorted(initial: u32, compressed: &[u8], decompressed: &mut [u32], num_bits: u8) -> usize {
        unsafe {
            scalar::decompress_sorted(initial, compressed, decompressed, num_bits)
        }
    }

    fn num_bits(decompressed: &[u32]) -> u8 {
        unsafe {
            scalar::num_bits(decompressed)
        }

    }

    fn num_bits_sorted(initial: u32, decompressed: &[u32]) -> u8 {
        unsafe {
            scalar::num_bits_sorted(initial, decompressed)
        }
    }
}



#[cfg(test)]
mod tests {
    use super::BitPacker1x;
    bitpacker_tests!(BitPacker1x);
}