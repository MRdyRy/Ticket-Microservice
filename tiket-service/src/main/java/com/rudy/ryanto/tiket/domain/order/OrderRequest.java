package com.rudy.ryanto.tiket.domain.order;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@AllArgsConstructor
@NoArgsConstructor
@Builder
public class OrderRequest {
    private Long masterTiketId;
    private int quantity;
    private String statusPayment;
}
