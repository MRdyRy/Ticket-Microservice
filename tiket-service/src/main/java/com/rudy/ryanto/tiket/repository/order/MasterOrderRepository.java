package com.rudy.ryanto.tiket.repository.order;

import com.rudy.ryanto.tiket.domain.order.MasterOrder;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface MasterOrderRepository extends JpaRepository<MasterOrder, Long> {
}
