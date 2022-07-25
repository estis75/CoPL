use super::{EvalML1, Rule};
use crate::solver::{Solver, RuleTree, Object};
use regex::Regex;

impl EvalML1 {
  fn parse_int(&self, val: String) -> Result<i64, std::num::ParseIntError> {
    val.parse()
  }

  fn parse_bool(&self, val: String) -> Result<bool, String> {
    if val == String::from("true") { 
      Ok(true)
    }else if val == String::from("false"){
      Ok(false)
    }else{
      Err(format!("this is not bool value: {}", val))
    }
  }

  fn is_value(&self, val: String) -> bool {
    self.parse_int(val.clone()).is_ok() || self.parse_bool(val).is_ok()
  }

  fn unwrap_if_parened(&self, exp: String) -> String {
    if let Some(cap) = Regex::new(r"^\((.*)\)$").unwrap().captures_iter(&exp).next() {
      format!("{}", &cap[1])
    }else{
      exp
    }
  }

  pub fn get_tree_eint(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalML1(Rule::EInt)).captures_iter(&self.obj).next() {
      let i1 = self.parse_int(cap[1].to_string());
      let i2 = cap[2].to_string();
      if i1.is_ok() {
        let i2 = if i2 == String::from("?") {
          Ok(i1.clone().unwrap())
        }else {
          self.parse_int(i2)
        };

        if i2.is_ok() {
          let i1 = i1.ok().unwrap();
          let i2 = i2.ok().unwrap();
          if i1 == i2 {
            v = Some(RuleTree{
              obj: Object::EvalML1(Rule::EInt),
              val: format!("{} evalto {}", i1, i2),
              node: None
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_ebool(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalML1(Rule::EBool)).captures_iter(&self.obj).next() {
      let i1 = self.parse_bool(cap[1].to_string());
      let i2 = cap[2].to_string();
      if i1.is_ok() {
        let i2 = if i2 == String::from("?") {
          Ok(i1.clone().unwrap())
        }else {
          self.parse_bool(i2)
        };

        if i2.is_ok(){
          let i1 = i1.ok().unwrap();
          let i2 = i2.ok().unwrap();
          if i1 == i2 {
            v = Some(RuleTree{
              obj: Object::EvalML1(Rule::EBool),
              val: format!("{} evalto {}", i1, i2),
              node: None
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_eift(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalML1(Rule::EIfT)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto true", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML1{obj: val}.solver();
      if let Some(c) = c {
        tp.push(c);
        let val = format!("{} evalto {}", self.unwrap_if_parened(cap[2].to_string()), &cap[4]);
        let c = EvalML1{obj: val}.solver();
        if let Some(c) = c {
          tp.push(c);

          v = Some(RuleTree{
            obj: Object::EvalML1(Rule::EIfT),
            val: format!("if {} then {} else {} evalto {}", &cap[1], &cap[2], &cap[3], &cap[2]),
            node: Some(tp)
          });
          state = true;
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_eiff(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalML1(Rule::EIfF)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto false", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML1{obj: val}.solver();
      if let Some(c) = c {
        tp.push(c);
        let val = format!("{} evalto {}", self.unwrap_if_parened(cap[3].to_string()), &cap[4]);
        let c = EvalML1{obj: val}.solver();
        if let Some(c) = c {
          tp.push(c);

          v = Some(RuleTree{
            obj: Object::EvalML1(Rule::EIfF),
            val: format!("if {} then {} else {} evalto {}", &cap[1], &cap[2], &cap[3], &cap[3]),
            node: Some(tp)
          });
          state = true;
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_eplus(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalML1(Rule::EPlus)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML1{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let val = format!("{} evalto ?", self.unwrap_if_parened(cap[2].to_string()));
          let c = EvalML1{obj: val}.solver();
          if let Some(rhs) = c {
            tp.push(rhs.clone());
            if let Some(rhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&rhs.val).next() {
              let lhs = self.parse_int(lhs[1].to_string());
              let rhs = self.parse_int(rhs[1].to_string());
              if lhs.is_ok() && rhs.is_ok() {
                let lhs = lhs.ok().unwrap();
                let rhs = rhs.ok().unwrap();
                dbg!(&cap);
                let val = if cap[3].to_string() == String::from("?") {
                  lhs + rhs
                }else{
                  self.parse_int(cap[3].to_string()).ok().unwrap()
                };
                dbg!(&lhs, &rhs, &val);
                if lhs + rhs == val {
                  let tpval = format!("{} plus {} is {}", lhs, rhs, val);
                  let c = EvalML1{obj: tpval}.solver();
                  if let Some(c) = c {
                    tp.push(c);

                    v = Some(RuleTree{
                      obj: Object::EvalML1(Rule::EPlus),
                      val: format!("{} + {} evalto {}", &cap[1], &cap[2], val),
                      node: Some(tp)
                    });
                    state = true;
                  }
                }else{
                  // panic!("{} + {} = {}", lhs, rhs, val)
                }
              }
            }
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_eminus(&self) -> (Option<RuleTree>, bool) {
    // check: 値がマイナスになるパターンは大丈夫なんか？
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalML1(Rule::EMinus)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML1{obj: val.clone()}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let val = format!("{} evalto ?", self.unwrap_if_parened(cap[2].to_string()));
          let c = EvalML1{obj: val}.solver();
          if let Some(rhs) = c {
            tp.push(rhs.clone());
            if let Some(rhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&rhs.val).next() {
              let lhs = self.parse_int(lhs[1].to_string()).ok().unwrap();
              let rhs = self.parse_int(rhs[1].to_string()).ok().unwrap();
              let val = if cap[3].to_string() == String::from("?") {
                lhs - rhs
              }else{
                self.parse_int(cap[3].to_string()).ok().unwrap()
              };
              if lhs - rhs == val {
                let tpval = format!("{} minus {} is {}", lhs, rhs, val);
                let c = EvalML1{obj: tpval}.solver();
                if let Some(c) = c {
                  tp.push(c);

                  v = Some(RuleTree{
                    obj: Object::EvalML1(Rule::EMinus),
                    val: format!("{} - {} evalto {}", &cap[1], &cap[2], val),
                    node: Some(tp)
                  });
                  state = true;
                }
              }else{
                // panic!("{} - {} = {}", lhs, rhs, val)
              }
            }
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_etimes(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalML1(Rule::ETimes)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML1{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let val = format!("{} evalto ?", self.unwrap_if_parened(cap[2].to_string()));
          let c = EvalML1{obj: val}.solver();
          if let Some(rhs) = c {
            tp.push(rhs.clone());
            if let Some(rhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&rhs.val).next() {
              let lhs = self.parse_int(lhs[1].to_string());
              let rhs = self.parse_int(rhs[1].to_string());
              if lhs.is_ok() && rhs.is_ok() {
                let lhs = lhs.ok().unwrap();
                let rhs = rhs.ok().unwrap();
                let val = if cap[3].to_string() == String::from("?") {
                  lhs * rhs
                }else{
                  self.parse_int(cap[3].to_string()).ok().unwrap()
                };
                if lhs * rhs == val {
                  let tpval = format!("{} times {} is {}", lhs, rhs, val);
                  let c = EvalML1{obj: tpval}.solver();
                  if let Some(c) = c {
                    tp.push(c);

                    v = Some(RuleTree{
                      obj: Object::EvalML1(Rule::ETimes),
                      val: format!("{} * {} evalto {}", &cap[1], &cap[2], val),
                      node: Some(tp)
                    });
                    state = true;
                  }
                }else{
                  // panic!("{} * {} = {}", lhs, rhs, val)
                }
              }
            }
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_elt(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalML1(Rule::ELt)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", &cap[1]);
      let c = EvalML1{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let val = format!("{} evalto ?", &cap[2]);
          let c = EvalML1{obj: val}.solver();
          if let Some(rhs) = c {
            tp.push(rhs.clone());
            if let Some(rhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&rhs.val).next() {
              let lhs = self.parse_int(lhs[1].to_string()).ok().unwrap();
              let rhs = self.parse_int(rhs[1].to_string()).ok().unwrap();
              let val = if cap[3].to_string() == String::from("?") {
                lhs < rhs
              }else{
                self.parse_bool(cap[3].to_string()).ok().unwrap()
              };
              if (lhs < rhs) == val {
                let tpval = format!("{} less than {} is {}", lhs, rhs, val);
                let c = EvalML1{obj: tpval}.solver();
                if let Some(c) = c {
                  tp.push(c);

                  v = Some(RuleTree{
                    obj: Object::EvalML1(Rule::ELt),
                    val: format!("{} < {} evalto {}", &cap[1], &cap[2], val),
                    node: Some(tp)
                  });
                  state = true;
                }
              }
            }
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_bplus(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalML1(Rule::BPlus)).captures_iter(&self.obj).next() {
      let i1 = self.parse_int(cap[1].to_string());
      let i2 = self.parse_int(cap[2].to_string());
      let i3 = cap[3].to_string();
      if i1.is_ok() && i2.is_ok() {
        let i3 = if i3 == String::from("?") {
          Ok(i1.clone().unwrap() + i2.clone().unwrap())
        }else {
          self.parse_int(i3)
        };


        if i3.is_ok(){
          let i1 = i1.ok().unwrap();
          let i2 = i2.ok().unwrap();
          let i3 = i3.ok().unwrap();

          if i1 + i2 == i3 {
            v = Some(RuleTree{
              obj: Object::EvalML1(Rule::BPlus),
              val: format!("{} plus {} is {}", i1, i2, i3),
              node: None
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_bminus(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalML1(Rule::BMinus)).captures_iter(&self.obj).next() {
      let i1 = self.parse_int(cap[1].to_string());
      let i2 = self.parse_int(cap[2].to_string());
      let i3 = cap[3].to_string();
      if i1.is_ok() && i2.is_ok() {
        let i3 = if i3 == String::from("?") {
          Ok(i1.clone().unwrap() - i2.clone().unwrap())
        }else {
          self.parse_int(i3)
        };

        if i3.is_ok(){
          let i1 = i1.ok().unwrap();
          let i2 = i2.ok().unwrap();
          let i3 = i3.ok().unwrap();
          if i1 - i2 == i3 {
            v = Some(RuleTree{
              obj: Object::EvalML1(Rule::BMinus),
              val: format!("{} minus {} is {}", i1, i2, i3),
              node: None
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_btimes(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalML1(Rule::BTimes)).captures_iter(&self.obj).next() {
      let i1 = self.parse_int(cap[1].to_string());
      let i2 = self.parse_int(cap[2].to_string());
      let i3 = cap[3].to_string();
      if i1.is_ok() && i2.is_ok() {
        let i3 = if i3 == String::from("?") {
          Ok(i1.clone().unwrap() * i2.clone().unwrap())
        }else {
          self.parse_int(i3)
        };

        if i3.is_ok(){
          let i1 = i1.ok().unwrap();
          let i2 = i2.ok().unwrap();
          let i3 = i3.ok().unwrap();
          if i1 * i2 == i3 {
            v = Some(RuleTree{
              obj: Object::EvalML1(Rule::BTimes),
              val: format!("{} times {} is {}", i1, i2, i3),
              node: None
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_blt(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalML1(Rule::BLt)).captures_iter(&self.obj).next() {
      let i1 = self.parse_int(cap[1].to_string());
      let i2 = self.parse_int(cap[2].to_string());
      let i3 = cap[3].to_string();
      if i1.is_ok() && i2.is_ok() {
        let i3 = if i3 == String::from("?") {
          Ok(i1.clone().unwrap() < i2.clone().unwrap())
        }else {
          self.parse_bool(i3)
        };

        if i3.is_ok() {
          let i1 = i1.ok().unwrap();
          let i2 = i2.ok().unwrap();
          let i3 = i3.ok().unwrap();
          if (i1 < i2) == i3 {
            v = Some(RuleTree{
              obj: Object::EvalML1(Rule::BLt),
              val: format!("{} less than {} is {}", i1, i2, i3),
              node: None
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }
}