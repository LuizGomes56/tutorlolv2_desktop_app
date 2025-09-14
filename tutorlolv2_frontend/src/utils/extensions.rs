use core::convert::TryFrom;
use tutorlolv2_imports::{
    CHAMPION_FORMULAS, CHAMPION_ID_TO_NAME, ChampionId, ITEM_FORMULAS, ITEM_ID_TO_NAME, ItemId,
    RUNE_FORMULAS, RUNE_ID_TO_NAME, RuneId,
};

use crate::components::ImageType;

pub trait StringExt {
    fn concat_char(&self, c: char) -> String;
}

impl StringExt for str {
    fn concat_char(&self, c: char) -> String {
        let mut s = String::with_capacity(self.len() + 1);
        s.push_str(self);
        s.push(c);
        s
    }
}

pub trait StringifyEnum {
    fn as_str(self) -> &'static str;
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
                    Self::from_repr_unchecked(v)
                }

                #[inline]
                fn [<try_from_ $ty>](n: $ty) -> Option<Self>
                where
                    Self::Repr: TryFrom<$ty>,
                {
                    <Self::Repr as TryFrom<$ty>>::try_from(n)
                        .ok()
                        .map(|v| Self::from_repr_unchecked(v))
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
            fn from_repr_unchecked(n: Self::Repr) -> Self {
                unsafe { core::mem::transmute(n) }
            }
            #[inline]
            fn into_repr_unchecked(self) -> Self::Repr {
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
                fn from_repr_unchecked(n: Self::Repr) -> Self;
                fn into_repr_unchecked(self) -> Self::Repr;

                impl_unsafe_cast!(u8, u16, u32, usize);
            }
            impl_unsafe_cast!(@ChampionId, u8);
            impl_unsafe_cast!(@RuneId, u8);
            impl_unsafe_cast!(@ItemId, u16);
        }
    };
}

define_unsafe_cast!();

pub trait ImportedEnum {
    const ID_TO_NAME: &'static [&'static str];
    const OFFSETS: &'static [(u32, u32)];
    fn into_image_type_unchecked(index: usize) -> ImageType;
}

impl ImportedEnum for ChampionId {
    const ID_TO_NAME: &'static [&'static str] = &CHAMPION_ID_TO_NAME;
    const OFFSETS: &'static [(u32, u32)] = &CHAMPION_FORMULAS;
    fn into_image_type_unchecked(index: usize) -> ImageType {
        ImageType::Champions(ChampionId::from_usize_unchecked(index))
    }
}

impl ImportedEnum for ItemId {
    const ID_TO_NAME: &'static [&'static str] = &ITEM_ID_TO_NAME;
    const OFFSETS: &'static [(u32, u32)] = &ITEM_FORMULAS;
    fn into_image_type_unchecked(index: usize) -> ImageType {
        ImageType::Items(ItemId::from_usize_unchecked(index))
    }
}

impl ImportedEnum for RuneId {
    const ID_TO_NAME: &'static [&'static str] = &RUNE_ID_TO_NAME;
    const OFFSETS: &'static [(u32, u32)] = &RUNE_FORMULAS;
    fn into_image_type_unchecked(index: usize) -> ImageType {
        ImageType::Runes(RuneId::from_usize_unchecked(index))
    }
}
