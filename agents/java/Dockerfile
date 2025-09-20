FROM openjdk:17-jdk-slim

WORKDIR /app

# Copy Mon-X agent
COPY MonXAgent.java /app/src/main/java/com/monx/agent/
COPY pom.xml /app/
COPY application.yml /app/src/main/resources/

# Install Maven and build
RUN apt-get update && apt-get install -y maven && \
    mkdir -p src/main/java/com/monx/agent && \
    mkdir -p src/main/resources && \
    mvn clean package -DskipTests

# Create simple Spring Boot app
RUN echo 'package com.monx.agent; \
import org.springframework.boot.SpringApplication; \
import org.springframework.boot.autoconfigure.SpringBootApplication; \
import org.springframework.scheduling.annotation.EnableScheduling; \
@SpringBootApplication @EnableScheduling \
public class MonXAgentApp { \
    public static void main(String[] args) { \
        SpringApplication.run(MonXAgentApp.class, args); \
    } \
}' > src/main/java/com/monx/agent/MonXAgentApp.java && \
    mvn clean package -DskipTests

EXPOSE 8080

CMD ["java", "-jar", "target/monx-agent-1.0.0.jar"]
