package com.rudy.ryanto.tiket.domain.tiket;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.util.List;

@Data
@AllArgsConstructor
@NoArgsConstructor
@Builder
public class TiketRequest {
    private MasterTiket masterTiket;
    private List<TiketCategories> tiketCategories;
    private List<TiketDetails> tiketDetails;
}
