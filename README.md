# Python Template
---
This repo is for the mini-project 2 for Cloud Computing at Duke University

Create a simple AWS Lambda function that processes data.
---

Project Requirements:

* Rust Lambda Function using Cargo Lambda
* Process and transform sample data

Points of Focus
* Lambda functionality
* API Gateway integration
* Data processing

---
## Lambda Function
`main.rs` is an AWS Lambda function written in Rust. It takes three inputs: glucose (int), units(int), and outcome(binary).
It returns percent of glucose with respect to units and product of glucose and units (as json).   

Requirements for this AWS Lambda function to work

You must have: 
* Rust programming language
* Cargo Lambda for AWS Lambda must be installed, allowing deployment of the lambda function
* AWS account (you must have correct permissions for aws lambda)

## Installation and Deployment of lambda function

* Clone this repo (specifically the new-lambda-project)
* Install Rust, Cargo, and AWS CLI (make sure to configure!)
* Set up AWS credentials (mainly access keys)
* Deploy the lambda function written in rust (`cargo lambda build` followed by `cargo lambda deploy`)
* Finally, to trigger the Lambda function, we need to set up the api gateway to run the lambda function.

Example Input: 
{
  "glucose": 5,
  "units": "10",
  "outcome": 1
}


Example Response: 
{
  "percent": .5,
  "product": 50
}


