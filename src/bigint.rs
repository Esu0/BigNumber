use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct BigInt{
    digits: Vec<i64>,
    sign: bool
}

#[derive(Clone)]
pub struct BigUint{
    digits: Vec<u64>
}

impl BigInt{
    const RADIX_DIGIT: usize = 5;
    const RADIX: i64 = 100000;
    pub fn new(numary: &str) -> Self {
        let s = &numary[0..1] == "-";
        let mut numary = numary;
        if s || &numary[0..1] == "+"{
            numary = &numary[1..];
        }
        let mut i: usize = numary.len();
        let mut v = Vec::with_capacity(numary.len());
        while i > BigInt::RADIX_DIGIT {
            let part = &numary[i - BigInt::RADIX_DIGIT..i];
            v.push(part.parse().unwrap_or(0));
            i -= BigInt::RADIX_DIGIT;
        }
        v.push(numary[..i].parse().unwrap_or(0));
        //上位桁の0をポップ
        while match v.last() {Some(e) => *e == 0, None => false} {
            v.pop();
        }
        BigInt{
            digits: v,
            sign: s
        }
    }

    pub fn get_vec(&self) -> Vec<i64>{
        self.digits.clone()
    }
    pub fn get_sign(&self) -> bool {
        self.sign
    }
}

impl BigUint {
    const RADIX_DIGIT: usize = 5;
    const RADIX: u64 = 100000;

    pub fn new(numary: &str) -> Self {
        let mut numary = numary;
        if &numary[0..1] == "+"{
            numary = &numary[1..];
        }
        let mut i: usize = numary.len();
        let mut v = Vec::with_capacity(numary.len());
        while i > BigUint::RADIX_DIGIT {
            let part = &numary[i - BigUint::RADIX_DIGIT..i];
            v.push(part.parse().unwrap_or(0));
            i -= BigUint::RADIX_DIGIT;
        }
        v.push(numary[..i].parse().unwrap_or(0));
        //上位桁の0をポップ
        while v.len() > 1 && match v.last() {Some(e) => *e == 0, None => false} {
            v.pop();
        }
        BigUint{
            digits: v
        }
    }

    fn del_zero(v: &mut Vec<u64>){
        while v.len() > 1 && *v.last().unwrap() == 0 {
            v.pop();
        }
    }
}

impl std::fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        if self.sign { 
            match write!(f, "-") {
                Ok(_n) => {},
                Err(m) => { return Err(m); }
            }
        }
        for elem in self.digits.iter().rev() {
            match write!(f, "{}", *elem) {
                Ok(_n) => {},
                Err(m) => { return Err(m); }
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for BigUint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        for elem in self.digits.iter().rev() {
            match write!(f, "{}", *elem) {
                Ok(_n) => {},
                Err(m) => { return Err(m); }
            }
        }
        Ok(())
    }
}

impl std::ops::Add for BigUint{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let l = std::cmp::max(self.digits.len(), other.digits.len());
        let mut v = Vec::with_capacity(l + 1);
        let mut i = -1i64;
        let mut carry = 0u8;
        v.resize_with(l, || {
            i += 1;
            let mut tmp = self.digits.get(i as usize).unwrap_or(&0u64) + other.digits.get(i as usize).unwrap_or(&0u64) + carry as u64;
            if tmp >= BigUint::RADIX { tmp -= BigUint::RADIX; carry = 1u8;}
            else {carry = 0;}
            tmp
        });
        if carry == 1 { v.push(1); }
        Self{ digits: v }
    }
}

impl std::ops::Sub for BigUint {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let l = std::cmp::max(self.digits.len(), other.digits.len());
        let mut v = Vec::with_capacity(l + 1);
        let mut i = -1i64;
        let mut carry = 0i8;
        v.resize_with(l, || {
            i += 1;
            let mut tmp = *self.digits.get(i as usize).unwrap_or(&0u64) as i64 + *other.digits.get(i as usize).unwrap_or(&0u64) as i64 + carry as i64;
            if tmp < 0 { tmp += BigUint::RADIX as i64; carry = -1i8; }
            else { carry = 0; }
            tmp as u64
        });
        if carry == -1 { panic!("BigUint overflow."); }
        BigUint::del_zero(&mut v);
        Self{ digits: v }
    }
}

#[allow(dead_code)]
fn powm(base: i64, exp: i64, modulo: i64) -> i64 {
    if exp <= 0 { return 1i64; }
    else if exp == 1 { return base; }
    else {
        let tmp: u128 = powm(base, exp / 2, modulo) as u128;
        if exp % 2 == 0 {(tmp * tmp % modulo as u128) as i64}
        else {(tmp * tmp % modulo as u128 * base as u128 % modulo as u128) as i64}
    }
}

struct NttTable{
    table: Vec<i64>,
    modulo: i64,
    prim_root: i64
}

impl NttTable{
    const MAX_SIZE: usize = 1 << 26;
    fn new(modulo: i64, prim_root: i64) -> Self {
        let mut t = Vec::with_capacity(NttTable::MAX_SIZE);
        t.resize(NttTable::MAX_SIZE, 0);
        Self{
            table: t,
            modulo,
            prim_root
        }
    }
}
/*
impl std::ops::Mul for BigUint {
    type Output = Self;
    fn mul(self, other: Self) -> Self {

    }
}
*/
/*
後で
impl std::ops::Add for BigInt {
    type Output = Self;
    fn add(self, Other: Self) -> Self {
        let mut v = Vec::with_capacity(std::cmp::max(self.digits.len(), Other.digits.len()));
        let mut i = -1i64;
        v.resize_with(std::cmp::max(self.digits.len(), Other.digits.len()), || { 
            i += 1;
            self.digits.get(i as usize).unwrap_or(&0i64) + Other.digits.get(i as usize).unwrap_or(&0i64)
        });
        Self{
            digits: v,
        }
    }
}
*/