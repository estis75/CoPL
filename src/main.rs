mod nat;
use nat::Nat;

#[allow(unused)]
mod compare_nat1;
#[allow(unused)]
use compare_nat1::CompareNat1;
mod compare_nat2;
#[allow(unused)]
use compare_nat2::CompareNat2;
mod compare_nat3;
#[allow(unused)]
use compare_nat3::CompareNat3;

mod evalnatexp;
#[allow(unused)]
use evalnatexp::EvalNatExp;
mod reducenatexp;
#[allow(unused)]
use reducenatexp::ReduceNatExp;

mod evalml1;
#[allow(unused)]
use evalml1::EvalML1;
mod evalml1err;
#[allow(unused)]
use evalml1err::EvalML1Err;
mod evalml2;
#[allow(unused)]
use evalml2::EvalML2;
mod evalml3;
#[allow(unused)]
use evalml3::EvalML3;
mod namelessml3;
#[allow(unused)]
use namelessml3::NamelessML3;
mod evalnamelessml3;
#[allow(unused)]
use evalnamelessml3::EvalNamelessML3;
mod evalml4;
#[allow(unused)]
use evalml4::EvalML4;

mod solver;
use std::io::stdin;

use std::io::BufRead;

fn main() {
  let stdin = stdin().lock();
  let mut obj = String::new();
  for rhs in stdin.lines() {
    let rhs = rhs.ok().unwrap();
    let rhs = rhs.trim();
    if rhs.len() == 0 {
      break;
    }else{
      obj = obj + " " + &rhs;
    }
  }

  let obj = obj.trim().to_string();
  let tree = EvalML4{obj: obj.clone()}.solve();
  if let Some(n) = tree {
    println!("{}", n);
  }else{
    panic!("invalid input: {}", obj);
  }
}


#[allow(unused)]
fn test_for_nat() {
    let n = Nat{obj: String::from("Z plus Z is Z")};
    println!("{:?}", n.solve());

    let n = Nat{obj: String::from("S(Z) plus Z is S(Z)")};
    println!("{:?}", n.solve());

    let n = Nat{obj: String::from("S(S(Z)) plus Z is S(S(Z))")};
    println!("{:?}", n.solve());

    let n = Nat{obj: String::from("Z times Z is Z")};
    println!("{:?}", n.solve());

    let n = Nat{obj: String::from("Z times S(Z) is Z")};
    println!("{:?}", n.solve());

    let n = Nat{obj: String::from("S(S(Z)) times S(Z) is S(S(Z))")}.solve();
    if let Some(c) = n {
        println!("{}", c);
    }else{
        println!("");
    }
}