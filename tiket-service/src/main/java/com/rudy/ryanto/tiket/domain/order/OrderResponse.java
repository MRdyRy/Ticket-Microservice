package com.rudy.ryanto.tiket.domain.order;

import com.rudy.ryanto.tiket.domain.tiket.MasterTiket;
import com.rudy.ryanto.tiket.domain.tiket.TiketDetails;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.math.BigDecimal;

@Data
@AllArgsConstructor
@NoArgsConstructor
@Builder
public class OrderResponse {
    private Long orderId;
    private MasterTiket masterTiket;
    private TiketDetails tiketDetails;
    private int qtyOrder;
    private String statusPayment;
    private BigDecimal totalPrice;
}
