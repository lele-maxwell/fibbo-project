use std::{env, result};
use crate::post_comment::post_comment;
use fib_calculator::fibbo;
use num_bigint::ToBigInt;
use octocrab::{ models::{ repos::DiffEntry, pulls::PullRequest, repos::Content }, Octocrab, Page };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let github_repository = env
        ::var("GITHUB_REPOSITORY")
        .unwrap_or_else(|_| "t-desmond/fibbot".to_string());
    let github_repository = github_repository.split("/").collect::<Vec<&str>>();
    let owner = github_repository[0];
    let repo = github_repository[1];

    let pr = octocrab::instance().pulls(owner, repo).list_files(1).await?;
    println!("{:?}", pr);
    let path = &pr.items.first().unwrap().patch.clone().unwrap();
    let numbers = extract_numbers::extract_numbers(&path);
    println!("{:?}", numbers);

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <enable_fib> <max_threshold>", args[0]);
        return Ok(());
    }

    let enable_fib = &args[1];
    let max_threshold = &args[2];

    println!("\n enable_fib: {}", enable_fib);
    println!("\n max_threshold: {}", max_threshold);

    if enable_fib == "true" {
        // Your Fibonacci logic here
        let pr = octocrab::instance().pulls(owner, repo).list_files(1).await?;
        println!("{:?}", pr);
        let path = &pr.items.first().unwrap().patch.clone().unwrap();
        let numbers = extract_numbers::extract_numbers(&path);
        let mut result:Vec<i32>= Vec::new();

        for num in numbers {
            //println!("{}", num);
            //std::io::stdin().read_line(&mut max_threshold).unwrap();
            //  let max_theshold = max_threshold.trim().parse().unwrap();

            if max_threshold.parse::<i32>().unwrap() > num {
                //let num = num.to_bigint();
                let fib = fibbo(num);
                println!("\n the fib of {} is : {} \n", num, fib);
                let pr_content= format!("{:?}", result);
                post_comment(pr_content.as_str()).await?;
                result.push(fib);
            } else if max_threshold.parse::<i32>().unwrap() < num {
                println!("\n number() is greater than (max_threshold){} \n", num);
            }
            

            // std::io::stdin().read_line(&mut num1).unwrap();
            // let num1: f64 = num1.trim().parse().unwrap();
        }

        println!("\n Fibonacci program is enabled with max threshold: {} \n ", max_threshold);
        // Example Fibonacci function call
    } else {
        println!("\n Fibonacci program is disabled");
        println!("\n enable your program with a true argument");
    }
    let pr_number: u64 = match env::var("PR_NUMBER") 
    {
        
        Ok(pr_str) if pr_str.is_empty() => pr_str.parse::<u64>().expect("Invalid PR_NUMBER"),
        _ => {
            println!("PR_NUMBER() environment variable is not set or is invalid. Defaulting to PR number 1.");
            1
        }
    };



    Ok(())
}
mod fib_calculator;
mod extract_numbers;
mod extract_pr_content;
mod post_comment;








