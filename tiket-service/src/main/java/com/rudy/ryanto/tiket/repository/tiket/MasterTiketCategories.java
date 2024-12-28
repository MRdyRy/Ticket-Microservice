package com.rudy.ryanto.tiket.repository.tiket;

import com.rudy.ryanto.tiket.domain.tiket.TiketCategories;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface MasterTiketCategories extends JpaRepository<TiketCategories,Long> {
}
