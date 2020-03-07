use scraper::{Selector};

use crate::models::dish_type_enum::{DishType};

#[derive(Clone)]
pub struct SubpageConfig {
    pub _relative_uri: String,
    pub _menu_items_selector: Selector,
    pub _next_page_selector: Selector,
    pub _subpage_dishes_category: DishType
}

impl SubpageConfig {
    pub fn is_for_dish_type(&self, _dish_type: DishType) -> bool {
        self._subpage_dishes_category == _dish_type
    }
}
