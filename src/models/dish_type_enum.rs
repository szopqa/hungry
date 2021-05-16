use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

#[derive(Debug, EnumIter, PartialEq, Copy, Clone)]
pub enum DishType {
    BREAKFAST,
    DINNER,
    LUNCH,
    SUPPER
}

impl From <&str> for DishType {
    fn from(dish_type: &str) -> Self {
        let _chosen_dish_type = dish_type.to_lowercase();
        match &_chosen_dish_type[..] {
            "dinner" => DishType::DINNER,
            "breakfast" => DishType::BREAKFAST,
            "lunch" => DishType::LUNCH,
            "supper" => DishType::SUPPER,
            _ => {
                let mut error_msg = String::from("Invalid dish type value. Only following values are allowed: \n");
                for allowed_dish_type in DishType::iter() {
                    error_msg.push_str(format!("{:?}\n", allowed_dish_type).as_str());
                }

                panic!(error_msg);
            }
        }
    }
}
