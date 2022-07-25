use super::{EvalML2, Rule};
use crate::solver::{Solver, RuleTree, Object};
use regex::Regex;

impl EvalML2 {
  fn parse_error(&self, val: String) -> Result<(), String> {
    if val == String::from("error") { 
      Ok(())
    }else{
      Err(format!("this is not \"error\" value: {}", val))
    }
  }

  pub fn get_tree_eplusbooll(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::EPlusBoolL);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_bool(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} + {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_eplusboolr(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::EPlusBoolR);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[2].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_bool(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} + {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_epluserrorl(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::EPlusErrorL);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_error(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} + {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_epluserrorr(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::EPlusErrorR);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[2].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_error(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} + {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_eminusbooll(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::EMinusBoolL);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_bool(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} - {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_eminusboolr(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::EMinusBoolR);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[2].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_bool(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} - {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_eminuserrorl(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::EMinusErrorL);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_error(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} - {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_eminuserrorr(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::EMinusErrorR);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[2].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_error(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} - {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_etimesbooll(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::ETimesBoolL);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_bool(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} * {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_etimesboolr(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::ETimesBoolR);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[2].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_bool(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} * {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_etimeserrorl(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::ETimesErrorL);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_error(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} * {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_etimeserrorr(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::ETimesErrorR);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[2].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_error(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} * {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_eltbooll(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::ELtBoolL);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_bool(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} < {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_eltboolr(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::ELtBoolR);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[2].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_bool(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} < {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_elterrorl(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::ELtErrorL);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_error(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} < {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_elterrorr(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::ELtErrorR);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[2].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_error(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} < {} evalto error", &cap[1], &cap[2]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_eifint(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::EIfInt);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_int(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("if {} then {} else {} evalto error", &cap[1], &cap[2], &cap[3]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }
  pub fn get_tree_eiferror(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::EIfError);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let lhs = self.parse_error(lhs[1].to_string());
          if lhs.is_ok() {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("if {} then {} else {} evalto error", &cap[1], &cap[2], &cap[3]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_eifterror(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::EIfTError);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let val = format!("{} evalto ?", self.unwrap_if_parened(cap[2].to_string()));
          let c = EvalML2{obj: val}.solver();
          if let Some(rhs) = c {
            tp.push(rhs.clone());
            if let Some(rhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&rhs.val).next() {
              let lhs = self.parse_bool(lhs[1].to_string());
              let rhs = self.parse_error(rhs[1].to_string());
              if lhs.is_ok() && lhs.ok().unwrap() && rhs.is_ok() {
                v = Some(RuleTree{
                  obj: OBJ,
                  val: format!("if {} then {} else {} evalto error", &cap[1], &cap[2], &cap[3]),
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

  pub fn get_tree_eifferror(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML2(Rule::EIfFError);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} evalto ?", self.unwrap_if_parened(cap[1].to_string()));
      let c = EvalML2{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let val = format!("{} evalto ?", self.unwrap_if_parened(cap[3].to_string()));
          let c = EvalML2{obj: val}.solver();
          if let Some(rhs) = c {
            tp.push(rhs.clone());
            if let Some(rhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&rhs.val).next() {
              let lhs = self.parse_bool(lhs[1].to_string());
              let rhs = self.parse_error(rhs[1].to_string());
              if lhs.is_ok() && !lhs.ok().unwrap() && rhs.is_ok() {
                v = Some(RuleTree{
                  obj: OBJ,
                  val: format!("if {} then {} else {} evalto error", &cap[1], &cap[2], &cap[3]),
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
}