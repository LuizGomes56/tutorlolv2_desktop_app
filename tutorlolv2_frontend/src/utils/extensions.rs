use crate::components::ImageType;
use tutorlolv2_imports::{
    CHAMPION_FORMULAS, CHAMPION_ID_TO_NAME, ChampionId, ITEM_FORMULAS, ITEM_ID_TO_NAME, ItemId,
    RUNE_FORMULAS, RUNE_ID_TO_NAME, RuneId,
};

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

pub trait ToStaticStr {
    fn as_static_str(&self) -> &'static str;
}

pub trait IndexCast {
    fn from_usize_unchecked(index: usize) -> Self;
    fn into_usize(self) -> usize;
}

#[macro_export]
macro_rules! impl_unsafe_cast {
    ($ty:ty, $repr:ty) => {
        impl $crate::utils::IndexCast for $ty {
            #[inline]
            fn from_usize_unchecked(index: usize) -> Self {
                unsafe { core::mem::transmute(index as $repr) }
            }
            #[inline]
            fn into_usize(self) -> usize {
                self as usize
            }
        }
    };
}

impl_unsafe_cast!(ChampionId, u8);
impl_unsafe_cast!(ItemId, u16);
impl_unsafe_cast!(RuneId, u8);

pub trait ImportedEnum: Copy + IndexCast + PartialEq + 'static {
    const ID_TO_NAME: &'static [&'static str];
    const OFFSETS: &'static [(u32, u32)];
    fn into_image_type_unchecked(index: usize) -> ImageType;
    fn into_image_type(self) -> ImageType;
}

macro_rules! impl_import_enum {
    ($field:ident) => {
        paste::paste! {
            impl ImportedEnum for [<$field Id>] {
                const ID_TO_NAME: &'static [&'static str] = &[<$field:upper _ID_TO_NAME>];
                const OFFSETS: &'static [(u32, u32)] = &[<$field:upper _FORMULAS>];
                #[inline(always)]
                fn into_image_type_unchecked(index: usize) -> ImageType {
                    Self::from_usize_unchecked(index).into_image_type()
                }
                #[inline(always)]
                fn into_image_type(self) -> ImageType {
                    ImageType::$field(self)
                }
            }
        }
    };
}

impl_import_enum!(Champion);
impl_import_enum!(Item);
impl_import_enum!(Rune);
