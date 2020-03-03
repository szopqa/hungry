use super::dish_type_enum::{DishType};

#[derive(Debug)]
pub struct MenuItem {
    pub _dish_name: String,
    pub _dish_path: String,
}

#[derive(Debug)]
pub struct Menu {
    pub _dish_type: DishType,
    pub _dishes: Vec<MenuItem>
}

impl Menu {
    pub fn new(_dish_type: DishType, _dishes: Vec<MenuItem>) -> Self {
        Self {
            _dish_type,
            _dishes
        }
    }
}