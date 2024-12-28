package com.rudy.ryanto.tiket.domain.order;

import com.rudy.ryanto.tiket.domain.AuditTrail;
import jakarta.persistence.*;
import lombok.*;

@EqualsAndHashCode(callSuper = true)
@Data
@Builder
@AllArgsConstructor
@NoArgsConstructor
@Table(name = "MASTER_ORDER")
public class MasterOrder extends AuditTrail {

    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    private Long id;

    @Column(name = "TOTAL_PRICE",precision = 2, nullable = false)
    private double total_price;

    @Column(name = "STATUS", nullable = false)
    private int status;
}
