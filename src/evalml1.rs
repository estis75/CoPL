use super::solver::{Solver, RuleTree, Object};
use regex::Regex;
use std::fmt::{self, Display, Formatter};
mod derivation;
#[allow(unused)]
use derivation::*;

pub struct EvalML1 {
  pub obj: String
}

#[derive(Debug, Clone)]
pub enum Rule{
  EInt,
  EBool,
  EIfT,
  EIfF,
  EPlus,
  EMinus,
  ETimes,
  ELt,
  BPlus,
  BMinus,
  BTimes,
  BLt,
}
impl Display for Rule {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::EInt => write!(f, "E-Int"),
      Self::EBool => write!(f, "E-Bool"),
      Self::EIfT => write!(f, "E-IfT"),
      Self::EIfF => write!(f, "E-IfF"),
      Self::EPlus => write!(f, "E-Plus"),
      Self::EMinus => write!(f, "E-Minus"),
      Self::ETimes => write!(f, "E-Times"),
      Self::ELt => write!(f, "E-Lt"),
      Self::BPlus => write!(f, "B-Plus"),
      Self::BMinus => write!(f, "B-Minus"),
      Self::BTimes => write!(f, "B-Times"),
      Self::BLt => write!(f, "B-Lt"),
    }
  }
}

impl EvalML1 {
  #[allow(unused)]
  pub fn solve(&self) -> Option<RuleTree> {
    self.solver()
  }
}

impl Solver for EvalML1 {
  fn solver(&self) -> Option<RuleTree> {
    let mut v = None;
    let mut state = false;
    dbg!(&self.obj);

    if !state { (v, state) = self.get_tree_eint(); }
    if !state { (v, state) = self.get_tree_ebool(); }

    if !state { (v, state) = self.get_tree_eift(); }
    if !state { (v, state) = self.get_tree_eiff(); }

    if !state { (v, state) = self.get_tree_eplus(); }
    if !state { (v, state) = self.get_tree_eminus(); }
    if !state { (v, state) = self.get_tree_etimes(); }
    if !state { (v, state) = self.get_tree_elt(); }

    if !state { (v, state) = self.get_tree_bplus(); }
    if !state { (v, state) = self.get_tree_bminus(); }
    if !state { (v, state) = self.get_tree_btimes(); }
    if !state { (v, state) = self.get_tree_blt(); }

    if state {
      v
    }else{
      // panic!("nothing is matched: {}", self.obj);
      None
    }
  }

  fn get_regex(&self, obj: Object) -> Regex{
    if let Object::EvalML1(c) = obj {
      match c {
        Rule::EInt => Regex::new(r"(.*) evalto (.*)").unwrap(),
        Rule::EBool => Regex::new(r"(.*) evalto (.*)").unwrap(),
        Rule::EIfT => Regex::new(r"^if (.*) then (.*) else (.*) evalto (.*)").unwrap(),
        Rule::EIfF => Regex::new(r"^if (.*) then (.*) else (.*) evalto (.*)").unwrap(),
        Rule::EPlus => Regex::new(r"(.*) \+ (.*) evalto (.*)").unwrap(),
        Rule::EMinus => Regex::new(r"(.*) - (.*) evalto (.*)").unwrap(),
        Rule::ETimes => Regex::new(r"(.*) \* (.*) evalto (.*)").unwrap(),
        Rule::ELt => Regex::new(r"(.*) < (.*) evalto (.*)").unwrap(),
        Rule::BPlus => Regex::new(r"(.*) plus (.*) is (.*)").unwrap(),
        Rule::BMinus => Regex::new(r"(.*) minus (.*) is (.*)").unwrap(),
        Rule::BTimes => Regex::new(r"(.*) times (.*) is (.*)").unwrap(),
        Rule::BLt => Regex::new(r"(.*) less than (.*) is (.*)").unwrap(),
      }
    }else{
      panic!("invalid object type: {:?}", obj)
    }
  }
}
