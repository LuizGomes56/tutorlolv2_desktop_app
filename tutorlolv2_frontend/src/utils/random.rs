use generated_code::{CHAMPION_ID_TO_NAME, ITEM_ID_TO_NAME, RUNE_ID_TO_NAME};
use generated_code::{ChampionId, ItemId, RuneId};
use web_sys::js_sys::Math;
use yew::AttrValue;

pub struct RandomInput;

macro_rules! impl_random_input {
    ($ty:ty, $size:ty, $constvar:ident) => {
        impl RandomInput {
            paste::paste! {
                pub fn [<$ty:snake>]() -> $ty {
                    unsafe {
                        let random_number = Self::rand_num_limited($constvar.len() as f64);
                        std::mem::transmute::<_, $ty>(random_number as $size)
                    }
                }
            }
        }
    };
}

impl_random_input!(ChampionId, u8, CHAMPION_ID_TO_NAME);
impl_random_input!(ItemId, u16, ITEM_ID_TO_NAME);
impl_random_input!(RuneId, u8, RUNE_ID_TO_NAME);

impl RandomInput {
    #[inline]
    pub fn rand_num_limited(limit: f64) -> f64 {
        Math::floor(Math::random() * limit)
    }

    #[inline]
    pub fn rand_id() -> AttrValue {
        AttrValue::from((Self::rand_num_limited((1 << 20) as f64) as usize).to_string())
    }
}
