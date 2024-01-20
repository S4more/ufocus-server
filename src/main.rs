use dotenv::dotenv;

mod api;
mod gpt_wrapper;

#[tokio::main]
async fn main() {
    // Main program logic goes here
    dotenv().ok();
    println!("Found dotenv file.");

    api::start_api().await
}
