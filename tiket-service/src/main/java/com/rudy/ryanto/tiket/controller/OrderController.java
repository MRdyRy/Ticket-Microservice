package com.rudy.ryanto.tiket.controller;
import com.rudy.ryanto.tiket.domain.BaseResponse;
import com.rudy.ryanto.tiket.domain.order.OrderRequest;
import com.rudy.ryanto.tiket.domain.order.OrderResponse;
import com.rudy.ryanto.tiket.service.order.TiketOrderService;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.*;
import reactor.core.publisher.Mono;

@Slf4j
@RestController
@RequestMapping("/api/order")
public class OrderController {

    private final TiketOrderService tiketOrderService;

    public OrderController(TiketOrderService tiketOrderService) {
        this.tiketOrderService = tiketOrderService;
    }

    @PostMapping("/create-order")
    public Mono<BaseResponse<OrderResponse>> createOrder(@RequestBody OrderRequest request) {
        log.info("create order request : {}", request);
        return tiketOrderService.orderTiket(request);
    }
}