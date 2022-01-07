const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;
// const MERCURY_ORBITAL_PERIOD: f64 = 0.2408467;
// const VENUS_ORBITAL_PERIOD: f64 = 0.61519726;
// const MARS_ORBITAL_PERIOD: f64 = 1.8808158;
// const JUPITER_ORBITAL_PERIOD: f64 = 11.862615;
// const SATURN_ORBITAL_PERIOD: f64 = 29.447498;
// const URANUS_ORBITAL_PERIOD: f64 = 84.016846;
// const NEPTUNE_ORBITAL_PERIOD: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        let earth_years: f64 = s as f64 / EARTH_YEAR_IN_SECONDS;

        Duration {
            earth_years,
        }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.earth_years / Self::ORBITAL_PERIOD
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;


impl Planet for Mercury {
    // fn years_during(d: &Duration) -> f64 {
    //     (d.earth_years) / MERCURY_ORBITAL_PERIOD
    // }
    const ORBITAL_PERIOD: f64 = 0.2408467;
}
impl Planet for Venus {
    // fn years_during(d: &Duration) -> f64 {
    //     (d.earth_years) / VENUS_ORBITAL_PERIOD
    // }
    const ORBITAL_PERIOD: f64 = 0.61519726;
}
impl Planet for Earth {
    // fn years_during(d: &Duration) -> f64 {
    //     d.earth_years
    // }
    const ORBITAL_PERIOD: f64 = 1.0;
}
impl Planet for Mars {
    // fn years_during(d: &Duration) -> f64 {
    //     (d.earth_years) / MARS_ORBITAL_PERIOD
    // }
    const ORBITAL_PERIOD: f64 = 1.8808158;
}
impl Planet for Jupiter {
    // fn years_during(d: &Duration) -> f64 {
    //     (d.earth_years) / JUPITER_ORBITAL_PERIOD
    // }
    const ORBITAL_PERIOD: f64 = 11.862615;
}
impl Planet for Saturn {
    // fn years_during(d: &Duration) -> f64 {
    //     (d.earth_years) / SATURN_ORBITAL_PERIOD
    // }
    const ORBITAL_PERIOD: f64 = 29.447498;
}
impl Planet for Uranus {
    // fn years_during(d: &Duration) -> f64 {
    //     (d.earth_years) / URANUS_ORBITAL_PERIOD
    // }
    const ORBITAL_PERIOD: f64 = 84.016846;
}
impl Planet for Neptune {
    // fn years_during(d: &Duration) -> f64 {
    //     (d.earth_years) / NEPTUNE_ORBITAL_PERIOD
    // }
    const ORBITAL_PERIOD: f64 = 164.79132;
}
