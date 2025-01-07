package com.rudy.ryanto.tiket.domain;

import jakarta.persistence.Column;

import java.util.Date;

public class AuditTrail {
    @Column(name = "CREATE_BY", nullable = false)
    private String createBy;
    private String updateBy;
    @Column(name = "CREATE_DATE", nullable = false)
    private Date createDate;
    private Date updateDate;
}
