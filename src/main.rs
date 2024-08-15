mod sphere_radius;
mod system_info;

use crate::sphere_radius::sphere_radius_request::system_name_sphere;
use crate::system_info::system_information_request::system_name;
use std::io;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("enter (1)  for system information\nenter (2) for sphere radius search");
    // gets the name of the system that the person wants.

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let selection = input.trim();

    if selection == "1" {
        system_name().await?;
    } else {
        system_name_sphere().await?;
    }

    Ok(())
}
