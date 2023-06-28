//! 实现对图进行输入和输出的module
//! 

/// 将数据从原来的形式与字节形式之间进行转换
/// 
/// 对基础的数字类型、Vec、String进行了实现
/// 
/// 对于其他自定义类型，需要手动实现该trait，如：
/// 
/// ```
/// use network_flow::io::BitIO;
/// use std::mem::size_of;
/// struct MyStruct {
///     a : String,
///     b : u32,
///     c : Vec<String>
/// }
/// impl BitIO for MyStruct {
///     fn to_bit(&self) -> Vec<u8> {
///         let mut temp = self.a.to_bit();
///         let mut res = temp.len().to_bit();
///         res.append(&mut temp);
///         res.append(&mut self.b.to_bit());
///         res.append(&mut self.c.to_bit());
///         res
///     }
///     fn from_bit(a : &[u8]) -> Self {
///         let mut temp = size_of::<usize>();
///         let len = usize::from_bit(a);
///         let temp1 = String::from_bit(&a[temp..]);
///         temp = temp + len;
///         let temp2 = u32::from_bit(&a[temp..]);
///         temp = temp + size_of::<u32>();
///         let temp3 = Vec::<String>::from_bit(&a[temp..]);
///         MyStruct { a: temp1, b: temp2, c: temp3 }
///     }
/// }
/// ```
/// 
/// 使用实现了该trait的类型构建的图，可以进行自动的文件输入输出，保存当前图中的状态或者读取原来的图的状态。
pub trait BitIO {
    /// 转为字节形式
    fn to_bit(&self) -> Vec<u8>;
    /// 从字节形式生成
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