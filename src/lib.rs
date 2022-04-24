pub mod bigint;
use std::time::{Duration, Instant};

#[cfg(test)]
mod tests {
    /*
    #[test]
    fn i128_benchmark() {
        use std::time::{Duration, Instant};
        use rand::Rng;
        let loop_num = 10000000;
        let mut rng = rand::thread_rng();
        let mut v1:Vec<i128> = Vec::with_capacity(loop_num);
        let mut v2:Vec<i64> = Vec::with_capacity(loop_num);
        v1.resize_with(loop_num, || {let r = rng.gen(); if r == 0 {1}else{r}});
        v2.resize_with(loop_num, || {let r = rng.gen(); if r == 0 {1}else{r}});
        let mut _mul: i128 = 0;
        let start = Instant::now();
        for j in 0..loop_num{
            _mul = v1[j] % v2[j] as i128;
        }
        let end = start.elapsed();
        println!("{}", end.as_secs_f32());
    }
    */
}

use bigint::BigUint;
#[test]
fn it_work()
{
    let a = BigUint::new_rand(100);
    let b = BigUint::new_rand(100);
    let c = a.clone() * b.clone();
    println!("{:?}", c.get_vec());
    println!("{} * {} = {}", a, b, c);
}