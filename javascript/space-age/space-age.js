const ORBITAL_PERIOD_EARTH_SECONDS = 31557600;

// In earth years
const orbitalPeriods = {
  mercury: 0.2408467,
  venus: 0.61519726,
  earth: 1.0,
  mars: 1.8808158,
  jupiter: 11.862615,
  saturn: 29.447498,
  uranus: 84.016846,
  neptune: 164.79132,
};

export const age = (planet, ageInSeconds) => {
  let earthAge = calcEarthAgeYears(ageInSeconds);

  return twoDecimalPoints( calcPlanetAgeYears(planet, earthAge) );
};


function calcEarthAgeYears(ageSeconds) {
  let earthAgeYears = ageSeconds / ORBITAL_PERIOD_EARTH_SECONDS;
  
  return earthAgeYears;
}

function calcPlanetAgeYears(planet, ageEarthYears) {
  let planetAge = ageEarthYears / orbitalPeriods[planet];

  return planetAge;
}

function twoDecimalPoints(float) {
  return Number.parseFloat(float.toFixed(2));
}
