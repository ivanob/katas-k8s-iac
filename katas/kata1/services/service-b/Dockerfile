# Build stage
FROM maven:3.9-eclipse-temurin-21 as builder

WORKDIR /app

COPY pom.xml .
RUN mvn dependency:go-offline

COPY src ./src
RUN mvn clean package -DskipTests

# Runtime stage
FROM eclipse-temurin:21-jre

WORKDIR /app

COPY --from=builder /app/target/service-b-1.0.0.jar app.jar

EXPOSE 8080

CMD ["java", "-jar", "app.jar"]
