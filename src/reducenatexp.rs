//
// unsolved: No.24
// MR-Multiを使うときにバラすのが大変そう
//
use super::solver::{Solver, RuleTree, Object};
use regex::Regex;
use std::fmt::{self, Display, Formatter};

pub struct ReduceNatExp {
  pub obj: String
}

#[derive(Debug, Clone)]
pub enum Rule{
  RPlus,
  RTimes,
  RPlusL,
  RPlusR,
  RTimesL,
  RTimesR,
  DRPlus,
  DRTimes,
  DRPlusL,
  DRPlusR,
  DRTimesL,
  DRTimesR,
  MRZero,
  MRMulti,
  MROne,
  EParen,
  PSucc,
  PZero,
  TSucc,
  TZero,
}
impl Display for Rule {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::RPlus => write!(f, "R-Plus"),
      Self::RTimes => write!(f, "R-Times"),
      Self::RPlusL => write!(f, "R-PlusL"),
      Self::RPlusR => write!(f, "R-PlusR"),
      Self::RTimesL => write!(f, "R-TimesL"),
      Self::RTimesR => write!(f, "R-TimesR"),
      Self::DRPlus => write!(f, "DR-Plus"),
      Self::DRTimes => write!(f, "DR-Times"),
      Self::DRPlusL => write!(f, "DR-PlusL"),
      Self::DRPlusR => write!(f, "DR-PlusR"),
      Self::DRTimesL => write!(f, "DR-TimesL"),
      Self::DRTimesR => write!(f, "DR-TimesR"),
      Self::MRZero => write!(f, "MR-Zero"),
      Self::MRMulti => write!(f, "MR-Multi"),
      Self::MROne => write!(f, "MR-One"),
      Self::EParen => write!(f, "E-Paren"),
      Self::PZero => write!(f, "P-Zero"),
      Self::PSucc => write!(f, "P-Succ"),
      Self::TZero => write!(f, "T-Zero"),
      Self::TSucc => write!(f, "T-Succ"),
    }
  }
}

impl ReduceNatExp {
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
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::EParen)).captures_iter(&exp).next() {
      if let Some(node) = self.get_val(&cap[1]) {
        let val = self.get_string(node);
        let tp_val = format!("{} evalto {}", &cap[1], &val);
        let tp = ReduceNatExp{obj: tp_val}.solver();
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

  fn get_tree_rtimes(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::RTimes)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} times {} is {}", &cap[1], &cap[2], &cap[3]);
      let c = ReduceNatExp{obj: val}.solver();
      if let Some(c) = c {
        tp.push(c);
        v = Some(RuleTree{
          obj: Object::ReduceNatExp(Rule::RTimes),
          val: self.obj.clone(),
          node: Some(tp)
        });
        state = true;
      }
    }
    (v, state)
  }

  fn get_tree_rtimesl(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::RTimesL)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      if cap[2] == cap[4] {
        let val = format!("{} ---> {}", &cap[1], &cap[3]);
        tp.push(ReduceNatExp{obj: val}.solver().unwrap());

        v = Some(RuleTree{
          obj: Object::ReduceNatExp(Rule::RTimesL),
          val: self.obj.clone(),
          node: Some(tp)
        });
        state = true;
      }
    }
    (v, state)
  }

  fn get_tree_rtimesr(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::RTimesR)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      if cap[1] == cap[3] {
        let val = format!("{} ---> {}", &cap[2], &cap[4]);
        tp.push(ReduceNatExp{obj: val}.solver().unwrap());

        v = Some(RuleTree{
          obj: Object::ReduceNatExp(Rule::RTimesR),
          val: self.obj.clone(),
          node: Some(tp)
        });
        state = true;
      }
    }
    (v, state)
  }

  fn get_tree_rplus(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::RPlus)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} plus {} is {}", &cap[1], &cap[2], &cap[3]);
      let c = ReduceNatExp{obj: val}.solver();
      if let Some(c) = c {
        tp.push(c);

        v = Some(RuleTree{
          obj: Object::ReduceNatExp(Rule::RPlus),
          val: self.obj.clone(),
          node: Some(tp)
        });
        state = true;
      }
    }
    (v, state)
  }

  fn get_tree_rplusl(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::RPlusL)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      if cap[2] == cap[4] {
        let val = format!("{} ---> {}", &cap[1], &cap[3]);
        tp.push(ReduceNatExp{obj: val}.solver().unwrap());

        v = Some(RuleTree{
          obj: Object::ReduceNatExp(Rule::RPlusL),
          val: self.obj.clone(),
          node: Some(tp)
        });
        state = true;
      }
    }
    (v, state)
  }

  fn get_tree_rplusr(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::RPlusR)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      if cap[1] == cap[3] {
        let val = format!("{} ---> {}", &cap[2], &cap[4]);
        tp.push(ReduceNatExp{obj: val}.solver().unwrap());

        v = Some(RuleTree{
          obj: Object::ReduceNatExp(Rule::RPlusR),
          val: self.obj.clone(),
          node: Some(tp)
        });
        state = true;
      }
    }
    (v, state)
  }

  fn get_tree_drtimes(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::DRTimes)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      if let Some(c1) = self.get_nat(&cap[1]) {
        if let Some(c2) = self.get_nat(&cap[2]) {
          if let Some(c3) = self.get_nat(&cap[3]) {
            if c1 * c2 == c3 {
              let val = format!("{} times {} is {}", &cap[1], &cap[2], &cap[3]);
              let c = ReduceNatExp{obj: val}.solver();
              if let Some(c) = c {
                tp.push(c);
                v = Some(RuleTree{
                  obj: Object::ReduceNatExp(Rule::DRTimes),
                  val: self.obj.clone(),
                  node: Some(tp)
                });
                state = true;
              }
            }
          }
        }
      }
    }
    (v, state)
  }

  fn get_tree_drtimesl(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::DRTimesL)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      if cap[2] == cap[4] {
        let val = format!("{} -d-> {}", &cap[1], &cap[3]);
        tp.push(ReduceNatExp{obj: val}.solver().unwrap());

        v = Some(RuleTree{
          obj: Object::ReduceNatExp(Rule::DRTimesL),
          val: self.obj.clone(),
          node: Some(tp)
        });
        state = true;
      }
    }
    (v, state)
  }

  fn get_tree_drtimesr(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::DRTimesR)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      if let Some(lhs) = self.get_nat(&cap[1]) {
        if let Some(rhs) = self.get_nat(&cap[3]) {
          if lhs == rhs {
            let val = format!("{} -d-> {}", &cap[2], &cap[4]);
            tp.push(ReduceNatExp{obj: val}.solver().unwrap());

            v = Some(RuleTree{
              obj: Object::ReduceNatExp(Rule::DRTimesR),
              val: self.obj.clone(),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  fn get_tree_drplus(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::DRPlus)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      if let Some(c1) = self.get_nat(&cap[1]) {
        if let Some(c2) = self.get_nat(&cap[2]) {
          if let Some(c3) = self.get_nat(&cap[3]) {
            if c1 + c2 == c3 {
              let val = format!("{} plus {} is {}", &cap[1], &cap[2], &cap[3]);
              tp.push(ReduceNatExp{obj: val}.solver().unwrap());

              v = Some(RuleTree{
                obj: Object::ReduceNatExp(Rule::DRPlus),
                val: self.obj.clone(),
                node: Some(tp)
              });
              state = true;
            }
          }
        }
      }
    }
    (v, state)
  }

  fn get_tree_drplusl(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::DRPlusL)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      if cap[2] == cap[4] {
        let val = format!("{} -d-> {}", &cap[1], &cap[3]);
        tp.push(ReduceNatExp{obj: val}.solver().unwrap());

        v = Some(RuleTree{
          obj: Object::ReduceNatExp(Rule::DRPlusL),
          val: self.obj.clone(),
          node: Some(tp)
        });
        state = true;
      }
    }
    (v, state)
  }

  fn get_tree_drplusr(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::DRPlusR)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      if let Some(lhs) = self.get_nat(&cap[1]) {
        if let Some(rhs) = self.get_nat(&cap[3]) {
          if lhs == rhs {
            let val = format!("{} -d-> {}", &cap[2], &cap[4]);
            tp.push(ReduceNatExp{obj: val}.solver().unwrap());

            v = Some(RuleTree{
              obj: Object::ReduceNatExp(Rule::DRPlusR),
              val: self.obj.clone(),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  fn get_tree_mrzero(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::MRZero)).captures_iter(&self.obj).next() {
      if cap[1] == cap[2] {
        v = Some(RuleTree{
          obj: Object::ReduceNatExp(Rule::MRZero),
          val: self.obj.clone(),
          node: None
        });
        state = true;
      }
    }
    (v, state)
  }

  fn get_tree_mrmulti(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::MRMulti)).captures_iter(&self.obj).next() {
      v = Some(RuleTree{
        obj: Object::ReduceNatExp(Rule::MRMulti),
        val: self.obj.clone(),
        node: None
      });
      state = true;
    }
    (v, state)
  }

  fn get_tree_mrone(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::MROne)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} ---> {}", &cap[1], &cap[2]);
      let c = ReduceNatExp{obj: val}.solver();
      if let Some(c) = c {
        tp.push(c);

        v = Some(RuleTree{
          obj: Object::ReduceNatExp(Rule::MROne),
          val: self.obj.clone(),
          node: Some(tp)
        });
        state = true;
      }
    }
    (v, state)
  }
}

impl Solver for ReduceNatExp {

  fn solver(&self) -> Option<RuleTree> {
    let mut v = None;
    let mut state = false;

    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::MRZero)).captures_iter(&self.obj).next() {
      if cap[1] == cap[2] {
        v = Some(RuleTree{
          obj: Object::ReduceNatExp(Rule::MRZero),
          val: self.obj.clone(),
          node: None
        });
        state = true;
      }
    }

    if !state { (v, state) = self.get_tree_rplus(); }
    if !state { (v, state) = self.get_tree_rplusl(); }
    if !state { (v, state) = self.get_tree_rplusr(); }

    if !state { (v, state) = self.get_tree_rtimes(); }
    if !state { (v, state) = self.get_tree_rtimesl(); }
    if !state { (v, state) = self.get_tree_rtimesr(); }

    if !state { (v, state) = self.get_tree_drplus(); }
    if !state { (v, state) = self.get_tree_drplusl(); }
    if !state { (v, state) = self.get_tree_drplusr(); }

    if !state { (v, state) = self.get_tree_drtimes(); }
    if !state { (v, state) = self.get_tree_drtimesl(); }
    if !state { (v, state) = self.get_tree_drtimesr(); }

    if !state { (v, state) = self.get_tree_mrzero(); }
    if !state { (v, state) = self.get_tree_mrone(); }
    if !state { (v, state) = self.get_tree_mrmulti(); }

    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::PZero)).captures_iter(&self.obj).next() {
      if cap[1] == cap[2] {
        v = Some(RuleTree{
          obj: Object::ReduceNatExp(Rule::PZero),
          val: self.obj.clone(),
          node: None
        });
        state = true;
      }
    }

    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::PSucc)).captures_iter(&self.obj).next() {
      let val = format!("{} plus {} is {}", &cap[1], &cap[2], &cap[3]);
      let tp = ReduceNatExp{obj: val}.solver();
      if let Some(c) = tp {
        v = Some(RuleTree{
          obj: Object::ReduceNatExp(Rule::PSucc),
          val: self.obj.clone(),
          node: Some(vec!{c})
        });
        state = true;
      }
    }

    if let Some(_) = self.get_regex(Object::ReduceNatExp(Rule::TZero)).captures_iter(&self.obj).next() {
      v = Some(RuleTree{
        obj: Object::ReduceNatExp(Rule::TZero),
        val: self.obj.clone(),
        node: None
      });
      state = true;
    }

    if let Some(cap) = self.get_regex(Object::ReduceNatExp(Rule::TSucc)).captures_iter(&self.obj).next() {
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
        let c = ReduceNatExp{obj: val}.solver();
        if let Some(lhs) = c {
          tp.push(lhs);
          let val = format!("{} plus {} is {}", &cap[2], &self.get_string(mulval), &cap[3]);
          let c = ReduceNatExp{obj: val}.solver();
          if let Some(rhs) = c {
            tp.push(rhs);
            v = Some(RuleTree{
              obj: Object::ReduceNatExp(Rule::TSucc),
              val: self.obj.clone(),
              node: Some(tp)
            });
            state = true;
          }
        }
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
    if let Object::ReduceNatExp(c) = obj {
      match c {
        Rule::RPlus => Regex::new(r"(.*) \+ (.*) ---> (.*)").unwrap(),
        Rule::RTimes => Regex::new(r"(.*) \* (.*) ---> (.*)").unwrap(),
        Rule::RPlusL => Regex::new(r"(.*) \+ (.*) ---> (.*) \+ (.*)").unwrap(),
        Rule::RPlusR => Regex::new(r"(.*) \+ (.*) ---> (.*) \+ (.*)").unwrap(),
        Rule::RTimesL => Regex::new(r"(.*) \* (.*) ---> (.*) \* (.*)").unwrap(),
        Rule::RTimesR => Regex::new(r"(.*) \* (.*) ---> (.*) \* (.*)").unwrap(),
        Rule::DRPlus => Regex::new(r"(.*) \+ (.*) -d-> (.*)").unwrap(),
        Rule::DRTimes => Regex::new(r"(.*) \* (.*) -d-> (.*)").unwrap(),
        Rule::DRPlusL => Regex::new(r"(.*) \+ (.*) -d-> (.*) \+ (.*)").unwrap(),
        Rule::DRPlusR => Regex::new(r"(.*) \+ (.*) -d-> (.*) \+ (.*)").unwrap(),
        Rule::DRTimesL => Regex::new(r"(.*) \* (.*) -d-> (.*) \* (.*)").unwrap(),
        Rule::DRTimesR => Regex::new(r"(.*) \* (.*) -d-> (.*) \* (.*)").unwrap(),
        Rule::MRZero => Regex::new(r"(.*) -\*-> (.*)").unwrap(),
        Rule::MRMulti => Regex::new(r"(.*) -\*-> (.*)").unwrap(),
        Rule::MROne => Regex::new(r"(.*) -\*-> (.*)").unwrap(),
        Rule::EParen => Regex::new(r"^\((.*)\) -.-> (.*)").unwrap(),
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
