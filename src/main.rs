use clap::Clap;
extern crate strum;
#[macro_use]
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
    kwestiasmaku::{KwestiasmakuDataSource}
};

#[derive(Clap)]
#[clap(version = "1.0", author = "Michał Sz.")]
struct Opts {
    /// Sets type of dish you want to generate menu for
    #[clap(short = "t", long = "type", default_value = "dinner")]
    dish_type: String,
    /// Number of meals in generated menu
    #[clap(short = "n", long = "number", default_value = "4")]
    num_of_dishes_in_menu: i32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let _dish_type: DishType = opts.dish_type.as_str().into();
    let _num_of_dishes_in_menu: i32 = opts.num_of_dishes_in_menu;

    let _kwestiasmaku_data_source = KwestiasmakuDataSource::new("https://www.kwestiasmaku.com");
    let _menu = _kwestiasmaku_data_source.get_menu_for_dish_type(_dish_type).await?;

    println!("{:?}", _menu);

    Ok(())
}
