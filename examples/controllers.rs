use std::error::Error;

use openrgb::OpenRGB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to local server
    let client = OpenRGB::connect().await?;

    // query controllers count
    let controllers = client.get_controller_count().await?;
    println!("{} controllers", controllers);

    // query and print each controller data
    for controller_id in 0..controllers {
        if controller_id == 15 {
            println!("getting 16");
        }
        println!(
            "controller {}: {:#?}",
            controller_id,
            client.get_controller(controller_id).await?
        );
    }

    Ok(())
}
