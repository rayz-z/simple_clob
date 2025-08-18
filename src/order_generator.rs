use rand::Rng;

#[derive(Clone, Copy)]
pub struct Probability(f64);

impl Probability {
    pub fn new(val: f64) -> Option<Self> {
        // refactor so all decimals are actually decimals
        if val > 1.0 || val < 0.0 {
            println!("Probability value out of range: {}", val);
            None
        } else {
            Some(Probability(val))
        }
    }

    pub fn get(self) -> f64 {
        // you don't want to get it since it will give you ownership instead of order generator --> make it so can't
        self.0
    }
}

pub struct OrderGenerator {
    rng: Probability,
    vol: Probability,
}

impl OrderGenerator {
    pub fn build(rng: f64, vol: f64) -> Option<Self> {
        let prob_rng = Probability::new(rng)?;
        let prob_vol = Probability::new(vol)?;
        Some(OrderGenerator {
            rng: prob_rng,
            vol: prob_vol,
        })
    }

    pub fn gen_order(&self, mut center: f64) -> (bool, u64) {
        let mut rand_gen = rand::rng();

        let buy_sell = rand_gen.random_bool(0.5);


        let vol = self.vol.get();
        let rand_noise = rand_gen.random_range(0.0..vol);
        if rand_gen.random_bool(self.rng.get() as f64) {// only generates two outcome prices instead of range
            center = (1.0 + rand_noise) * center;
        } else {
            center = (1.0 - rand_noise) * center;
        }

        (buy_sell, center as u64) // center gets rounded
    }

    pub fn start() {}

    pub fn stop() {}
}
