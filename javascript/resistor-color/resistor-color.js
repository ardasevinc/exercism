//
// This is only a SKELETON file for the 'Resistor Color' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const colorCode = (colorStr) => {
  for (let i = 0; i < COLORS.length; i++) {
    if (COLORS[i] == colorStr) {
      return i;
    }
  }
};

export const colorCodeVer2 = (colorStr) => {
  return (COLORS.indexOf(colorStr));
};

export const COLORS = ['black', 'brown', 'red', 'orange', 'yellow',
'green', 'blue', 'violet', 'grey', 'white'];
