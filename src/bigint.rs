use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Clone)]
pub struct BigInt{
    digits: Vec<i64>,
    sign: bool
}

#[derive(Clone)]
pub struct BigUint{
    digits: Vec<i64>
}

#[allow(dead_code)]
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
    const RADIX: i64 = 100000;

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

    fn del_zero(v: &mut Vec<i64>){
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
            let mut tmp = self.digits.get(i as usize).unwrap_or(&0i64) + other.digits.get(i as usize).unwrap_or(&0i64) + carry as i64;
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
            let mut tmp = *self.digits.get(i as usize).unwrap_or(&0i64) + *other.digits.get(i as usize).unwrap_or(&0i64) + carry as i64;
            if tmp < 0 { tmp += BigUint::RADIX; carry = -1i8; }
            else { carry = 0; }
            tmp
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

#[allow(dead_code)]
fn invm(num: i64, modulo: i64) -> i64 {
    let mut a = num;
    let mut b = modulo;
    let mut u = 1i64;
    let mut v = 0i64;
    while b != 0 {
        let tmp = u;
        u = v;
        v = tmp - a / b * v;

        let tmp = a;
        a = b;
        b = tmp % b;
    }
    u %= modulo;
    if u < 0 { u + modulo }
    else { u }
}

#[derive(Clone)]
struct NttTable{
    table: Vec<i64>,
    modulo: i64,
    prim_root: i64, 
    max_root_num: usize,
    calc: usize
}

impl NttTable{
    const MAX_SIZE: usize = 1 << 4;
    fn new(modulo: i64, prim_root: i64) -> Self {
        let mut t = Vec::with_capacity(NttTable::MAX_SIZE + 1);
        t.resize(NttTable::MAX_SIZE + 1, 0);
        t[NttTable::MAX_SIZE] = 1;
        
        Self{
            table: t,
            modulo,
            prim_root,
            max_root_num: (modulo - 1 & -(modulo - 1)) as usize,
            calc: 0
        }
    }
    //2^ord乗根を計算
    fn calculate(&mut self, ord: usize){
        let add = NttTable::MAX_SIZE >> ord; 
        if add == 0 || (add & self.calc) != 0 {return;}
        let mut i = 0usize;
        let mut root: u128 = 1;
        let prim = powm(self.prim_root, (self.max_root_num >> ord) as i64, self.modulo);
        while i < NttTable::MAX_SIZE {
            if self.table[i] == 0 { self.table[i] = root as i64; }
            root *= prim as u128;
            root %= self.modulo as u128;
            i += add;
        }
        self.calc |= (-(add as i64)) as usize;
    }

    fn trans_f(&mut self, dat: &Vec<i64>, new_size: usize, ord: usize) -> Vec<i64> {
        if new_size > NttTable::MAX_SIZE { panic!("vector too large"); }
        let mut tmp = Vec::with_capacity(new_size);
        tmp.extend_from_slice(dat);
        tmp.resize(new_size, 0);
        self.calculate(ord);

        let mut loop1 = 1usize;
        let mut loop2 = new_size / 2;
        let mut dist = NttTable::MAX_SIZE / new_size;
        while loop2 != 0 {
            let mut ind1: usize = 0;
            let mut ind2: usize = loop2;
            for _i in 0..loop1 {
                let mut indr: usize = 0;
                for _i in 0..loop2 {
                    let t = tmp[ind1];
                    tmp[ind1] += tmp[ind2];
                    tmp[ind1] %= self.modulo;
                    tmp[ind2] = ((t as u128 + self.modulo as u128 - tmp[ind2] as u128) * self.table[indr] as u128 % self.modulo as u128) as i64;
                    //println!("{}", self.table[indr]);
                    ind1 += 1;
                    ind2 += 1;
                    indr += dist;
                }
                ind1 += loop2;
                ind2 += loop2;
            }
            dist <<= 1;
            loop1 <<= 1;
            loop2 >>= 1;
        }
        tmp
    }

    fn trans_t_i(&mut self, dat: &Vec<i64>, new_size: usize, ord: usize) -> Vec<i64> {
        if new_size > NttTable::MAX_SIZE { panic!("vector too large"); }
        let mut tmp = Vec::with_capacity(new_size);
        tmp.extend_from_slice(dat);
        tmp.resize(new_size, 0);
        self.calculate(ord);

        let mut loop1 = new_size / 2;
        let mut loop2 = 1usize;
        let mut dist = NttTable::MAX_SIZE / 2;
        while loop1 != 0 {
            let mut ind1: usize = 0;
            let mut ind2: usize = loop2;
            for _i in 0..loop1 {
                let mut indr: usize = NttTable::MAX_SIZE;
                for _i in 0..loop2 {
                    let t = tmp[ind1];
                    tmp[ind2] = (tmp[ind2] as u128 * self.table[indr] as u128 % self.modulo as u128) as i64;
                    //println!("{}", self.table[indr]);
                    tmp[ind1] += tmp[ind2];
                    tmp[ind1] %= self.modulo;
                    tmp[ind2] = ((t as u128 + self.modulo as u128 - tmp[ind2] as u128) % self.modulo as u128) as i64;
                    ind1 += 1;
                    ind2 += 1;
                    indr -= dist;
                }
                ind1 += loop2;
                ind2 += loop2;
            }
            dist >>= 1;
            loop1 >>= 1;
            loop2 <<= 1;
        }
        tmp
    }

    fn hadamard(&self, v1: &mut Vec<i64>, v2: Vec<i64>) {
        if v1.len() != v2.len() {panic!("not same size in Hadamard");}
        for (i, elem) in v2.iter().enumerate() {
            v1[i] = (v1[i] as u128 * *elem as u128 % self.modulo as u128) as i64;
        }
    }

    fn scale(&self, v1: &mut Vec<i64>, size: usize) {
        let prod = invm(size as i64, self.modulo) as u128;
        for i in 0..v1.len() {
            v1[i] = (v1[i] as u128 * prod % self.modulo as u128) as i64;
        }
    }
}


fn convolution(num1: &Vec<i64>, num2: &Vec<i64>) -> Vec<i64> {
    static TBL: Lazy<Mutex<RefCell<NttTable>>> = Lazy::new(|| Mutex::new(RefCell::new(NttTable::new(4179340454199820289, 3))));
    let t_mtx = TBL.lock().unwrap();
    let mut t = t_mtx.borrow_mut();
    let mut len = num1.len() + num2.len();
    let mut ord = 0;
    if (len & 0xffffffff00000000 as usize) != 0 { ord += 32; len &= 0xffffffff00000000; }
    if (len & 0xffff0000ffff0000 as usize) != 0 { ord += 16; len &= 0xffff0000ffff0000; }
    if (len & 0xff00ff00ff00ff00 as usize) != 0 { ord += 8; len &= 0xff00ff00ff00ff00; }
    if (len & 0xf0f0f0f0f0f0f0f0 as usize) != 0 { ord += 4; len &= 0xf0f0f0f0f0f0f0f0; }
    if (len & 0xcccccccccccccccc as usize) != 0 { ord += 2; len &= 0xcccccccccccccccc; }
    if (len & 0xaaaaaaaaaaaaaaaa as usize) != 0 { ord += 1; len &= 0xaaaaaaaaaaaaaaaa; }
    let mut result = t.trans_f(num1, len, ord);
    let tmp = t.trans_f(num2, len, ord);
    t.hadamard(&mut result, tmp);
    let mut result = t.trans_t_i(&result, len, ord);
    result.resize(num1.len() + num2.len(), 0);
    t.scale(&mut result, len);
    result
}


impl std::ops::Mul for BigUint {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut v = convolution(&self.digits, &other.digits);
        BigUint::del_zero(&mut v);
        BigUint{
            digits: v
        }
    }
}

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