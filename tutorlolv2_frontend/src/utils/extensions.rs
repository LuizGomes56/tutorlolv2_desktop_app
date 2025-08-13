use core::convert::TryFrom;
use generated_code::{ChampionId, ItemId, RuneId};

pub trait StringExt {
    fn padding_chars(&self) -> String;
    fn concat_char(&self, c: char) -> String;
    fn to_sized_slice<const N: usize>(&self) -> [u8; N];
    fn first_char(&self) -> char;
}

impl StringExt for str {
    fn first_char(&self) -> char {
        self.chars().next().unwrap_or_default()
    }
    fn concat_char(&self, c: char) -> String {
        let mut s = String::with_capacity(self.len() + 1);
        s.push_str(self);
        s.push(c);
        s
    }
    fn padding_chars(&self) -> String {
        let mut out = [0u8; 3];
        let mut i = 0;
        let mut written = 0;
        for b in self.bytes() {
            if b != b'_' {
                if i >= 1 && written < 3 {
                    out[written] = b;
                    written += 1;
                }
                i += 1;
            }
        }
        String::from_utf8_lossy(&out[..written]).into_owned()
    }
    fn to_sized_slice<const N: usize>(&self) -> [u8; N] {
        let mut out = [0u8; N];
        let bytes = self.as_bytes();
        let len = bytes.len().min(N);
        out[..len].copy_from_slice(&bytes[..len]);
        out
    }
}

pub trait StringifyEnum {
    fn as_str(&self) -> &'static str;
}

#[macro_export]
macro_rules! impl_unsafe_cast {
    ($($ty:ident),*) => {
        paste::paste! {
            $(
                #[inline]
                fn [<from_ $ty _unchecked>](n: $ty) -> Self
                where
                    Self::Repr: TryFrom<$ty>,
                {
                    let v = <Self::Repr as TryFrom<$ty>>::try_from(n)
                        .ok()
                        .unwrap_or_else(|| unsafe { core::hint::unreachable_unchecked() });
                    unsafe { Self::from_repr_unchecked(v) }
                }

                #[inline]
                fn [<try_from_ $ty>](n: $ty) -> Option<Self>
                where
                    Self::Repr: TryFrom<$ty>,
                {
                    <Self::Repr as TryFrom<$ty>>::try_from(n)
                        .ok()
                        .map(|v| unsafe { Self::from_repr_unchecked(v) })
                }

                #[inline]
                fn [<into_ $ty _unchecked>](self) -> $ty
                where
                    Self::Repr: TryInto<$ty>,
                {
                    let repr = unsafe { core::mem::transmute_copy::<Self, Self::Repr>(&self) };
                    <Self::Repr as TryInto<$ty>>::try_into(repr)
                        .ok()
                        .unwrap_or_else(|| unsafe { core::hint::unreachable_unchecked() })
                }

                #[inline]
                fn [<try_into_ $ty>](self) -> Option<$ty>
                where
                    Self::Repr: TryInto<$ty>,
                {
                    let repr = unsafe { core::mem::transmute_copy::<Self, Self::Repr>(&self) };
                    <Self::Repr as TryInto<$ty>>::try_into(repr).ok()
                }
            )*
        }
    };
    (@$ty:ty, $repr:ty) => {
        impl $crate::utils::UnsafeCast for $ty {
            type Repr = $repr;
            #[inline]
            unsafe fn from_repr_unchecked(n: Self::Repr) -> Self {
                unsafe { core::mem::transmute::<$repr, Self>(n) }
            }
            unsafe fn into_repr_unchecked(self) -> Self::Repr {
                unsafe { core::mem::transmute(self) }
            }
        }
    };
}

macro_rules! define_unsafe_cast {
    () => {
        paste::paste! {
            pub trait UnsafeCast: Sized {
                type Repr: Copy;
                unsafe fn from_repr_unchecked(n: Self::Repr) -> Self;
                unsafe fn into_repr_unchecked(self) -> Self::Repr;

                impl_unsafe_cast!(u8, u16, u32, usize);
            }
            impl_unsafe_cast!(@ChampionId, u8);
            impl_unsafe_cast!(@RuneId, u8);
            impl_unsafe_cast!(@ItemId, u16);
        }
    };
}

define_unsafe_cast!();
