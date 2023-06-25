// trait for type T E to support T * E
pub trait MulTE<T, E> {
    fn mul(a :&T, b :&E) -> E;
}

// a implementation for primitary type
// defaultly, T is smaller than E
pub(crate) struct MulTEDefaultType;

impl MulTE<u8, u8> for MulTEDefaultType {
    fn mul(a :&u8, b :&u8) -> u8 {
        a * b
    }
}

impl MulTE<u8, u16> for MulTEDefaultType {
    fn mul(a :&u8, b :&u16) -> u16 {
        (*a) as u16 * b
    }
}

impl MulTE<u8, u32> for MulTEDefaultType {
    fn mul(a :&u8, b :&u32) -> u32 {
        (*a) as u32 * b
    }
}

impl MulTE<u8, u64> for MulTEDefaultType {
    fn mul(a :&u8, b :&u64) -> u64 {
        (*a) as u64 * b
    }
}

impl MulTE<u8, u128> for MulTEDefaultType {
    fn mul(a :&u8, b :&u128) -> u128 {
        (*a) as u128 * b
    }
}

impl MulTE<u8, i8> for MulTEDefaultType {
    fn mul(a :&u8, b :&i8) -> i8 {
        (*a) as i8 * b
    }
}

impl MulTE<u8, i16> for MulTEDefaultType {
    fn mul(a :&u8, b :&i16) -> i16 {
        (*a) as i16 * b
    }
}

impl MulTE<u8, i32> for MulTEDefaultType {
    fn mul(a :&u8, b :&i32) -> i32 {
        (*a) as i32 * b
    }
}

impl MulTE<u8, i64> for MulTEDefaultType {
    fn mul(a :&u8, b :&i64) -> i64 {
        (*a) as i64 * b
    }
}

impl MulTE<u8, i128> for MulTEDefaultType {
    fn mul(a :&u8, b :&i128) -> i128 {
        (*a) as i128 * b
    }
}

impl MulTE<u8, f32> for MulTEDefaultType {
    fn mul(a :&u8, b :&f32) -> f32 {
        (*a) as f32 * b
    }
}

impl MulTE<u8, f64> for MulTEDefaultType {
    fn mul(a :&u8, b :&f64) -> f64 {
        (*a) as f64 * b
    }
}

impl MulTE<i8, u8> for MulTEDefaultType {
    fn mul(a :&i8, b :&u8) -> u8 {
        (*a) as u8 * b
    }
}

impl MulTE<i8, u16> for MulTEDefaultType {
    fn mul(a :&i8, b :&u16) -> u16 {
        (*a) as u16 * b
    }
}

impl MulTE<i8, u32> for MulTEDefaultType {
    fn mul(a :&i8, b :&u32) -> u32 {
        (*a) as u32 * b
    }
}

impl MulTE<i8, u64> for MulTEDefaultType {
    fn mul(a :&i8, b :&u64) -> u64 {
        (*a) as u64 * b
    }
}

impl MulTE<i8, u128> for MulTEDefaultType {
    fn mul(a :&i8, b :&u128) -> u128 {
        (*a) as u128 * b
    }
}

impl MulTE<i8, i8> for MulTEDefaultType {
    fn mul(a :&i8, b :&i8) -> i8 {
        a * b
    }
}

impl MulTE<i8, i16> for MulTEDefaultType {
    fn mul(a :&i8, b :&i16) -> i16 {
        (*a) as i16 * b
    }
}

impl MulTE<i8, i32> for MulTEDefaultType {
    fn mul(a :&i8, b :&i32) -> i32 {
        (*a) as i32 * b
    }
}

impl MulTE<i8, i64> for MulTEDefaultType {
    fn mul(a :&i8, b :&i64) -> i64 {
        (*a) as i64 * b
    }
}

impl MulTE<i8, i128> for MulTEDefaultType {
    fn mul(a :&i8, b :&i128) -> i128 {
        (*a) as i128 * b
    }
}

impl MulTE<i8, f32> for MulTEDefaultType {
    fn mul(a :&i8, b :&f32) -> f32 {
        (*a) as f32 * b
    }
}

impl MulTE<i8, f64> for MulTEDefaultType {
    fn mul(a :&i8, b :&f64) -> f64 {
        (*a) as f64 * b
    }
}

impl MulTE<u16, u16> for MulTEDefaultType {
    fn mul(a :&u16, b :&u16) -> u16 {
        a * b
    }
}

impl MulTE<u16, u32> for MulTEDefaultType {
    fn mul(a :&u16, b :&u32) -> u32 {
        (*a) as u32 * b
    }
}

impl MulTE<u16, u64> for MulTEDefaultType {
    fn mul(a :&u16, b :&u64) -> u64 {
        (*a) as u64 * b
    }
}

impl MulTE<u16, u128> for MulTEDefaultType {
    fn mul(a :&u16, b :&u128) -> u128 {
        (*a) as u128 * b
    }
}

impl MulTE<u16, i16> for MulTEDefaultType {
    fn mul(a :&u16, b :&i16) -> i16 {
        (*a) as i16 * b
    }
}

impl MulTE<u16, i32> for MulTEDefaultType {
    fn mul(a :&u16, b :&i32) -> i32 {
        (*a) as i32 * b
    }
}

impl MulTE<u16, i64> for MulTEDefaultType {
    fn mul(a :&u16, b :&i64) -> i64 {
        (*a) as i64 * b
    }
}

impl MulTE<u16, i128> for MulTEDefaultType {
    fn mul(a :&u16, b :&i128) -> i128 {
        (*a) as i128 * b
    }
}

impl MulTE<u16, f32> for MulTEDefaultType {
    fn mul(a :&u16, b :&f32) -> f32 {
        (*a) as f32 * b
    }
}

impl MulTE<u16, f64> for MulTEDefaultType {
    fn mul(a :&u16, b :&f64) -> f64 {
        (*a) as f64 * b
    }
}

impl MulTE<i16, u16> for MulTEDefaultType {
    fn mul(a :&i16, b :&u16) -> u16 {
        (*a) as u16 * b
    }
}

impl MulTE<i16, u32> for MulTEDefaultType {
    fn mul(a :&i16, b :&u32) -> u32 {
        (*a) as u32 * b
    }
}

impl MulTE<i16, u64> for MulTEDefaultType {
    fn mul(a :&i16, b :&u64) -> u64 {
        (*a) as u64 * b
    }
}

impl MulTE<i16, u128> for MulTEDefaultType {
    fn mul(a :&i16, b :&u128) -> u128 {
        (*a) as u128 * b
    }
}

impl MulTE<i16, i16> for MulTEDefaultType {
    fn mul(a :&i16, b :&i16) -> i16 {
        a * b
    }
}

impl MulTE<i16, i32> for MulTEDefaultType {
    fn mul(a :&i16, b :&i32) -> i32 {
        (*a) as i32 * b
    }
}

impl MulTE<i16, i64> for MulTEDefaultType {
    fn mul(a :&i16, b :&i64) -> i64 {
        (*a) as i64 * b
    }
}

impl MulTE<i16, i128> for MulTEDefaultType {
    fn mul(a :&i16, b :&i128) -> i128 {
        (*a) as i128 * b
    }
}

impl MulTE<i16, f32> for MulTEDefaultType {
    fn mul(a :&i16, b :&f32) -> f32 {
        (*a) as f32 * b
    }
}

impl MulTE<i16, f64> for MulTEDefaultType {
    fn mul(a :&i16, b :&f64) -> f64 {
        (*a) as f64 * b
    }
}

impl MulTE<u32, u32> for MulTEDefaultType {
    fn mul(a :&u32, b :&u32) -> u32 {
        a * b
    }
}

impl MulTE<u32, u64> for MulTEDefaultType {
    fn mul(a :&u32, b :&u64) -> u64 {
        (*a) as u64 * b
    }
}

impl MulTE<u32, u128> for MulTEDefaultType {
    fn mul(a :&u32, b :&u128) -> u128 {
        (*a) as u128 * b
    }
}

impl MulTE<u32, i32> for MulTEDefaultType {
    fn mul(a :&u32, b :&i32) -> i32 {
        (*a) as i32 * b
    }
}

impl MulTE<u32, i64> for MulTEDefaultType {
    fn mul(a :&u32, b :&i64) -> i64 {
        (*a) as i64 * b
    }
}

impl MulTE<u32, i128> for MulTEDefaultType {
    fn mul(a :&u32, b :&i128) -> i128 {
        (*a) as i128 * b
    }
}

impl MulTE<u32, f32> for MulTEDefaultType {
    fn mul(a :&u32, b :&f32) -> f32 {
        (*a) as f32 * b
    }
}

impl MulTE<u32, f64> for MulTEDefaultType {
    fn mul(a :&u32, b :&f64) -> f64 {
        (*a) as f64 * b
    }
}

impl MulTE<i32, u32> for MulTEDefaultType {
    fn mul(a :&i32, b :&u32) -> u32 {
        (*a) as u32 * b
    }
}

impl MulTE<i32, u64> for MulTEDefaultType {
    fn mul(a :&i32, b :&u64) -> u64 {
        (*a) as u64 * b
    }
}

impl MulTE<i32, u128> for MulTEDefaultType {
    fn mul(a :&i32, b :&u128) -> u128 {
        (*a) as u128 * b
    }
}

impl MulTE<i32, i32> for MulTEDefaultType {
    fn mul(a :&i32, b :&i32) -> i32 {
        a * b
    }
}

impl MulTE<i32, i64> for MulTEDefaultType {
    fn mul(a :&i32, b :&i64) -> i64 {
        (*a) as i64 * b
    }
}

impl MulTE<i32, i128> for MulTEDefaultType {
    fn mul(a :&i32, b :&i128) -> i128 {
        (*a) as i128 * b
    }
}

impl MulTE<i32, f32> for MulTEDefaultType {
    fn mul(a :&i32, b :&f32) -> f32 {
        (*a) as f32 * b
    }
}

impl MulTE<i32, f64> for MulTEDefaultType {
    fn mul(a :&i32, b :&f64) -> f64 {
        (*a) as f64 * b
    }
}

impl MulTE<u64, u64> for MulTEDefaultType {
    fn mul(a :&u64, b :&u64) -> u64 {
        a * b
    }
}

impl MulTE<u64, u128> for MulTEDefaultType {
    fn mul(a :&u64, b :&u128) -> u128 {
        (*a) as u128 * b
    }
}

impl MulTE<u64, i64> for MulTEDefaultType {
    fn mul(a :&u64, b :&i64) -> i64 {
        (*a) as i64 * b
    }
}

impl MulTE<u64, i128> for MulTEDefaultType {
    fn mul(a :&u64, b :&i128) -> i128 {
        (*a) as i128 * b
    }
}

impl MulTE<u64, f32> for MulTEDefaultType {
    fn mul(a :&u64, b :&f32) -> f32 {
        (*a) as f32 * b
    }
}

impl MulTE<u64, f64> for MulTEDefaultType {
    fn mul(a :&u64, b :&f64) -> f64 {
        (*a) as f64 * b
    }
}

impl MulTE<i64, u64> for MulTEDefaultType {
    fn mul(a :&i64, b :&u64) -> u64 {
        (*a) as u64 * b
    }
}

impl MulTE<i64, u128> for MulTEDefaultType {
    fn mul(a :&i64, b :&u128) -> u128 {
        (*a) as u128 * b
    }
}

impl MulTE<i64, i64> for MulTEDefaultType {
    fn mul(a :&i64, b :&i64) -> i64 {
        a * b
    }
}

impl MulTE<i64, i128> for MulTEDefaultType {
    fn mul(a :&i64, b :&i128) -> i128 {
        (*a) as i128 * b
    }
}

impl MulTE<i64, f32> for MulTEDefaultType {
    fn mul(a :&i64, b :&f32) -> f32 {
        (*a) as f32 * b
    }
}

impl MulTE<i64, f64> for MulTEDefaultType {
    fn mul(a :&i64, b :&f64) -> f64 {
        (*a) as f64 * b
    }
}

impl MulTE<u128, u128> for MulTEDefaultType {
    fn mul(a :&u128, b :&u128) -> u128 {
        a * b
    }
}

impl MulTE<u128, i128> for MulTEDefaultType {
    fn mul(a :&u128, b :&i128) -> i128 {
        (*a) as i128 * b
    }
}

impl MulTE<u128, f32> for MulTEDefaultType {
    fn mul(a :&u128, b :&f32) -> f32 {
        (*a) as f32 * b
    }
}

impl MulTE<u128, f64> for MulTEDefaultType {
    fn mul(a :&u128, b :&f64) -> f64 {
        (*a) as f64 * b
    }
}

impl MulTE<i128, u128> for MulTEDefaultType {
    fn mul(a :&i128, b :&u128) -> u128 {
        (*a) as u128 * b
    }
}

impl MulTE<i128, i128> for MulTEDefaultType {
    fn mul(a :&i128, b :&i128) -> i128 {
        a * b
    }
}

impl MulTE<i128, f32> for MulTEDefaultType {
    fn mul(a :&i128, b :&f32) -> f32 {
        (*a) as f32 * b
    }
}

impl MulTE<i128, f64> for MulTEDefaultType {
    fn mul(a :&i128, b :&f64) -> f64 {
        (*a) as f64 * b
    }
}

impl MulTE<f32, f32> for MulTEDefaultType {
    fn mul(a :&f32, b :&f32) -> f32 {
        a * b
    }
}

impl MulTE<f32, f64> for MulTEDefaultType {
    fn mul(a :&f32, b :&f64) -> f64 {
        (*a) as f64 * b
    }
}

impl MulTE<f64, f64> for MulTEDefaultType {
    fn mul(a :&f64, b :&f64) -> f64 {
        a * b
    }
}