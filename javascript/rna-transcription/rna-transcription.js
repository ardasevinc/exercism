//
// This is only a SKELETON file for the 'RNA Transcription' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const toRna = (dnaSeqString) => {
  let rnaSeq = dnaSeqString
    .split("")
    .map((nucleotide) => {
      if (nucleotide === 'G') return 'C';
      else if (nucleotide === 'C') return 'G';
      else if (nucleotide === 'T') return 'A';
      else return 'U';
    })
    .join("");

  return rnaSeq;
};
