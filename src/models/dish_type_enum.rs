#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub enum DishType {
    BREAKFAST,
    DINNER
}

impl From <&str> for DishType {
    fn from(dish_type: &str) -> Self {
        let _chosen_dish_type = dish_type.to_lowercase();
        match &_chosen_dish_type[..] {
            "dinner" => DishType::DINNER,
            "breakfast" => DishType::BREAKFAST,
            _ => panic!("Unsupported dish type!")
        }
    }
}
