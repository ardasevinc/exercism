// This exercise is skipped because of premature complexity
pub fn nth(n: u32) -> u32 {
    let sieve = gen_sieve(10);

    for x in sieve {
        println!("{}", x);
    }
    5
}


fn gen_sieve(n: u32) -> Vec<bool> {
    let sieve = vec![true; (n-1) as usize];

    for i in (2..n.sqrt()) {
        if 
    }
}
