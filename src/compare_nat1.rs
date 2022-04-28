use super::solver::{Solver, RuleTree, Object};
use regex::Regex;
use std::fmt::{self, Display, Formatter};

pub struct CompareNat1 {
  pub obj: String
}

#[derive(Debug, Clone)]
pub enum Rule{
  LSucc,
  LTrans,
}
impl Display for Rule {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::LSucc => write!(f, "L-Succ"),
      Self::LTrans => write!(f, "L-Trans"),
    }
  }
}

impl CompareNat1 {
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

impl Solver for CompareNat1 {
  fn solver(&self) -> Option<RuleTree> {
    let mut v = None;
    let mut state = false;

    if let Some(cap) = self.get_regex(Object::CompareNat1(Rule::LSucc)).captures_iter(&self.obj).next() {
      if cap[1] == cap[2] {
        v = Some(RuleTree{
          obj: Object::CompareNat1(Rule::LSucc),
          val: self.obj.clone(),
          node: None
        });
        state = true;
      }
    }

    if let Some(cap) = self.get_regex(Object::CompareNat1(Rule::LTrans)).captures_iter(&self.obj).next() {
      if String::from("S(") + &cap[1] + ")" != cap[2] {
        let mut tp = Vec::with_capacity(2);
        let mulval = self.get_nat(&cap[1]) + 1;
        let val = format!("{} is less than {}", &cap[1], &self.get_string(mulval));
        tp.push(CompareNat1{obj: val}.solver().unwrap());
        let val = format!("{} is less than {}", &self.get_string(mulval), &cap[2]);
        tp.push(CompareNat1{obj: val}.solver().unwrap());

        v = Some(RuleTree{
          obj: Object::CompareNat1(Rule::LTrans),
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
    if let Object::CompareNat1(c) = obj {
      match c {
        Rule::LSucc => Regex::new(r"(.*) is less than S\((.*)\)").unwrap(),
        Rule::LTrans => Regex::new(r"(.*) is less than (.*)").unwrap(),
      }
    }else{
      panic!("invalid object type: {:?}", obj)
    }
  }
}
