use super::dish_type_enum::{DishType};
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
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

    pub fn pick_num_of_elements(&mut self, _num_of_dishes: usize) -> Self {
        let mut rng = &mut rand::thread_rng();

        Self {
            _dish_type: self._dish_type,
            _dishes: self._dishes.choose_multiple(&mut rng, _num_of_dishes).cloned().collect()
        }
    }

    fn describe_single_menu_item(_menu_item: &MenuItem) -> () {
        println!("{}", _menu_item._dish_name.to_uppercase());
        println!("      Full recipe available at: {}", _menu_item._dish_path);
    }

    pub fn describe(&self) -> () {
        println!("\nMenu for {:?} containing {} elements", self._dish_type, self._dishes.len());

        for _each_menu_item in &self._dishes {
            println!("");
            Self::describe_single_menu_item(&_each_menu_item);
        }
    }
}