package com.rudy.ryanto.tiket.domain.tiket;

import com.rudy.ryanto.tiket.domain.AuditTrail;
import jakarta.persistence.*;
import lombok.*;

import java.math.BigDecimal;
import java.util.Date;

@EqualsAndHashCode(callSuper = true)
@Data
@AllArgsConstructor
@NoArgsConstructor
@Entity
@Builder
@Table( name = "TIKET_DETAILS")
public class TiketDetails extends AuditTrail {
    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    @Column(name = "ID", nullable = false)
    private Long id;
    private Long idMaster;
    private BigDecimal price;
    private boolean isPromo;
    private int quantity;
    private Date validDate;
    private Date endDate;
}
