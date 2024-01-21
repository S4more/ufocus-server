use std::time::Instant;

use dotenv::dotenv;

mod api;
mod gpt_streaming;
mod gpt_wrapper;

const PAGE_CONTENT: &str = "Filters and Topics,Search settings,Ads,Search Results,Page Navigation,Footer Links Accessibility Links,Web Result with Site Links,Twitter Results,Complementary Results AWS Cloud Compute Service,AWS Compute Solutions,What is Cloud Computing?,AWS Storage Solutions,Cloud Computing Services - Amazon Web Services (AWS),AWS Management Console,Training and Certification,Free Tier,Amazon Web Services Login,Benefits - AWS,What is AWS? - Cloud Computing with AWS - Amazon Web Services,Salary: AWS in India 2024 - Glassdoor,Getting Started - Cloud Computing Tutorials for Building on AWS,Amazon Web Services,Welcome | AWS Training & Certification,Amazon Web Services (@awscloud) · X,AWS Training & Certification,Amazon Web Services (AWS),American Welding Society (AWS) - Welding Excellence ...,AWS re:Invent | Amazon Web Services,More results,Try again,Description    _Servi Wikipedia https en wikipedia org wiki Amazon_Web_Servi Amazon Web Services AWS Services History Availability topology Pop lofts Welc";

#[tokio::main]
async fn main() {
    // Main program logic goes here
    dotenv().ok();
    println!("Found dotenv file.");

    // let start = Instant::now();
    // gpt_wrapper::query_gpt(PAGE_CONTENT.to_string())
    //     .await
    //     .unwrap();
    // let duration = start.elapsed();
    // println!("Time elapsed in query() is: {:?}", duration);

    // let start = Instant::now();
    // gpt_streaming::stream_gpt(PAGE_CONTENT.to_string())
    //     .await
    //     .unwrap();
    // let duration = start.elapsed();
    // println!("Time elapsed in stream() is: {:?}", duration);

    api::start_api().await
}
