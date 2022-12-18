terraform {
  required_providers {
    aws = {
      version = ">= 4.13.0"
    }
  }
}

provider "aws" {
  default_tags {
    tags = {
      Terraform   = true
      Application = "gitlab-slack-notifier"
    }
  }
}
terraform {
  backend "s3" {
    dynamodb_table = "terraform-lock"
    encrypt        = true
    key            = "terraform/aws-lambda/terraform.tfstate"
    region         = "ap-northeast-2"
    bucket         = "lechuckroh-terraform-state"
  }
}

##########################################
# IAM
##########################################

resource "aws_iam_role" "lambda_exec" {
  name               = var.role_name
  description        = "A role to execute lambda for gitlab-slack-notifier"
  assume_role_policy = jsonencode({
    Version   = "2012-10-17"
    Statement = [
      {
        Effect    = "Allow",
        Principal = {
          Service = ["lambda.amazonaws.com"]
        },
        Action = "sts:AssumeRole"
      }
    ]
  })
}

resource "aws_iam_policy" "lambda_logging" {
  name        = var.policy_name
  description = "A policy for cloudWatchLogs"
  policy      = jsonencode({
    Version : "2012-10-17",
    Statement : [
      {
        Effect : "Allow",
        Action : [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents"
        ],
        Resource : ["arn:aws:logs:*:*:*"]
      }
    ],
  })
}

resource "aws_iam_role_policy_attachment" "lambda_logs" {
  role       = aws_iam_role.lambda_exec.name
  policy_arn = aws_iam_policy.lambda_logging.arn
}


##########################################
# Lambda
##########################################

locals {
  nodejs_count = var.lambda_language == "nodejs" ? 1 : 0
  rust_count   = var.lambda_language == "rust" ? 1 : 0
}

data "archive_file" "nodejs" {
  count       = local.nodejs_count
  type        = "zip"
  source_dir  = var.nodejs_source_dir
  output_path = var.nodejs_output_path
}

resource "aws_lambda_function" "nodejs" {
  count            = local.nodejs_count
  function_name    = var.nodejs_function_name
  role             = aws_iam_role.lambda_exec.arn
  filename         = data.archive_file.nodejs.output_path
  source_code_hash = data.archive_file.nodejs.output_base64sha256
  handler          = var.nodejs_handler_name
  runtime          = var.nodejs_runtime
  architectures    = var.nodejs_architectures
  publish          = true

  environment {
    variables = {
      SLACK_INCOMING_WEBHOOK_PATH = var.slack_incoming_webhook_path
    }
  }

  tags = {
    Language = "node.js"
  }
}

resource "aws_lambda_function_url" "nodejs" {
  count              = local.nodejs_count
  authorization_type = "NONE"
  function_name      = aws_lambda_function.nodejs.function_name
  cors {
    allow_methods = ["*"]
    allow_origins = ["*"]
  }

  tags = {
    Language = "node.js"
  }
}

data "local_file" "rust" {
  count    = local.rust_count
  filename = var.rust_zip_filename
}

resource "aws_lambda_function" "rust" {
  count            = local.rust_count
  function_name    = var.rust_function_name
  role             = aws_iam_role.lambda_exec.arn
  filename         = data.local_file.rust.filename
  source_code_hash = data.local_file.rust.content_base64
  handler          = var.rust_handler_name
  runtime          = var.rust_runtime
  architectures    = var.rust_architectures
  publish          = true

  environment {
    variables = {
      RUST_BACKTRACE              = 1
      SLACK_INCOMING_WEBHOOK_PATH = var.slack_incoming_webhook_path
    }
  }

  tags = {
    Language = "rust"
  }
}

resource "aws_lambda_function_url" "rust" {
  count              = local.rust_count
  authorization_type = "NONE"
  function_name      = aws_lambda_function.rust.function_name
  cors {
    allow_methods = ["*"]
    allow_origins = ["*"]
  }

  tags = {
    Language = "rust"
  }
}
