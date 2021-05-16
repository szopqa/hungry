use scraper::{Selector};

use crate::models::dish_type_enum::{DishType};

#[derive(Clone, Debug)]
pub struct SubPageConfig {
    pub _ingredients_selector: Selector
}

#[derive(Clone, Debug)]
pub struct PageConfig {
    pub _relative_uri: String,
    pub _menu_items_selector: Selector,
    pub _next_page_selector: Selector,
    pub _sub_page_config: SubPageConfig,
    pub _sub_page_dishes_category: DishType
}

impl PageConfig {
    pub fn is_for_dish_type(&self, _dish_type: DishType) -> bool {
        self._sub_page_dishes_category == _dish_type
    }
}
