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
      Terraform = true
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

data "archive_file" "file" {
  type        = "zip"
  source_dir  = var.source_dir
  output_path = var.output_path
}

resource "aws_lambda_function" "fn" {
  function_name    = var.function_name
  role             = aws_iam_role.lambda_exec.arn
  filename         = data.archive_file.file.output_path
  source_code_hash = data.archive_file.file.output_base64sha256
  handler          = var.handler_name
  runtime          = var.runtime
  architectures    = [var.architecture]
  publish          = true

  environment {
    variables = {
      SLACK_INCOMING_WEBHOOK_PATH = var.slack_incoming_webhook_path
    }
  }
}

resource "aws_lambda_function_url" "webhook_latest" {
  authorization_type = "NONE"
  function_name      = aws_lambda_function.fn.function_name
  cors {
    allow_methods = ["*"]
    allow_origins = ["*"]
  }
}
