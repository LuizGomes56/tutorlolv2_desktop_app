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
    fn recommended<const N: usize, const L: usize, T>(
        champion_id: ChampionId,
        constant: [[&'static [T]; N]; L],
    ) -> &'static [T] {
        unsafe {
            let positions = CHAMPION_POSITIONS.get_unchecked(champion_id as usize);
            let random_index = RandomInput::rand_num_limited(positions.len() as f64) as usize;
            let position = positions.get_unchecked(random_index);
            constant
                .get_unchecked(champion_id as usize)
                .get_unchecked(*position as usize)
        }
    }
    pub fn recommended_items(champion_id: ChampionId) -> &'static [ItemId] {
        Self::recommended(champion_id, RECOMMENDED_ITEMS)
    }
    pub fn recommended_runes(champion_id: ChampionId) -> &'static [RuneId] {
        Self::recommended(champion_id, RECOMMENDED_RUNES)
    }
}

impl_random_input!(ChampionId, u8, CHAMPION_ID_TO_NAME);
impl_random_input!(ItemId, u16, ITEM_ID_TO_NAME);
impl_random_input!(RuneId, u8, RUNE_ID_TO_NAME);

impl RandomInput {
    fn rand_num_limited(limit: f64) -> f64 {
        Math::floor(Math::random() * limit)
    }

    pub fn rand_u8(n: u8) -> u8 {
        Self::rand_num_limited(n as f64) as u8
    }

    pub fn rand_id() -> AttrValue {
        AttrValue::from((Self::rand_num_limited(f64::MAX) as usize).to_string())
    }
}
