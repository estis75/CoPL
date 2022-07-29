use super::{
  nat, 
  compare_nat1, compare_nat2, compare_nat3, 
  evalnatexp, reducenatexp, 
  evalml1, evalml1err,
  evalml2, 
  evalml3, namelessml3, evalnamelessml3,
};
use regex::Regex;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Object {
  Nat(nat::Rule),
  CompareNat1(compare_nat1::Rule),
  CompareNat2(compare_nat2::Rule),
  CompareNat3(compare_nat3::Rule),
  EvalNatExp(evalnatexp::Rule),
  ReduceNatExp(reducenatexp::Rule),
  EvalML1(evalml1::Rule),
  EvalML1Err(evalml1err::Rule),
  EvalML2(evalml2::Rule),
  EvalML3(evalml3::Rule),
  NamelessML3(namelessml3::Rule),
  EvalNamelessML3(evalnamelessml3::Rule),
}

impl Display for Object {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Nat(c) => write!(f, "{}", c),
      Self::CompareNat1(c) => write!(f, "{}", c),
      Self::CompareNat2(c) => write!(f, "{}", c),
      Self::CompareNat3(c) => write!(f, "{}", c),
      Self::EvalNatExp(c) => write!(f, "{}", c),
      Self::ReduceNatExp(c) => write!(f, "{}", c),
      Self::EvalML1(c) => write!(f, "{}", c),
      Self::EvalML1Err(c) => write!(f, "{}", c),
      Self::EvalML2(c) => write!(f, "{}", c),
      Self::EvalML3(c) => write!(f, "{}", c),
      Self::NamelessML3(c) => write!(f, "{}", c),
      Self::EvalNamelessML3(c) => write!(f, "{}", c),
    }
  }
}

#[derive(Debug, Clone)]
pub struct RuleTree {
  pub obj: Object,
  pub val: String,
  pub node: Option<Vec::<RuleTree>>
}

impl Display for RuleTree {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    if let Some(root) = self.node.clone() {
      write!(f, 
        "\n{} by {} {{{}}};\n",
        self.val,
        self.obj, 
        root.iter().fold(
          String::new(), |l, val| l + &format!("{}",val)
        ), 
      )
    }else{
      write!(f, "\n{} by {} {{}};\n", &self.val, self.obj)
    }
  }
}

pub trait Solver {
  fn solver(&self) -> Option<RuleTree>;
  fn get_regex(&self, obj: Object) -> Regex;
}
