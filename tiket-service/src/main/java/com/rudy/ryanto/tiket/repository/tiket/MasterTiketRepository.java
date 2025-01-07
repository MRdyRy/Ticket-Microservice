package com.rudy.ryanto.tiket.repository.tiket;

import com.rudy.ryanto.tiket.domain.tiket.MasterTiket;
import org.springframework.data.repository.reactive.ReactiveCrudRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface MasterTiketRepository extends ReactiveCrudRepository<MasterTiket, Long> {
}
