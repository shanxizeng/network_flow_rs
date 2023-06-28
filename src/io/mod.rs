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
impl<T : BitIO> BitIO for Vec<T> {
    fn to_bit(&self) -> Vec<u8> {
        let mut res = vec![];
        let l = self.len().to_be_bytes();
        for i in l {
            res.push(i);
        }
        for i in self {
            let mut temp = i.to_bit();
            let l = temp.len().to_be_bytes();
            for i in l {
                res.push(i);
            }
            res.append(&mut temp);
        }
        res
    }
    fn from_bit(a : &[u8]) -> Self {
        let mut res = vec![];
        let mut buf = [0; size_of::<usize>()];
        let mut temp = 0;
        for i in 0..size_of::<usize>() {
            buf[i] = a[temp];
            temp = temp + 1;
        }
        let len = usize::from_be_bytes(buf);
        for _ in 0..len {
            for i in 0..size_of::<usize>() {
                buf[i] = a[temp];
                temp = temp + 1;
            }
            let len = usize::from_be_bytes(buf);
            let mut buf2 = vec![];
            for _ in 0..len {
                buf2.push(a[temp]);
                temp = temp + 1;
            }
            res.push(T::from_bit(&buf2));
        } 
        res
    }
}
impl BitIO for String {
    fn to_bit(&self) -> Vec<u8> {
        let mut res = vec![];
        let l = self.as_bytes().len().to_be_bytes();
        for i in l {
            res.push(i);
        }
        for i in self.as_bytes() {
            res.push(*i);
        }
        res
    }
    fn from_bit(a : &[u8]) -> Self {
        let mut res = vec![];
        let mut buf = [0; size_of::<usize>()];
        let mut temp = 0;
        for i in 0..size_of::<usize>() {
            buf[i] = a[temp];
            temp = temp + 1;
        }
        let len = usize::from_be_bytes(buf);
        for _ in 0..len {
            res.push(a[temp]);
            temp = temp + 1;
        } 
        String::from_utf8(res).expect("from_bit<String>:invalid utf8")
    }
}