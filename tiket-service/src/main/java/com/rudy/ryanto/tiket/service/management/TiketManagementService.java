package com.rudy.ryanto.tiket.service.management;

import com.rudy.ryanto.tiket.domain.BaseResponse;
import com.rudy.ryanto.tiket.domain.CommonConstants;
import com.rudy.ryanto.tiket.domain.tiket.TiketRequest;
import com.rudy.ryanto.tiket.domain.tiket.TiketResponse;
import com.rudy.ryanto.tiket.exception.TiketException;
import com.rudy.ryanto.tiket.repository.tiket.MasterTiketCategories;
import com.rudy.ryanto.tiket.repository.tiket.MasterTiketRepository;
import com.rudy.ryanto.tiket.repository.tiket.TiketDetailsRepository;
import io.github.resilience4j.ratelimiter.annotation.RateLimiter;
import lombok.extern.slf4j.Slf4j;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.lang.NonNull;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

@Slf4j
@Service
public class TiketManagementService {

    private final MasterTiketRepository masterTiketRepository;
    private final MasterTiketCategories masterTiketCategories;
    private final TiketDetailsRepository tiketDetailsRepository;
    private final RedisTemplate<String, Object> redisTemplate;

    public TiketManagementService(MasterTiketRepository masterTiketRepository, MasterTiketCategories masterTiketCategories, TiketDetailsRepository tiketDetailsRepository, RedisTemplate redisTemplate) {
        this.masterTiketRepository = masterTiketRepository;
        this.masterTiketCategories = masterTiketCategories;
        this.tiketDetailsRepository = tiketDetailsRepository;
        this.redisTemplate = redisTemplate;
    }

    @Transactional(rollbackFor = Exception.class)
    @RateLimiter(name = "ticketService", fallbackMethod = "fallbackGeneral")
    public Mono<BaseResponse<TiketResponse>> createTiket(@NonNull TiketRequest request) {
        log.debug("in create tiket management service !");
        BaseResponse<TiketResponse> response = new BaseResponse<>();
        return masterTiketRepository.save(request.getMasterTiket()).flatMap(masterTiket -> {
            response.getData().setMasterTiket(masterTiket);
            return Flux.fromIterable(request.getTiketDetails()).flatMap(tiketDetail -> {
                tiketDetail.setIdMaster(masterTiket.getId());
                return tiketDetailsRepository.save(tiketDetail).doOnNext(savedDetail -> response.getData().getTiketDetails().add(savedDetail));
            }).then(Mono.just(response));
        }).onErrorResume(e -> {
            log.error("error create tikets, caused : ", e);
            return Mono.error(new TiketException());
        });
    }


    @Transactional(rollbackFor = Exception.class)
    @RateLimiter(name = "ticketService", fallbackMethod = "fallbackGeneral")
    public Mono<BaseResponse<TiketResponse>> createCategories(TiketRequest request) {
        log.debug("in create categories services {}", request.getTiketCategories().size());
        BaseResponse<TiketResponse> response = new BaseResponse<>();
        return masterTiketCategories.saveAll(request.getTiketCategories()).collectList().flatMap(categories -> {
            response.setData(TiketResponse.builder().tiketCategories(categories).build());
            return Mono.just(response);
        }).onErrorResume(e -> {
            log.error("error create categories, caused :", e);
            return Mono.error(new TiketException());
        });
    }

    @Transactional(readOnly = true)
    public Mono<BaseResponse<Object>> reloadAllCache() {
        log.debug("in reload all cache services");
        return masterTiketCategories.findAll().collectList().flatMap(categories -> {
            redisTemplate.opsForValue().set("categories", categories);
            return Mono.just(BaseResponse.builder().statusCode(CommonConstants.STATUS_CODE_SUCCESS).data("success reload cache").build());
        });
    }


    public Mono<TiketResponse> fallbackGeneral(Throwable ex) {
        return Mono.error(new RuntimeException("Rate limit exceeded, try again later"));
    }


}
