resource "aws_ecr_repository" "service_a" {
  name                 = "service-a"
  image_tag_mutability = "MUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }

  tags = {
    Name = "kata1-service-a"
  }
}

resource "aws_ecr_repository" "service_b" {
  name                 = "service-b"
  image_tag_mutability = "MUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }

  tags = {
    Name = "kata1-service-b"
  }
}

output "ecr_service_a_url" {
  value       = aws_ecr_repository.service_a.repository_url
  description = "ECR repository URL for service-a"
}

output "ecr_service_b_url" {
  value       = aws_ecr_repository.service_b.repository_url
  description = "ECR repository URL for service-b"
}
