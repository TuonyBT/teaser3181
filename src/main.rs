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

    pss.iter().for_each(|p| println!("{:?}", p));

    for p in pss.keys() {
        if true {
//  in fact the only pair that works happen to be odd and coprime so we'll be lazy and omit these tests
//            println!("p: {}, b: {}, a: {}", p, b, a);

//  Next we need to find r, theta pairs such that r and theta each have alternative permutations of the same two digits
//  p = pi * r * (1 + theta_/180) where theta is theta_ to the nearest degree

            let mut r = 10_u32;
            let mut theta_ = 180.0;

            while r < 100 && theta_ > 0.0 {
                theta_ = (*p as f32 / r as f32 - PI) * 180.0 / PI;

                let theta = theta_.round() as u32;
                if r / 10 == theta % 10 && r % 10 == theta / 10 {
                    println!("r: {}, theta_: {}", r, theta);

//  p must be capable of expression as b.(a + b) where b > a and a, b are odd and coprime
//  so let's factorise p first, and then look for factor pairs that meet these criteria

                    for b in all_factors(*p as usize) {
                        if b.pow(2) < *p && b.pow(2) * 2 > *p {
                            let a = p / b - b;
                            println!("New p: {}, b: {}, a: {}", p, b, a);    
                        }
                    

                    }

                }
                r += 1;
            }

        }
    }

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