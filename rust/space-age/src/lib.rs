#[derive(Debug)]
pub struct Duration {
    second: f64
}

impl From<u64> for Duration {
    fn from(second: u64) -> Self {
        Duration { second: second as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

const EARTH_SECOND: f64 = 31_557_600.0;

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury { fn years_during(d: &Duration) -> f64 { d.second / (0.2408467  * EARTH_SECOND) } }
impl Planet for Venus   { fn years_during(d: &Duration) -> f64 { d.second / (0.61519726 * EARTH_SECOND) } }
impl Planet for Earth   { fn years_during(d: &Duration) -> f64 { d.second / (1.0        * EARTH_SECOND) } }
impl Planet for Mars    { fn years_during(d: &Duration) -> f64 { d.second / (1.8808158  * EARTH_SECOND) } }
impl Planet for Jupiter { fn years_during(d: &Duration) -> f64 { d.second / (11.862615  * EARTH_SECOND) } }
impl Planet for Saturn  { fn years_during(d: &Duration) -> f64 { d.second / (29.447498  * EARTH_SECOND) } }
impl Planet for Uranus  { fn years_during(d: &Duration) -> f64 { d.second / (84.016846  * EARTH_SECOND) } }
impl Planet for Neptune { fn years_during(d: &Duration) -> f64 { d.second / (164.79132  * EARTH_SECOND) } }
