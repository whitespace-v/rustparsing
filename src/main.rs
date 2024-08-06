use std::error::Error;
mod ninja;


fn main() -> Result<(), Box<dyn Error>>{
    let metadata = ninja::ninja_auth::auth()?;
    let params = ninja::collector::collect(metadata)?;
    let cars = ninja::car_page_parser::parse(metadata, params)?;
    ninja::database::send(cars)?;
    Ok(())
}
