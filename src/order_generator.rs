use rand::Rng;
use crate::Order;
use std::time::SystemTime;

#[derive(Clone, Copy)]
pub struct Probability(u64);

impl Probability {
    pub fn new(val: u64) -> Option<Self> {
        if val > 1 {
            None
        } else {
            Some(Probability(val))
        }
    }

    pub fn get(self) -> u64 {// you don't want to get it since it will give you ownership instead of order generator --> make it so can't
        self.0
    }
}

pub struct OrderGenerator {
    rng: Probability,
    vol: Probability,
}

impl OrderGenerator {
    pub fn build(&self, rng: u64, vol:u64) -> Option<Self> {
        let prob_rng = Probability::new(rng)?;
        let prob_vol = Probability::new(vol)?;
        Some(OrderGenerator { rng: prob_rng, vol: prob_vol })
    }

    pub fn gen_order(&self, mut center: u64, id: u128) -> Order {
        let mut rand_gen = rand::rng();

        let buy_sell = rand_gen.random_bool(0.5);

        if rand_gen.random_bool(self.rng.clone().get() as f64) {
            center = self.vol.clone().get() * center + center;
        } else {
            center = self.vol.clone().get() * center - center;
        }

        Order { buy_order: buy_sell, price: center, id, time_created: SystemTime::now() }
    }

    pub fn start() {}

    pub fn stop() {}

}
