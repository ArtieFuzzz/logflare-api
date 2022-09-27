use color_eyre::{
    eyre::{eyre, Context},
    Result,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Hello, World!");

    Ok(())
}
