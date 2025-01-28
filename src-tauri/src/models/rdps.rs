#[derive(Debug, Clone)]
pub struct RdpsData {
    pub multi_dmg: RdpsRates,
    pub atk_pow_sub_rate_2: RdpsSelfRates,
    pub atk_pow_sub_rate_1: RdpsRates,
    pub skill_dmg_rate: RdpsSelfRates,
    pub atk_pow_amplify: Vec<RdpsBuffData>,
    pub crit: RdpsSelfRates,
    pub crit_dmg_rate: f64,
}

impl Default for RdpsData {
    fn default() -> Self {
        Self {
            multi_dmg: RdpsRates::default(),
            atk_pow_sub_rate_2: RdpsSelfRates::default(),
            atk_pow_sub_rate_1: RdpsRates::default(),
            skill_dmg_rate: RdpsSelfRates::default(),
            atk_pow_amplify: Vec::new(),
            crit: RdpsSelfRates::default(),
            crit_dmg_rate: 2.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RdpsRates {
    pub sum_rate: f64,
    pub total_rate: f64,
    pub values: Vec<RdpsBuffData>,
}

impl Default for RdpsRates {
    fn default() -> Self {
        Self {
            sum_rate: 0.0,
            total_rate: 1.0,
            values: Vec::new(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct RdpsSelfRates {
    pub self_sum_rate: f64,
    pub sum_rate: f64,
    pub values: Vec<RdpsBuffData>,
}

#[derive(Debug, Default, Clone)]
pub struct RdpsBuffData {
    pub caster: String,
    pub rate: f64,
}
