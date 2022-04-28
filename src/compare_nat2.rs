use super::solver::{Solver, RuleTree, Object};
use regex::Regex;
use std::fmt::{self, Display, Formatter};

pub struct CompareNat2 {
  pub obj: String
}

#[derive(Debug, Clone)]
pub enum Rule{
  LZero,
  LSuccSucc,
}
impl Display for Rule {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::LZero => write!(f, "L-Zero"),
      Self::LSuccSucc => write!(f, "L-SuccSucc"),
    }
  }
}

impl CompareNat2 {
  pub fn solve(&self) -> Option<RuleTree> {
    self.solver()
  }

  fn get_nat(&self, arg: &str) -> usize {
    if let Some(c) = Regex::new(r"S\((.*)\)").unwrap().captures_iter(arg).next() {
      self.get_nat(&c[1]) + 1
    }else{
      0
    }
  }

  fn get_string(&self, num: usize) -> String {
    if num == 0 {
      String::from("Z")
    }else{
      format!("S({})", self.get_string(num-1))
    }
  }
}

impl Solver for CompareNat2 {
  fn solver(&self) -> Option<RuleTree> {
    let mut v = None;
    let mut state = false;

    if let Some(cap) = self.get_regex(Object::CompareNat2(Rule::LZero)).captures_iter(&self.obj).next() {
      v = Some(RuleTree{
        obj: Object::CompareNat2(Rule::LZero),
        val: self.obj.clone(),
        node: None
      });
      state = true;
    }

    if let Some(cap) = self.get_regex(Object::CompareNat2(Rule::LSuccSucc)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(1);
      let val = format!("{} is less than {}", &cap[1], &cap[2]);
      tp.push(CompareNat2{obj: val}.solver().unwrap());

      v = Some(RuleTree{
        obj: Object::CompareNat2(Rule::LSuccSucc),
        val: self.obj.clone(),
        node: Some(tp)
      });
      state = true;
    }

    if state {
      v
    }else{
      panic!("nothing is matched: {}", self.obj);
    }
  }

  fn get_regex(&self, obj: Object) -> Regex{
    if let Object::CompareNat2(c) = obj {
      match c {
        Rule::LZero => Regex::new(r"Z is less than S\((.*)\)").unwrap(),
        Rule::LSuccSucc => Regex::new(r"S\((.*)\) is less than S\((.*)\)").unwrap(),
      }
    }else{
      panic!("invalid object type: {:?}", obj)
    }
  }
}
