package com.rudy.ryanto.tiket.domain.order;

import com.rudy.ryanto.tiket.domain.AuditTrail;
import jakarta.persistence.*;
import lombok.*;

import java.math.BigDecimal;

@EqualsAndHashCode(callSuper = true)
@Data
@Builder
@Entity
@AllArgsConstructor
@NoArgsConstructor
@Table(name = "DETAIL_ORDER")
public class DetailOrder extends AuditTrail {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;
    private int quantity;
    private Long idOrder;
    private Long idMasterTiket;
    private BigDecimal price;


}
