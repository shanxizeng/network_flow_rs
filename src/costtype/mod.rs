//! 这个module用于实现不同类型的值相乘，可以支持图中的费用和流量使用不同的类型
//! （如费用使用f64类型，而流量使用u32类型）

/// 通过这个trait中的mul函数实现T和E类型相乘
/// 
/// 如果需要使用自己定义的数据类型，则需要实现这个trait，如
/// 
/// ```ignore
/// struct Complex {
///     x : f64,
///     y : f64
/// }
/// 
/// struct Mul_u32_Complex;
/// 
/// impl MulTE<u32, Complex> for Mul_u32_Complex {
///     fn mul(a : &u32, b : &Complex) -> Complex {
///         Complex {
///             x : a as f64 * b.x,
///             y : a as f64 * b.y
///         }
///     }
/// }
/// 
/// let x = Graph::<String, u32, Complex, Mul_u32_Complex>::new();
/// ```
pub trait MulTE<T, E> {
    /// 两个数相乘后得到一个和第二个数类型相同的值。
    /// 
    /// 这要求原则上b的精度应该大于a的精度。
    /// 
    /// 注意两个参数均为引用，使得可以更方便地作用于非Copy的type。
    /// 
    /// 基础的数据类型（u8-u128, i8-i128, f32, f64）之间的转化默认已经实现，如可以声明graph为
    /// 
    /// ```ignore
    ///     Graph::<String, u32, f64>
    /// ```
    fn mul(a :&T, b :&E) -> E;
}

// a implementation for primitary type
// defaultly, T is smaller than E
pub struct MulTEDefaultType;

macro_rules! MulTEPrim {
    ($lhs : ty, $rhs : ty) => {
        impl MulTE<$lhs, $rhs> for MulTEDefaultType {
            fn mul(a :&$lhs, b :&$rhs) -> $rhs {
                (*a) as $rhs * b
            }
        }
    };
}

MulTEPrim!(u8, u8);
MulTEPrim!(u8, u16);
MulTEPrim!(u8, u32);
MulTEPrim!(u8, u64);
MulTEPrim!(u8, u128);
MulTEPrim!(u8, i8);
MulTEPrim!(u8, i16);
MulTEPrim!(u8, i32);
MulTEPrim!(u8, i64);
MulTEPrim!(u8, i128);
MulTEPrim!(u8, f32);
MulTEPrim!(u8, f64);

MulTEPrim!(i8, u8);
MulTEPrim!(i8, u16);
MulTEPrim!(i8, u32);
MulTEPrim!(i8, u64);
MulTEPrim!(i8, u128);
MulTEPrim!(i8, i8);
MulTEPrim!(i8, i16);
MulTEPrim!(i8, i32);
MulTEPrim!(i8, i64);
MulTEPrim!(i8, i128);
MulTEPrim!(i8, f32);
MulTEPrim!(i8, f64);

MulTEPrim!(u16, u16);
MulTEPrim!(u16, u32);
MulTEPrim!(u16, u64);
MulTEPrim!(u16, u128);
MulTEPrim!(u16, i16);
MulTEPrim!(u16, i32);
MulTEPrim!(u16, i64);
MulTEPrim!(u16, i128);
MulTEPrim!(u16, f32);
MulTEPrim!(u16, f64);

MulTEPrim!(i16, u16);
MulTEPrim!(i16, u32);
MulTEPrim!(i16, u64);
MulTEPrim!(i16, u128);
MulTEPrim!(i16, i16);
MulTEPrim!(i16, i32);
MulTEPrim!(i16, i64);
MulTEPrim!(i16, i128);
MulTEPrim!(i16, f32);
MulTEPrim!(i16, f64);

MulTEPrim!(u32, u32);
MulTEPrim!(u32, u64);
MulTEPrim!(u32, u128);
MulTEPrim!(u32, i32);
MulTEPrim!(u32, i64);
MulTEPrim!(u32, i128);
MulTEPrim!(u32, f32);
MulTEPrim!(u32, f64);

MulTEPrim!(i32, u32);
MulTEPrim!(i32, u64);
MulTEPrim!(i32, u128);
MulTEPrim!(i32, i32);
MulTEPrim!(i32, i64);
MulTEPrim!(i32, i128);
MulTEPrim!(i32, f32);
MulTEPrim!(i32, f64);

MulTEPrim!(u64, u64);
MulTEPrim!(u64, u128);
MulTEPrim!(u64, i64);
MulTEPrim!(u64, i128);
MulTEPrim!(u64, f32);
MulTEPrim!(u64, f64);

MulTEPrim!(i64, u64);
MulTEPrim!(i64, u128);
MulTEPrim!(i64, i64);
MulTEPrim!(i64, i128);
MulTEPrim!(i64, f32);
MulTEPrim!(i64, f64);

MulTEPrim!(u128, u128);
MulTEPrim!(u128, i128);
MulTEPrim!(u128, f32);
MulTEPrim!(u128, f64);

MulTEPrim!(i128, u128);
MulTEPrim!(i128, i128);
MulTEPrim!(i128, f32);
MulTEPrim!(i128, f64);

MulTEPrim!(f32, f32);
MulTEPrim!(f32, f64);

MulTEPrim!(f64, f64);