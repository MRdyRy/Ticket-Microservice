package com.rudy.ryanto.tiket.repository.tiket;

import com.rudy.ryanto.tiket.domain.tiket.MasterTiket;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface MasterTiketRepository extends JpaRepository<MasterTiket, Long> {
}
