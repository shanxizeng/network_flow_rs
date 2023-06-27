//! 实现对图进行输入和输出的module
//! 

pub trait BitIO {
    fn to_bit(&self) -> Vec<u8>;
    fn from_bit(a : &[u8]) -> Self;
}

use core::mem::size_of;

macro_rules! BitIOPrim {
    ($ty : ty) => {
        impl BitIO for $ty {
            fn to_bit(&self) -> Vec<u8> {
                let mut res = vec![];
                let l = self.to_be_bytes();
                for i in l {
                    res.push(i);
                }
                res
            }
            fn from_bit(a : &[u8]) -> Self {
                let mut arr = [0; size_of::<$ty>()];
                for i in 0..size_of::<$ty>() {
                    arr[i] = a[i];
                }
                <$ty>::from_be_bytes(arr)
            }
        }
    };
}

BitIOPrim!(u8);
BitIOPrim!(u16);
BitIOPrim!(u32);
BitIOPrim!(u64);
BitIOPrim!(u128);
BitIOPrim!(usize);
BitIOPrim!(i8);
BitIOPrim!(i16);
BitIOPrim!(i32);
BitIOPrim!(i64);
BitIOPrim!(i128);
BitIOPrim!(isize);
BitIOPrim!(f32);
BitIOPrim!(f64);