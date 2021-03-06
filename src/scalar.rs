const NUM_INTS_PER_REGISTER: usize = 1;

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

fn compute_delta(curr: u32, prev: u32) -> u32 { curr.wrapping_sub(prev) }

fn integrate_delta(offset: DataType, delta: DataType) -> DataType {
    offset.wrapping_add(delta)
}

declare_bitpacker!(ScalarBitPacker, 32);
