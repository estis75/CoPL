mod nat;
use nat::Nat;

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

mod solver;
use std::io::stdin;

fn main() {
  let mut obj = String::new();
  stdin().read_line(&mut obj).unwrap();
  obj = obj.trim().to_string();

  let tree = ReduceNatExp{obj: obj.clone()}.solve();
  if let Some(n) = tree {
    println!("{}", n);
  }else{
    panic!("invalid input: {}", obj);
  }
}


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