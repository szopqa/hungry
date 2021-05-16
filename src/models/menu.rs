use super::dish_type_enum::DishType;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Ingredient {
    pub _name: String,
    pub _amount: f32,
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub _dish_name: String,
    pub _dish_absolute_path: String,
    pub _dish_relative_path: String,
    pub _ingredients: Vec<Ingredient>,
}

impl MenuItem {
    pub fn update_with_ingredients(&mut self, _ingredients: Vec<Ingredient>) {
        self._ingredients = _ingredients;
    }
}

#[derive(Debug)]
pub struct Menu {
    pub _dish_type: DishType,
    pub _dishes: Vec<MenuItem>,
}

impl Menu {
    pub fn new(_dish_type: DishType, _dishes: Vec<MenuItem>) -> Self {
        Self {
            _dish_type,
            _dishes,
        }
    }

    pub fn pick_num_of_elements(&mut self, _num_of_dishes: usize) -> Self {
        let mut rng = &mut rand::thread_rng();

        Self {
            _dish_type: self._dish_type,
            _dishes: self
                ._dishes
                .choose_multiple(&mut rng, _num_of_dishes)
                .cloned()
                .collect(),
        }
    }

    fn describe_single_menu_item(_menu_item: &MenuItem)  {
        println!("{}\n", _menu_item._dish_name.to_uppercase());
        println!(
            "      Full recipe available at: \n        {}\n",
            _menu_item._dish_absolute_path
        );
        println!("       Ingredients:");
        for _each_ingredient in &_menu_item._ingredients {
            println!("          * {}", _each_ingredient._name);
        }
    }

    pub fn describe(&self)  {
        println!(
            "\nMenu for {:?} containing {} elements",
            self._dish_type,
            self._dishes.len()
        );

        for _each_menu_item in &self._dishes {
            println!();
            Self::describe_single_menu_item(&_each_menu_item);
        }
    }
}
