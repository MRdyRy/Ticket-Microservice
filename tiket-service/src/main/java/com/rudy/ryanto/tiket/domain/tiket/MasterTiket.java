package com.rudy.ryanto.tiket.domain.tiket;

import com.rudy.ryanto.tiket.domain.AuditTrail;
import jakarta.persistence.*;
import lombok.*;

@EqualsAndHashCode(callSuper = true)
@Data
@Builder
@AllArgsConstructor
@NoArgsConstructor
@Table(name = "MASTER_TIKET")
public class MasterTiket extends AuditTrail {
    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    @Column(name = "ID", nullable = false)
    private Long id;
    @Column(name = "TIKET_NAME", nullable = false)
    private String tiketName;
    private String decription;
    @Column(name = "STATUS", nullable = false)
    private boolean Status;
    @Column(name = "CATEGORIES_ID", nullable = false)
    private Long categoriesId;
}
