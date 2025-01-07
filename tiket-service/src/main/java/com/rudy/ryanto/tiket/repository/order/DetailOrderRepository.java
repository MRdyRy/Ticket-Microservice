package com.rudy.ryanto.tiket.repository.order;

import com.rudy.ryanto.tiket.domain.order.DetailOrder;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface DetailOrderRepository extends JpaRepository<DetailOrder, Long> {
}
