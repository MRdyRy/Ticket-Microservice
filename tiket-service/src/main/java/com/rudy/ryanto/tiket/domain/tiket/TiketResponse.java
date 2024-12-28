package com.rudy.ryanto.tiket.domain.tiket;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.util.List;

@Builder
@Data
@AllArgsConstructor
@NoArgsConstructor
public class TiketResponse {
    private MasterTiket masterTiket;
    private List<TiketDetails> tiketDetails;
    private List<TiketCategories> tiketCategories;
}
