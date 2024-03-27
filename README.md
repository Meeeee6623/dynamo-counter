# DynamoDB Command Counter

This project is a Rust-based AWS Lambda function using the cargo-lambda framework, designed to work with Amazon DynamoDB to implement a simple counter. Whenever a specific command is received in the JSON payload of a Lambda invocation, this function increments a "count" value in a DynamoDB table for the corresponding "command" key. It's a straightforward way to track how many times different commands are used.

## Features

- **AWS Lambda Integration:** Built with Rust and cargo-lambda for seamless deployment to AWS Lambda.
- **DynamoDB Backend:** Utilizes Amazon DynamoDB for reliable, scalable storage of command counts.
- **Automatic Count Increment:** Each invocation with a specific command increments its count in the DynamoDB table.
- **Simple API:** Expects a JSON payload with a "command" field and responds with the updated count for that command.

## Prerequisites

To use this project, you'll need:

- Rust and cargo-lambda installed on your development machine.
- AWS CLI configured with appropriate access credentials and permissions.
- An Amazon DynamoDB table named `my-table` with a primary key named `command`.

## Setup

### Clone the Repository

First, clone this repository to your local machine:

```bash
git clone https://github.com/Meeeee6623/dynamo-counter
cd dynamo-counter
```

### Create the DynamoDB Table

If you haven't already created the DynamoDB table, you can do so using the AWS CLI:

```sh
aws dynamodb create-table \
    --table-name my-table \
    --attribute-definitions AttributeName=command,AttributeType=S \
    --key-schema AttributeName=command,KeyType=HASH \
    --billing-mode PAY_PER_REQUEST
```

### Build and Deploy

To build and deploy the function to AWS Lambda, use cargo-lambda:

```bash
cargo lambda build --release
cargo lambda deploy --iam-role <your-iam-role-arn>
```

Replace `<your-iam-role-arn>` with the ARN of the IAM role that your Lambda function will assume. This role needs to have permissions to access DynamoDB.

## Usage

Invoke the Lambda function with a JSON payload specifying the "command". For example:

```json
{
  "command": "test-command"
}
```

The Lambda function will increment the count for `exampleCommand` in the DynamoDB table and return the updated count in the response.

## Response

The function responds with a JSON object containing the `req_id` and a message indicating the command received and its current count:

```json
{
  "req_id": "request-id-value",
  "command":"test-command",
  "count": 1
}
```

## Local Testing

For local testing, you can use cargo-lambda's support for running Lambda functions on your development machine. Ensure you have AWS credentials configured for a user with access to DynamoDB:

```bash
cargo lambda watch
```

Then, invoke the function with a test event JSON file or using a tool like Postman or curl to send requests to the local server started by cargo-lambda.
