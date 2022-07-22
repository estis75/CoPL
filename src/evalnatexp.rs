use super::solver::{Solver, RuleTree, Object};
use regex::Regex;
use std::fmt::{self, Display, Formatter};

pub struct EvalNatExp {
  pub obj: String
}

#[derive(Debug, Clone)]
pub enum Rule{
  EConst,
  EParen,
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
      Self::EParen => write!(f, "E-Paren"),
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

  fn get_nat(&self, arg: &str) -> Option<usize> {
    if let Some(c) = Regex::new(r"S\((.*)\)").unwrap().captures_iter(arg).next() {
      if let Some(c) = self.get_nat(&c[1]) {
        Some(c + 1)
      }else{
        None
      }
    }else if arg == String::from("Z") {
      Some(0)
    }else{
      None
    }
  }

  fn get_string(&self, num: usize) -> String {
    if num == 0 {
      String::from("Z")
    }else{
      format!("S({})", self.get_string(num-1))
    }
  }

  fn get_val(&self, e: &str) -> Option<usize> {
    // dbg!(e);
    if let Some(c) = Regex::new(r"(.*) \* (.*)").unwrap().captures_iter(e).next() {
      if let Some(lhs) = self.get_val(&c[1]) {
        if let Some(rhs) = self.get_val(&c[2]) {
          Some(lhs * rhs)
        }else{
          None
        }
      }else{
        None
      }
    }else if let Some(c) = Regex::new(r"[^S]\((.*)\)|^\((.*)\)").unwrap().captures_iter(e).next() {
      if let Some(c) = c.get(1) {
        self.get_val(&c.as_str())
      }else if let Some(c) = c.get(2){
        self.get_val(&c.as_str())
      }else{
        panic!("why does it match?");
      }
    }else if let Some(c) = Regex::new(r"(.*) \+ (.*)").unwrap().captures_iter(e).next() {
      if let Some(lhs) = self.get_val(&c[1]) {
        if let Some(rhs) = self.get_val(&c[2]) {
          Some(lhs + rhs)
        }else{
          None
        }
      }else{
        None
      }
    }else{
      self.get_nat(e)
    }
  }

  fn is_parend_exp(&self, exp: String) -> String {
    if let Some(cap) = self.get_regex(Object::EvalNatExp(Rule::EParen)).captures_iter(&exp).next() {
      if let Some(node) = self.get_val(&cap[1]) {
        let val = self.get_string(node);
        let tp_val = format!("{} evalto {}", &cap[1], &val);
        let tp = EvalNatExp{obj: tp_val}.solver();
        if let Some(_) = tp {
          format!("{} evalto {}", &cap[1], &val)
        }else{
          exp
        }
      }else{
        exp
      }
    }else{
      exp
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
      let mut tmp_state = true;

      let lval = if let Some(lhs) = self.get_val(&cap[1]) {
        let lval = self.get_string(lhs);
        let val = self.is_parend_exp(format!("{} evalto {}", &cap[1], &lval));
        tp.push(EvalNatExp{obj: val}.solver().unwrap());
        lval
      }else{
        tmp_state = false;
        String::from("")
      };

      let rval = if let Some(rhs) = self.get_val(&cap[2]) {
        let rval = self.get_string(rhs);
        let val = self.is_parend_exp(format!("{} evalto {}", &cap[2], &rval));
        tp.push(EvalNatExp{obj: val}.solver().unwrap());
        rval
      }else{
        tmp_state = false;
        String::from("")
      };

      if tmp_state {
        let val = format!("{} times {} is {}", &lval, &rval, &cap[3]);
        tp.push(EvalNatExp{obj: val}.solver().unwrap());

        v = Some(RuleTree{
          obj: Object::EvalNatExp(Rule::ETimes),
          val: self.obj.clone(),
          node: Some(tp)
        });
        state = true;
      }
    }

    if let Some(cap) = self.get_regex(Object::EvalNatExp(Rule::EPlus)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);
      let mut tmp_state = true;

      let lval = if let Some(lhs) = self.get_val(&cap[1]) {
        let lval = self.get_string(lhs);
        let val = self.is_parend_exp(format!("{} evalto {}", &cap[1], &lval));
        tp.push(EvalNatExp{obj: val}.solver().unwrap());
        lval
      }else{
        tmp_state = false;
        String::from("")
      };

      let rval = if let Some(rhs) = self.get_val(&cap[2]) {
        let rval = self.get_string(rhs);
        let val = self.is_parend_exp(format!("{} evalto {}", &cap[2], &rval));
        tp.push(EvalNatExp{obj: val}.solver().unwrap());
        rval
      }else{
        tmp_state = false;
        String::from("")
      };

      if tmp_state {
        let val = format!("{} plus {} is {}", &lval, &rval, &cap[3]);
        tp.push(EvalNatExp{obj: val}.solver().unwrap());

        v = Some(RuleTree{
          obj: Object::EvalNatExp(Rule::EPlus),
          val: self.obj.clone(),
          node: Some(tp)
        });
        state = true;
      }
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

    if let Some(_) = self.get_regex(Object::EvalNatExp(Rule::TZero)).captures_iter(&self.obj).next() {
      v = Some(RuleTree{
        obj: Object::EvalNatExp(Rule::TZero),
        val: self.obj.clone(),
        node: None
      });
      state = true;
    }

    if let Some(cap) = self.get_regex(Object::EvalNatExp(Rule::TSucc)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(2);
      let mulval = if let Some(lhs) = self.get_nat(&cap[1]) {
        if let Some(rhs) = self.get_nat(&cap[2]) {
          Some(lhs * rhs)
        }else{
          None
        }
      }else{
        None
      };
      // println!("{} times {} is {}", &cap[1], &cap[2], &self.get_string(mulval));
      if let Some(mulval) = mulval {
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
    }

    if state {
      v
    }else{
      // panic!("nothing is matched: {}", self.obj);
      None
    }
  }

  fn get_regex(&self, obj: Object) -> Regex{
    if let Object::EvalNatExp(c) = obj {
      match c {
        Rule::EConst => Regex::new(r"(.*) evalto (.*)").unwrap(),
        Rule::EParen => Regex::new(r"^\((.*)\) evalto (.*)").unwrap(),
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
