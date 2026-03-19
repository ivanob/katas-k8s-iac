package com.kata;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.*;

@SpringBootApplication
public class ServiceBApplication {
    public static void main(String[] args) {
        SpringApplication.run(ServiceBApplication.class, args);
    }
}

@RestController
@RequestMapping("/api")
class RequestCounterController {
    private long requestCount = 0;

    @PostMapping("/data")
    public Response receiveData(@RequestBody Request request) {
        requestCount++;
        System.out.println("Received: " + request.message);
        return new Response(true, request.message, requestCount);
    }

    @GetMapping("/health")
    public Health health() {
        return new Health("ok");
    }

    record Request(String message) {}
    record Response(boolean received, String message, long totalRequests) {}
    record Health(String status) {}
}
