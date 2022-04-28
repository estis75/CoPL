mod nat;
use nat::Nat;

mod compare_nat1;
use compare_nat1::CompareNat1;
mod compare_nat2;
use compare_nat2::CompareNat2;
mod compare_nat3;
use compare_nat3::CompareNat3;

mod evalnatexp;
use evalnatexp::EvalNatExp;

mod solver;
use std::io::stdin;

fn main() {
    let mut obj = String::new();
    stdin().read_line(&mut obj).unwrap();
    obj = obj.trim().to_string();

    let tree = EvalNatExp{obj}.solve();
    if let Some(n) = tree {
        println!("{}", n);
    }else{
        println!("");

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