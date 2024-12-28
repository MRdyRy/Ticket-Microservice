package com.rudy.ryanto.tiket.config;

import io.github.resilience4j.circuitbreaker.CircuitBreaker;
import io.github.resilience4j.circuitbreaker.CircuitBreakerConfig;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import java.time.Duration;

@Configuration
public class ReselienceConfig {

    @Bean
    public CircuitBreaker setup () {
        CircuitBreakerConfig config = CircuitBreakerConfig.custom()
                .failureRateThreshold(50)
                .waitDurationInOpenState(Duration.ofMillis(1000))
                .slidingWindow(100,50,
                        CircuitBreakerConfig.SlidingWindowType.COUNT_BASED)
                .build();
        return CircuitBreaker.of("tiket-service", config);

    }
}
