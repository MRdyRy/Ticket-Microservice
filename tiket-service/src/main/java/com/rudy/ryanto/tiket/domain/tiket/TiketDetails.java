package com.rudy.ryanto.tiket.domain.tiket;

import com.rudy.ryanto.tiket.domain.AuditTrail;
import jakarta.persistence.Column;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import lombok.*;

import java.util.Date;

@EqualsAndHashCode(callSuper = true)
@Data
@AllArgsConstructor
@NoArgsConstructor
@Builder
public class TiketDetails extends AuditTrail {
    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    @Column(name = "ID", nullable = false)
    private Long id;
    private Long idMaster;
    private double price;
    private boolean isPromo;
    private int quantity;
    private Date validDate;
    private Date endDate;
}
