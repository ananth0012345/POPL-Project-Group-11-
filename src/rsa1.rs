extern crate rsa; 
extern crate rand; 
extern crate hex; 
use rsa::{PublicKey, RSAPrivateKey, PaddingScheme}; 
use rand::rngs::OsRng; 
use std::str; 
use std::env; 
// use num_bigint::{BigUint}; 
use num_traits::{One}; 
 
fn main()  
{ 
let mut rng = OsRng; 
 let mut bits = 512; 
let mut string = String::from("Hello world!"); 
let args: Vec<String>  = env::args().collect(); 
 
if args.len() >1  
{  
      string = args[1].clone(); 
} 
 if args.len()> 2  
 {  
      bits = args[2].clone().parse::<usize>().unwrap();  
}  
   
let key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key"); 
 
println!("Message:\t{}",string); 
 println!("Number of bits:\t{}",bits); 
 
let data = string.as_bytes(); 
 
println!("\nN:\t{} (Hex: {:x})",key.n(),key.n()); 
 println!("E:\t{} (Hex: {:x})",key.e(),key.e()); 
 println!("D:\t{} (Hex: {:x})",key.d(),key.d() ); 
 println!("\nPrimes (P and Q):"); 
 
 for prime in key.primes()  
 { 
  println!("\t{} (Hex:{:x})",prime,prime); 
  } 
 
let enc_data = key.encrypt(&mut rng, PaddingScheme::PKCS1v15, &data[..]).expect("failed to encrypt"); 
let hex_string = hex::encode(enc_data.clone()); 
 
println!("\n\nEncrypted:\t{}",hex_string); 
 
let dec_data = key.decrypt(PaddingScheme::PKCS1v15, &enc_data).expect("failed to decrypt"); 
let mystr = str::from_utf8(&dec_data).unwrap(); 
 
println!("\nDecrypted :\t{}",mystr); 
 
// Final check for (d x e) mod (p-1)*(q-1) 
 
let p=  key.primes()[0].clone(); 
 let q=  key.primes()[1].clone(); 
let val1: rsa::BigUint = One::one(); 
let phi = (p - val1.clone()) * (q - val1.clone()); 
let val = (key.d()*key.e()) % phi; 
 
 println!("\n(d*e) mod (p-1)(q-1):\t{}",val); 
 
}
