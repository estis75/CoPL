use super::{nat, compare_nat1, compare_nat2, compare_nat3, evalnatexp, reducenatexp};
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
      _ => panic!("not implemented: {:?}", self)
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
