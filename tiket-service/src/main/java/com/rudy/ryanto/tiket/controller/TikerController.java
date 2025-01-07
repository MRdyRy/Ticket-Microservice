package com.rudy.ryanto.tiket.controller;

import com.rudy.ryanto.tiket.domain.BaseResponse;
import com.rudy.ryanto.tiket.domain.tiket.TiketRequest;
import com.rudy.ryanto.tiket.domain.tiket.TiketResponse;
import com.rudy.ryanto.tiket.service.management.TiketManagementService;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.*;
import reactor.core.publisher.Mono;

@Slf4j
@RestController
@RequestMapping("/api/tiket")
public class TikerController {

    private final TiketManagementService tiketManagementService;

    public TikerController(TiketManagementService tiketManagementService) {
        this.tiketManagementService = tiketManagementService;
    }

    @PostMapping("/create")
    public Mono<BaseResponse<TiketResponse>> createTiket(@RequestBody TiketRequest request) {
        log.info("create tiket request : {}", request);
        return tiketManagementService.createTiket(request);
    }

    @PostMapping("/create-categories")
    public Mono<BaseResponse<TiketResponse>> createCategories(@RequestBody TiketRequest request) {
        log.info("create categories request : {}", request);
        return tiketManagementService.createCategories(request);
    }

    @GetMapping("/reload-cache")
    public Mono<BaseResponse<Object>> reloadAllCache() {
        log.info("reload all cache request");
        return tiketManagementService.reloadAllCache();
    }
}
