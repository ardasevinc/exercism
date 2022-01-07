//
// This is only a SKELETON file for the 'Two fer' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const twoFer = (name) => {
  // Original
  // if (name) {
  //   return `One for ${ name }, one for me.`;
  // } else {
  //   return "One for you, one for me.";
  // }

  // As a test
  // return (
  //   name ? `One for ${name}, one for me.`
  //   : "One for you, one for me."
  // );
  
  // Inspired from community solutions
  return `One for ${ name || "you" }, one for me.`;
};
