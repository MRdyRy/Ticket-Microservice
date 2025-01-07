package com.rudy.ryanto.tiket.service.order;

import com.rudy.ryanto.tiket.domain.BaseResponse;
import com.rudy.ryanto.tiket.domain.CommonConstants;
import com.rudy.ryanto.tiket.domain.order.DetailOrder;
import com.rudy.ryanto.tiket.domain.order.MasterOrder;
import com.rudy.ryanto.tiket.domain.order.OrderRequest;
import com.rudy.ryanto.tiket.domain.order.OrderResponse;
import com.rudy.ryanto.tiket.repository.order.DetailOrderRepository;
import com.rudy.ryanto.tiket.repository.order.MasterOrderRepository;
import com.rudy.ryanto.tiket.repository.tiket.MasterTiketCategories;
import com.rudy.ryanto.tiket.repository.tiket.MasterTiketRepository;
import com.rudy.ryanto.tiket.repository.tiket.TiketDetailsRepository;
import io.github.resilience4j.ratelimiter.annotation.RateLimiter;
import lombok.extern.slf4j.Slf4j;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.lang.NonNull;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import reactor.core.publisher.Mono;

import java.math.BigDecimal;
import java.util.Objects;

@Slf4j
@Service
public class TiketOrderService {

    private final MasterTiketRepository masterTiketRepository;
    private final MasterTiketCategories masterTiketCategories;
    private final TiketDetailsRepository tiketDetailsRepository;
    private final RedisTemplate<String, Object> redisTemplate;
    private final MasterOrderRepository masterOrderRepository;
    private final DetailOrderRepository detailOrderRepository;

    public TiketOrderService(MasterTiketRepository masterTiketRepository,
                             MasterTiketCategories masterTiketCategories,
                             TiketDetailsRepository tiketDetailsRepository,
                             RedisTemplate redisTemplate,
                             MasterOrderRepository masterOrderRepository,
                             DetailOrderRepository detailOrderRepository) {
        this.masterTiketRepository = masterTiketRepository;
        this.masterTiketCategories = masterTiketCategories;
        this.tiketDetailsRepository = tiketDetailsRepository;
        this.redisTemplate = redisTemplate;
        this.masterOrderRepository = masterOrderRepository;
        this.detailOrderRepository = detailOrderRepository;
    }

    @Transactional(rollbackFor = Exception.class)
    @RateLimiter(name = "ticketService", fallbackMethod = "fallbackGeneral")
    public Mono<BaseResponse<OrderResponse>> orderTiket(@NonNull OrderRequest request) {
        log.debug("in order tiket service !");
        BaseResponse<OrderResponse> response = new BaseResponse<>();
        try {
            var masterTiket = masterTiketRepository.findById(request.getMasterTiketId()).block();
            if (masterTiket == null) {
                log.error("master tiket not found !");
                return Mono.error(new Exception("master tiket not found"));
            }
            var tiketDetails = tiketDetailsRepository.findById(request.getMasterTiketId()).block();
            if (tiketDetails == null) {
                log.error("tiket details not found !");
                return Mono.error(new Exception("tiket details not found"));
            }
            // save order
            var masterOrder = masterOrderRepository.save(MasterOrder.builder().status(CommonConstants.STATUS_PAY_PENDING).total_price(tiketDetails.getPrice().multiply(BigDecimal.valueOf(request.getQuantity()))).build());
            var detailOrder = detailOrderRepository.save(DetailOrder.builder().idOrder(masterOrder.getId()).idMasterTiket(request.getMasterTiketId()).price(tiketDetails.getPrice()).quantity(request.getQuantity()).build());
            response.setData(OrderResponse.builder().orderId(masterOrder.getId()).qtyOrder(detailOrder.getQuantity()).statusPayment(masterOrder.getStatus()).totalPrice(masterOrder.getTotal_price()).build());
        } catch (Exception e) {
            log.error("error order tikets, caused : ", e);
            return Mono.error(e);
        }
        return Mono.just(response);
    }

    private Object getCategory() {
        var categories = redisTemplate.opsForValue().get("categories");

        if (categories == null) {
            log.debug("categories not found in redis, fetching from database");
            categories = masterTiketCategories.findAll().collectList().block();
            redisTemplate.opsForValue().set("categories", Objects.requireNonNull(categories));
        }
        return categories;
    }
}
