use clap::Clap;
extern crate strum;
extern crate strum_macros;

mod clients;
mod providers;

mod models;
use models::{
    dish_type_enum::{DishType}
};

mod data_sources;
use data_sources:: {
    data_source::{DataSource},
    kwestiasmaku::{KSDataSource}
};

#[derive(Clap)]
#[clap(version = "0.3.0", author = "Michał Sz.")]
struct Opts {
    /// Sets type of dish you want to generate menu for
    #[clap(short = "f", long = "for", default_value = "dinner")]
    dish_type: String,
    /// Number of meals in generated menu
    #[clap(short = "n", long = "number", default_value = "7")]
    num_of_dishes_in_menu: usize
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let _dish_type: DishType = opts.dish_type.as_str().into();
    let _num_of_dishes_in_menu: usize = opts.num_of_dishes_in_menu;

    let _ks_data_source = KSDataSource::new("https://www.kwestiasmaku.com");

    let _generated_menu = _ks_data_source
        .get_menu_for_dish_type(_dish_type).await?
        .pick_num_of_elements(_num_of_dishes_in_menu);
        
    _ks_data_source
        .get_ingredients_for_menu(_generated_menu).await?
        .describe();

    Ok(())
}
