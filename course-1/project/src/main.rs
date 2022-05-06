extern crate glass_pumpkin;


extern crate num_traits;




use num_traits::*;

use glass_pumpkin::prime;

use num_bigint::{BigInt,Sign};

use std::env;


fn modinv(a0: BigInt, m0: BigInt) -> BigInt {
    if m0 == one() {return one()}
    let (mut a, mut m, mut x0, mut inv) = (a0, m0.clone(), zero(), one());
    while a > one() {
    inv -= (&a / &m) * &x0;
    a = (&a % &m);
    std::mem::swap(&mut a, &mut m);
    std::mem::swap(&mut x0, &mut inv)
    }
    if inv < zero() { inv += m0 }
    inv
    }


fn main(){

    let mut no_bits=512;
    let mut val="999";

  let args: Vec<String>   = env::args().collect();

  if args.len()> 1 { val= args[1].as_str();}
  if args.len()> 2 { no_bits = args[2].clone().parse::<usize>().unwrap(); }


   let  p = BigInt::from_biguint(Sign::Plus,prime::new(no_bits).unwrap());
   let  q = BigInt::from_biguint(Sign::Plus,prime::new(no_bits).unwrap());

    let  n = p.clone()* q.clone();

    let e = BigInt::parse_bytes(b"65537", 10).unwrap();
    let M = BigInt::parse_bytes(val.as_bytes(), 10).unwrap();
    let Enc = M.modpow(&e, &n);

    println!("Input={}",val);
    println!("No of bits={}",no_bits);
    println!("p={:?}\nq={:?}\nn={:?}", p,q,n);

    let totient = (p - BigInt::one())*(q-BigInt::one());
    let d=modinv(e.clone(),totient.clone());
  
    println!("\nd={}",d);

    println!("\nEncrypted={}",Enc);



    let Dec = Enc.modpow(&d,&n);
    println!("\nDecrypted={}",Dec);




}