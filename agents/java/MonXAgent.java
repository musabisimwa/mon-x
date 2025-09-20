package com.monx.agent;

import org.springframework.boot.actuate.health.Health;
import org.springframework.boot.actuate.health.HealthIndicator;
import org.springframework.stereotype.Component;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.web.client.RestTemplate;
import org.springframework.http.HttpEntity;
import org.springframework.http.HttpHeaders;
import org.springframework.http.MediaType;

import java.lang.management.ManagementFactory;
import java.lang.management.MemoryMXBean;
import java.lang.management.OperatingSystemMXBean;
import java.time.Instant;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ThreadLocalRandom;

@Component
public class MonXAgent implements HealthIndicator {
    
    @Value("${monx.kafka.bootstrap-servers:localhost:9092}")
    private String kafkaServers;
    
    @Value("${monx.agent.id:${spring.application.name:unknown-app}}")
    private String agentId;
    
    private final RestTemplate restTemplate = new RestTemplate();
    private final OperatingSystemMXBean osBean = ManagementFactory.getOperatingSystemMXBean();
    private final MemoryMXBean memoryBean = ManagementFactory.getMemoryMXBean();
    
    @Scheduled(fixedRate = 5000) // Every 5 seconds
    public void sendMetrics() {
        try {
            sendMetric("cpu", getCpuUsage());
            sendMetric("memory", getMemoryUsage());
            sendMetric("load", osBean.getSystemLoadAverage());
        } catch (Exception e) {
            sendLog("ERROR", "Failed to send metrics: " + e.getMessage());
        }
    }
    
    @Scheduled(fixedRate = 10000) // Every 10 seconds
    public void sendLogs() {
        try {
            // Simulate application logs
            String[] levels = {"INFO", "WARN", "ERROR"};
            String[] messages = {
                "Application started successfully",
                "Processing user request",
                "Database connection established",
                "High memory usage detected",
                "Connection timeout occurred",
                "Request processing completed"
            };
            
            String level = levels[ThreadLocalRandom.current().nextInt(levels.length)];
            String message = messages[ThreadLocalRandom.current().nextInt(messages.length)];
            
            sendLog(level, message);
        } catch (Exception e) {
            System.err.println("Failed to send logs: " + e.getMessage());
        }
    }
    
    @Scheduled(fixedRate = 30000) // Every 30 seconds
    public void registerAgent() {
        try {
            Map<String, Object> agent = new HashMap<>();
            agent.put("name", agentId);
            agent.put("last_seen", Instant.now().toString());
            
            Map<String, Boolean> capabilities = new HashMap<>();
            capabilities.put("logs", true);
            capabilities.put("metrics", true);
            capabilities.put("traces", true);
            capabilities.put("processes", true);
            agent.put("capabilities", capabilities);
            
            sendToKafka("agents", agent);
        } catch (Exception e) {
            System.err.println("Failed to register agent: " + e.getMessage());
        }
    }
    
    private void sendMetric(String type, double value) {
        Map<String, Object> metric = new HashMap<>();
        metric.put("timestamp", Instant.now().toString());
        metric.put("agent_id", agentId);
        metric.put("metric_type", type);
        metric.put("value", value);
        metric.put("unit", getUnit(type));
        
        Map<String, String> labels = new HashMap<>();
        labels.put("host", System.getProperty("os.name"));
        labels.put("version", "1.0");
        metric.put("labels", labels);
        
        sendToKafka("metrics-" + agentId, metric);
    }
    
    private void sendLog(String level, String message) {
        Map<String, Object> log = new HashMap<>();
        log.put("timestamp", Instant.now().getEpochSecond());
        log.put("level", level);
        log.put("message", message);
        log.put("service", agentId);
        log.put("agent_id", agentId);
        log.put("source", "application");
        
        sendToKafka("logs-" + agentId, log);
    }
    
    private void sendToKafka(String topic, Object data) {
        try {
            // Simple HTTP POST to Kafka REST proxy or Mon-X backend
            String url = "http://host.docker.internal:8080/api/ingest/" + topic;
            
            HttpHeaders headers = new HttpHeaders();
            headers.setContentType(MediaType.APPLICATION_JSON);
            
            HttpEntity<Object> request = new HttpEntity<>(data, headers);
            restTemplate.postForEntity(url, request, String.class);
            
        } catch (Exception e) {
            System.err.println("Failed to send to Kafka: " + e.getMessage());
        }
    }
    
    private double getCpuUsage() {
        // Simulate CPU usage with some realistic variation
        double base = osBean.getSystemLoadAverage() * 20;
        return Math.max(0, Math.min(100, base + ThreadLocalRandom.current().nextGaussian() * 10));
    }
    
    private double getMemoryUsage() {
        long used = memoryBean.getHeapMemoryUsage().getUsed();
        long max = memoryBean.getHeapMemoryUsage().getMax();
        return (double) used / max * 100;
    }
    
    private String getUnit(String type) {
        switch (type) {
            case "cpu": return "percent";
            case "memory": return "percent";
            case "load": return "average";
            default: return "count";
        }
    }
    
    @Override
    public Health health() {
        try {
            double cpu = getCpuUsage();
            double memory = getMemoryUsage();
            
            if (cpu > 90 || memory > 90) {
                return Health.down()
                    .withDetail("cpu", cpu + "%")
                    .withDetail("memory", memory + "%")
                    .withDetail("status", "degraded")
                    .build();
            }
            
            return Health.up()
                .withDetail("cpu", cpu + "%")
                .withDetail("memory", memory + "%")
                .withDetail("agent_id", agentId)
                .build();
                
        } catch (Exception e) {
            return Health.down()
                .withDetail("error", e.getMessage())
                .build();
        }
    }
}
