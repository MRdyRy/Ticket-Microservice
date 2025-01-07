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
@Table(name = "MASTER_ORDER")
public class MasterOrder extends AuditTrail {

    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    private Long id;

    @Column(name = "TOTAL_PRICE",precision = 2, nullable = false)
    private BigDecimal total_price;

    @Column(name = "STATUS", nullable = false)
    private String status;


}
