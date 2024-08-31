use crate::math::{Mat4, Vec2, Vec3, Vec4};

pub trait ToBytes {
    fn to_bytes(&self) -> &[u8];
}
macro_rules! impl_tobytes {
    ($t:ty) => {
        impl ToBytes for $t {
            fn to_bytes(&self) -> &[u8] {
                unsafe {
                    std::slice::from_raw_parts(
                        self as *const _ as *const u8,
                        std::mem::size_of::<$t>(),
                    )
                }
            }
        }
    };
}

impl_tobytes!(i32);
impl_tobytes!(f32);
impl_tobytes!(Vec2<f32>);
impl_tobytes!(Vec3<f32>);
impl_tobytes!(Vec4<f32>);
impl_tobytes!(Mat4<f32>);

impl<T: ToBytes, const N: usize> ToBytes for &[T; N] {
    fn to_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(*self as *const _ as *const u8, std::mem::size_of::<T>() * N)
        }
    }
}

impl<T: ToBytes> ToBytes for &[T] {
    fn to_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ptr() as *const _ as *const u8,
                std::mem::size_of::<T>() * self.len(),
            )
        }
    }
}
