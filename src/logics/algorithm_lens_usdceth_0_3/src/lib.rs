use algorithm_lens_usdceth_0_3_accessors :: * ;
# [derive (Clone , Debug , Default ,
    candid :: CandidType , serde :: Deserialize , serde :: Serialize)]
pub struct LensValue {
    pub address: String,
    pub current_price: f32,
    pub range_top: f32,
    pub range_bottom: f32,
    pub edpr: f32
}
pub async fn calculate (targets : Vec < String >) -> LensValue {
    let pool_fees_result = get_get_last_snapshot_value_in_pool_fees_usdceth_0_3 (targets . get (0usize) . unwrap () . clone ()) . await ;
    let tc28x6_result = get_get_last_snapshot_value_in_tcumul_28x6hr_usdceth_0_3 (targets . get (1usize) . unwrap () . clone ()) . await ;
    let tc30_result = get_get_last_snapshot_value_in_tcumul_30_min_usdceth_0_3 (targets . get (2usize) . unwrap () . clone ()) . await ;
    let v3pool_result = get_get_last_snapshot_value_in_v3pool_usdceth_0_3 (targets . get (3usize) . unwrap () . clone ()) . await ;
    let pool_usd_prices_result = get_get_last_snapshot_value_in_pool_usd_prices (targets . get (4usize) . unwrap () . clone ()) . await ;
    
    let (fees_24h_usd, _, _) = pool_fees_result ;
    let (_, _, t0_price_usd, t1_price_usd, _)  = pool_usd_prices_result;
    let (
        address, _, _, _, _, _, _, current_tick_liquidity, _, _, _, _, _,
        sqrt_ratio_x96, tick_current, tick_spacing, ticks,
        _, _, _, _, _, _, _, _, _, _
    ) = v3pool_result;
    let (tick_cumul_28x6h, _) = tc28x6_result;
    let (tick_cumul_30m, _) = tc30_result;

    let price_x96 = u128::pow(sqrt_ratio_x96, 2) as f32;
    let mut current_price = price_x96 / f32::powf(2.0,192.0);
    let current_price = current_price / f32::powf(10.0,12.0);

    // let twat30 = (tick_cumul_30m[1] as i32 - tick_cumul_30m[0] as i32) / 1800;
    let compressed: i32 = tick_current / tick_spacing;
    if tick_current <0 && tick_current % tick_spacing != 0 {
        compressed -= 1;
    }
    let floor = compressed * tick_spacing;
    let mut min_tick = floor;
    let mut max_tick = floor + tick_spacing;
    let mut i;
    for i in (1..29).rev() {
        let new_tick = (tick_cumul_28x6h[i] as i32 - tick_cumul_28x6h[i-1] as i32) / 21600;
        if new_tick < min_tick {
            min_tick = new_tick;
        } else if new_tick > max_tick {
            max_tick = new_tick;
        }
    }
    let mut state = current_tick_liquidity as i128;
    let mut sum_liquidity: i128 = 0;
    if tick_current >= min_tick && tick_current < max_tick {
        sum_liquidity = current_tick_liquidity as i128;
    }

    for i in (floor + tick_spacing..max_tick).step_by(tick_spacing as usize) {
        match std::panic::catch_unwind(|| {
            let (_, _, _, _, liquidity_net) = ticks.get(&i.to_string());
            state += liquidity_net;
            sum_liquidity += state;
        }) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
    state = current_tick_liquidity as i128;
    for i in (floor + tick_spacing..min_tick).rev().step_by(tick_spacing as usize) {
        match std::panic::catch_unwind(|| {
            let (_, _, _, _, liquidity_net) = ticks.get(&i.to_string());
            state -= liquidity_net;
            sum_liquidity += state;
        }) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
    let upper_sqrt_price_x96 = f32::powf(1.0001, max_tick);
    let mut range_top = upper_sqrt_price_x96 / f32::powf(2.0,192.0);
    range_top = range_top / f32::powf(10.0,12.0);
    let lower_sqrt_price_x96 = f32::powf(1.0001, max_tick);
    let mut range_bottom = upper_sqrt_price_x96 / f32::powf(2.0,192.0);
    range_bottom = range_bottom / f32::powf(10.0,12.0);
    
    let tvl_in_range = sum_liquidity as f32
        * ((sqrt_ratio_x96 as f32 / upper_sqrt_price_x96)
            * (upper_sqrt_price_x96 - sqrt_ratio_x96 as f32)
            + (sqrt_ratio_x96 as f32 - lower_sqrt_price_x96));

    let fees_24h_eth = fees_24h_usd / t1_price_usd;
    let edr = fees_24h_eth / tvl_in_range;
    let edpr = edr / 100;

    let result = LensValue {
        address,
        current_price,
        range_top,
        range_bottom,
        edpr
    };
    result
}
