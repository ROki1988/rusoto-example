extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_s3::{S3, S3Client};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

fn main() {
    let provider = DefaultCredentialsProvider::new().unwrap();
    let client = S3Client::new(default_tls_client().unwrap(), provider, Region::ApNortheast1);

    match client.list_buckets() {
        Ok(output) => {
            match output.buckets {
                Some(bucket_name_list) => {
                    println!("Tables in database:");

                    for bucket in bucket_name_list {
                        println!("{:?}", bucket.name);
                    }
                }
                None => println!("No tables in database!"),
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}