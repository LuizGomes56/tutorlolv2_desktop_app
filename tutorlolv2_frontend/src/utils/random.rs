use tutorlolv2_imports::{
    CHAMPION_ID_TO_NAME, CHAMPION_POSITIONS, ITEM_ID_TO_NAME, RECOMMENDED_ITEMS, RECOMMENDED_RUNES,
    RUNE_ID_TO_NAME,
};
use tutorlolv2_imports::{ChampionId, ItemId, RuneId};
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

impl RandomInput {
    pub fn recommended_items(champion_id: ChampionId) -> &'static [tutorlolv2_imports::ItemId] {
        unsafe {
            let positions = CHAMPION_POSITIONS.get_unchecked(champion_id as usize);
            let random_index = RandomInput::rand_num_limited(positions.len() as f64) as usize;
            let position = positions.get_unchecked(random_index);
            RECOMMENDED_ITEMS
                .get_unchecked(champion_id as usize)
                .get_unchecked(*position as usize)
        }
    }
    pub fn recommended_runes(champion_id: ChampionId) -> &'static [tutorlolv2_imports::RuneId] {
        unsafe {
            let positions = CHAMPION_POSITIONS.get_unchecked(champion_id as usize);
            let random_index = RandomInput::rand_num_limited(positions.len() as f64) as usize;
            let position = positions.get_unchecked(random_index);
            RECOMMENDED_RUNES
                .get_unchecked(champion_id as usize)
                .get_unchecked(*position as usize)
        }
    }
}

impl_random_input!(ChampionId, u8, CHAMPION_ID_TO_NAME);
impl_random_input!(ItemId, u16, ITEM_ID_TO_NAME);
impl_random_input!(RuneId, u8, RUNE_ID_TO_NAME);

impl RandomInput {
    #[inline]
    fn rand_num_limited(limit: f64) -> f64 {
        Math::floor(Math::random() * limit)
    }

    #[inline]
    pub fn rand_u8(n: u8) -> u8 {
        Self::rand_num_limited(n as f64) as u8
    }

    #[inline]
    pub fn rand_id() -> AttrValue {
        AttrValue::from((Self::rand_num_limited((u32::MAX) as f64) as usize).to_string())
    }
}
