use std::{collections::{BTreeSet, BTreeMap}, f32::consts::PI};
use itertools::Itertools;


fn main() {

//  Set up a dictionary of Pythagorean triplets indexed according to perimeter length p < 100
//  Limits set by hypotenuse h < a + b => h < p/2
    let mut pss = BTreeMap::<u32, Vec<[u32; 3]>>::new();

    for h in 1..50 {
        let ps = pythag_triplets(h).into_iter().filter(|[x, y, z]| x + y + z < 100).collect::<BTreeSet<[u32; 3]>>();
        if !ps.is_empty() {
            for [x, y, z] in ps.to_owned() {
                let p = x + y + z;
                pss.entry(p).or_insert(Vec::<[u32; 3]>::new()).push([x, y, z]);
            }
        }
    }

//  We are looking for values of p for which more than one triplet exists
    pss = pss.into_iter().filter(|(_k, v)| v.len() > 1).collect::<BTreeMap<u32, Vec<[u32; 3]>>>();

    for p in pss.keys() {
        if p_expression(*p) && globe_check(*p) {
            println!("Solution p: {}", p);    
        }
    }

}

//  Check that perimeter can be expressed in terms of a and b meeting puzzle criteria

fn p_expression(p: u32) -> bool {

//  p must be capable of expression as b.(a + b) where b > a and a, b are odd and share no prime factors
//  so let's factorise p first, and then look for factor pairs that meet these criteria
    let mut pass = false;

    for b in all_factors(p as usize) {
        if b % 2 == 0 {continue}
        if b.pow(2) < p && b.pow(2) * 2 > p {
            let a = p / b - b;
            if a % 2 != 0 && hcf(a, b) == 1 {
                pass = true;    
            }
        }
    }
    pass
}

//  Check that the perimeter can be stretched over the globe as described
//  We need to find r, theta pairs such that r and theta each have alternative permutations of the same two digits
//  p = pi * r * (1 + theta_/180) where theta is theta_ to the nearest degree

fn globe_check(p: u32) -> bool {

    let mut pass = false;
    //  Initialise r and theta such that they have two digits; r will increase and theta decrease in our loop
    let mut r = 10_u32;
    let mut theta_ = 99.0;

    while r < 100 && theta_ >= 10.0 {
        theta_ = (p as f32 / r as f32  / PI - 1.0) * 180.0;

        let theta = theta_.round() as u32;
        if r / 10 == theta % 10 && r % 10 == theta / 10 {
            pass = true;
            break
        }
        r += 1;

    }
    pass
}

//  Return all Pythagorean triplets with hypotenuse n
pub fn pythag_triplets(c: u32) -> BTreeSet<[u32; 3]> {

    let mut triplets = BTreeSet::<[u32; 3]>::new();
    let mut b = c - 1;
    let mut a = 1u32;

    while a < b {
        let a_sq = c.pow(2) - b.pow(2);
        let a_ = (a_sq as f64).sqrt();
        a = a_.floor() as u32;
        if a.pow(2) + b.pow(2) == c.pow(2) {
            triplets.insert([a, b, c]);
        }
        b -= 1;
    }
    triplets
} 

//  Prime factor finder ported from Jim Randell's Enigma Python library
//  Wheel factorisation using wheels of circumference 30
pub fn prime_factor(m: usize) -> Vec<[usize; 2]> {
    let mut factors: Vec<[usize; 2]> = Vec::new();
    if m > 1 {
        let mut n = m;
        let mut i = 2;
        let ds = [1, 2, 2, 4, 2, 4, 2, 4, 6, 2, 6];
        let js = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 3];
        let mut j = 0;
        while i*i <= n {
            let mut e = 0;
            loop {
                let (d, r) = (&n/&i, &n%&i);

                if r > 0 {
                    break;
                }
            e += 1;
            n = d;
            }
            if e > 0 {
                factors.push([i, e]);
            }
            i += ds[j];
            j = js[j];
        }
        if n > 1 {
            factors.push([n, 1]);
        }
    }
    factors
}


fn all_factors(m: usize) -> Vec<u32> {

    let mut all_factors = Vec::<u32>::new();

    let facs = prime_factor(m);
    let pfs =facs.iter()
    .map(|[f, _p]| f).collect::<Vec<&usize>>();

    for pwrs in facs.iter()
    .map(|[_f, p]| 0..*p + 1).multi_cartesian_product() {  

        all_factors.push(pwrs.into_iter().zip(&pfs).map(|(p, f)| f.pow(p as u32) as u32)
        .product::<u32>());

    }
    all_factors.sort();
    all_factors
}

//  Highest common factor of two numbers

fn hcf(x: u32, y: u32) -> u32 {

    // Prime factors of the two subject numbers
    let x_facs = prime_factor(x as usize);
    let y_facs = prime_factor(y as usize);

    // Common prime factors
    let cfs = x_facs.iter().cartesian_product(&y_facs)
                                                    .filter(|(x, y)| x[0] == y[0])
                                                    .map(|(x, y)|[x[0], x[1].min(y[1])])
                                                    .collect::<Vec<[usize; 2]>>();

    //  Product of all common factors raised to respective smaller power
    cfs.iter().map(|z| (z[0] as u32).pow(z[1] as u32)).product::<u32>()

}