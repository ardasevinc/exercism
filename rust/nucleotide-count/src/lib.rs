use std::collections::HashMap;
const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    // `?` operator propagates (returns to caller) the error
    is_invalid_dna(dna)?;
    is_invalid_nuc(nucleotide)?;

    let nuc_count = dna.matches(nucleotide).count();
    Ok(nuc_count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    is_invalid_dna(dna)?;

    let mut nuc_counts = HashMap::new();

    for nuc in NUCLEOTIDES.iter() {
        nuc_counts.insert(*nuc, dna.matches(*nuc).count());
    }
    
    Ok(nuc_counts)
}

fn is_invalid_dna(dna: &str) -> Result<(), char> {
    let invalid_test = dna.chars().find(|c| !NUCLEOTIDES.contains(c));

    match invalid_test {
        Some(x) => Err(x),
        None    => Ok(())
    }
}

fn is_invalid_nuc(nuc: char) -> Result<(), char> {
    match !NUCLEOTIDES.contains(&nuc) {
        true  => Err(nuc),
        false => Ok(())
    }
}
