use generated_code::{
    CHAMPION_ID_TO_NAME, CHAMPION_POSITIONS, ITEM_ID_TO_NAME, RECOMMENDED_ITEMS, RUNE_ID_TO_NAME,
};
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

impl RandomInput {
    pub fn recommended_items(champion_id: ChampionId) -> Vec<ItemId> {
        unsafe {
            let positions = CHAMPION_POSITIONS.get_unchecked(champion_id as usize);
            let random_index = RandomInput::rand_num_limited(positions.len() as f64) as usize;
            let position = positions.get_unchecked(random_index);
            RECOMMENDED_ITEMS
                .get_unchecked(champion_id as usize)
                .get_unchecked(*position as usize)
                .to_vec()
        }
    }
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
