use super::solver::{Solver, RuleTree, Object};
use regex::Regex;
use std::fmt::{self, Display, Formatter};

pub struct CompareNat3 {
  pub obj: String
}

#[derive(Debug, Clone)]
pub enum Rule{
  LSucc,
  LSuccR,
}
impl Display for Rule {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::LSucc => write!(f, "L-Succ"),
      Self::LSuccR => write!(f, "L-SuccR"),
    }
  }
}

impl CompareNat3 {
  #[allow(unused)]
  pub fn solve(&self) -> Option<RuleTree> {
    self.solver()
  }
}

impl Solver for CompareNat3 {
  fn solver(&self) -> Option<RuleTree> {
    let mut v = None;
    let mut state = false;

    if let Some(cap) = self.get_regex(Object::CompareNat3(Rule::LSucc)).captures_iter(&self.obj).next() {
      if cap[1] == cap[2] {
        v = Some(RuleTree{
          obj: Object::CompareNat3(Rule::LSucc),
          val: self.obj.clone(),
          node: None
        });
        state = true;
      }
    }

    if let Some(cap) = self.get_regex(Object::CompareNat3(Rule::LSuccR)).captures_iter(&self.obj).next() {
      if cap[1] != cap[2] {
        let mut tp = Vec::with_capacity(1);
        let val = format!("{} is less than {}", &cap[1], &cap[2]);
        tp.push(CompareNat3{obj: val}.solver().unwrap());

        v = Some(RuleTree{
          obj: Object::CompareNat3(Rule::LSuccR),
          val: self.obj.clone(),
          node: Some(tp)
        });
        state = true;
      }
    }

    if state {
      v
    }else{
      panic!("nothing is matched: {}", self.obj);
    }
  }

  fn get_regex(&self, obj: Object) -> Regex{
    if let Object::CompareNat3(c) = obj {
      match c {
        Rule::LSucc => Regex::new(r"(.*) is less than S\((.*)\)").unwrap(),
        Rule::LSuccR => Regex::new(r"(.*) is less than S\((.*)\)").unwrap(),
      }
    }else{
      panic!("invalid object type: {:?}", obj)
    }
  }
}
