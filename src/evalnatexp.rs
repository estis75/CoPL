use super::solver::{Solver, RuleTree, Object};
use regex::Regex;
use std::fmt::{self, Display, Formatter};

pub struct EvalNatExp {
  pub obj: String
}

#[derive(Debug, Clone)]
pub enum Rule{
  EConst,
  EPlus,
  ETimes,
  PSucc,
  PZero,
  TSucc,
  TZero,
}
impl Display for Rule {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::EConst => write!(f, "E-Const"),
      Self::EPlus => write!(f, "E-Plus"),
      Self::ETimes => write!(f, "E-Times"),
      Self::PZero => write!(f, "P-Zero"),
      Self::PSucc => write!(f, "P-Succ"),
      Self::TZero => write!(f, "T-Zero"),
      Self::TSucc => write!(f, "T-Succ"),
    }
  }
}

impl EvalNatExp {
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

  fn get_val(&self, e: &str) -> usize {
    if let Some(c) = Regex::new(r"(.*) \+ (.*)").unwrap().captures_iter(e).next() {
      self.get_val(&c[1]) + self.get_val(&c[2])
    }else{
      if let Some(c) = Regex::new(r"(.*) \* (.*)").unwrap().captures_iter(e).next() {
        self.get_val(&c[1]) * self.get_val(&c[2])
      }else{
        self.get_nat(e)
      }
    }
  }
}

impl Solver for EvalNatExp {
  fn solver(&self) -> Option<RuleTree> {
    let mut v = None;
    let mut state = false;

    if let Some(cap) = self.get_regex(Object::EvalNatExp(Rule::EConst)).captures_iter(&self.obj).next() {
      if cap[1] == cap[2] {
        v = Some(RuleTree{
          obj: Object::EvalNatExp(Rule::EConst),
          val: self.obj.clone(),
          node: None
        });
        state = true;
      }
    }

    if let Some(cap) = self.get_regex(Object::EvalNatExp(Rule::ETimes)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let lval = self.get_string(self.get_val(&cap[1]));
      let val = format!("{} evalto {}", &cap[1], &lval);
      tp.push(EvalNatExp{obj: val}.solver().unwrap());

      let rval = self.get_string(self.get_val(&cap[2]));
      let val = format!("{} evalto {}", &cap[2], &rval);
      tp.push(EvalNatExp{obj: val}.solver().unwrap());

      let val = format!("{} times {} is {}", &lval, &rval, &cap[3]);
      tp.push(EvalNatExp{obj: val}.solver().unwrap());

      v = Some(RuleTree{
        obj: Object::EvalNatExp(Rule::ETimes),
        val: self.obj.clone(),
        node: Some(tp)
      });
      state = true;
    }

    if let Some(cap) = self.get_regex(Object::EvalNatExp(Rule::EPlus)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let lval = self.get_string(self.get_val(&cap[1]));
      let val = format!("{} evalto {}", &cap[1], &lval);
      tp.push(EvalNatExp{obj: val}.solver().unwrap());

      let rval = self.get_string(self.get_val(&cap[2]));
      let val = format!("{} evalto {}", &cap[2], &rval);
      tp.push(EvalNatExp{obj: val}.solver().unwrap());

      let val = format!("{} plus {} is {}", &lval, &rval, &cap[3]);
      tp.push(EvalNatExp{obj: val}.solver().unwrap());

      v = Some(RuleTree{
        obj: Object::EvalNatExp(Rule::EPlus),
        val: self.obj.clone(),
        node: Some(tp)
      });
      state = true;
    }

    if let Some(cap) = self.get_regex(Object::EvalNatExp(Rule::PZero)).captures_iter(&self.obj).next() {
      if cap[1] == cap[2] {
        v = Some(RuleTree{
          obj: Object::EvalNatExp(Rule::PZero),
          val: self.obj.clone(),
          node: None
        });
        state = true;
      }else{
        panic!("this is not a valid statement: {}", self.obj)
      }
    }

    if let Some(cap) = self.get_regex(Object::EvalNatExp(Rule::PSucc)).captures_iter(&self.obj).next() {
      let val = format!("{} plus {} is {}", &cap[1], &cap[2], &cap[3]);
      let tp = EvalNatExp{obj: val}.solver();
      let node = if let Some(c) = tp {
        Some(vec!{c})
      }else{
        None
      };

      v = Some(RuleTree{
        obj: Object::EvalNatExp(Rule::PSucc),
        val: self.obj.clone(),
        node: node
      });
      state = true;
    }

    if let Some(cap) = self.get_regex(Object::EvalNatExp(Rule::TZero)).captures_iter(&self.obj).next() {
      v = Some(RuleTree{
        obj: Object::EvalNatExp(Rule::TZero),
        val: self.obj.clone(),
        node: None
      });
      state = true;
    }

    if let Some(cap) = self.get_regex(Object::EvalNatExp(Rule::TSucc)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(2);
      let mulval = self.get_nat(&cap[1]) * self.get_nat(&cap[2]);
      // println!("{} times {} is {}", &cap[1], &cap[2], &self.get_string(mulval));
      let val = format!("{} times {} is {}", &cap[1], &cap[2], &self.get_string(mulval));
      tp.push(EvalNatExp{obj: val}.solver().unwrap());
      // println!("{} plus {} is {}", &cap[2], &self.get_string(mulval), &cap[3]);
      let val = format!("{} plus {} is {}", &cap[2], &self.get_string(mulval), &cap[3]);
      tp.push(EvalNatExp{obj: val}.solver().unwrap());

      v = Some(RuleTree{
        obj: Object::EvalNatExp(Rule::TSucc),
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
    if let Object::EvalNatExp(c) = obj {
      match c {
        Rule::EConst => Regex::new(r"(.*) evalto (.*)").unwrap(),
        Rule::EPlus => Regex::new(r"(.*) \+ (.*) evalto (.*)").unwrap(),
        Rule::ETimes => Regex::new(r"(.*) \* (.*) evalto (.*)").unwrap(),
        Rule::PZero => Regex::new(r"Z plus (.*) is (.*)").unwrap(),
        Rule::PSucc => Regex::new(r"S\((.*)\) plus (.*) is S\((.*)\)").unwrap(),
        Rule::TZero => Regex::new(r"Z times (.*) is Z").unwrap(),
        Rule::TSucc => Regex::new(r"S\((.*)\) times (.*) is (.*)").unwrap(),
      }
    }else{
      panic!("invalid object type: {:?}", obj)
    }
  }
}
