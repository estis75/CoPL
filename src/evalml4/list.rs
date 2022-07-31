use super::{EvalML4, Rule, Value};
use crate::solver::{Solver, RuleTree, Object};
use regex::Regex;
use std::boxed::Box;

impl EvalML4 {

  pub fn get_tree_enil(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML4(Rule::ENil);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      if &cap[2] == "?" {
        v = Some(RuleTree{
          obj: OBJ,
          val: format!("{} |- [] evalto []", &cap[1]),
          node: None
        });
        state = true;
      }else if &cap[2] == "[]" {
        v = Some(RuleTree{
          obj: OBJ,
          val: format!("{} |- [] evalto []", &cap[1]),
          node: None
        });
        state = true;
      }
    }
    (v, state)
  }

  fn asserting_object(input: &str) -> bool {
    let mut input_iter = input.chars().peekable();

    let mut parens = 0;
    let mut sqparens = 0;
    while let Some(c) = input_iter.next() {
      if c == '(' {
        parens += 1;
      }else if c == ')'{
        parens -= 1;
      }else if c == '[' {
        sqparens += 1;
      }else if c == ']' {
        sqparens -= 1;
      }
    }
    sqparens == 0 && parens == 0
  }

  pub fn get_tree_econs(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML4(Rule::ECons);
    let mut v = None;
    let mut state = false;
    let mut tp = Vec::with_capacity(3);

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let env = self.get_env(cap[1].to_string());

      if Self::asserting_object(&cap[2]) && (cap[2].contains("::") || cap[2].contains("::")) && !cap[2].contains("if") && !cap[2].contains("else") {

        if let Ok(lhs) = self.parse_list(&(cap[2].to_string() + " :: []")) {

          if &cap[3] == "?" {
            let mut lhs = lhs.into_iter();
            if let Some(lval) = lhs.next() {
              let val = if let Value::Parentheses(p) = lval.clone() {
                format!("{} |- {} evalto ?", self.format_vectored_env(&env), p.to_string())
              }else{
                format!("{} |- {} evalto ?", self.format_vectored_env(&env), lval.to_string())
              };
              let c = EvalML4{obj: val}.solver();
              if let Some(c) = c {
                tp.push(c.clone());
                if let Some(vl) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
                  let st = format!("{}", Value::List(lhs.collect::<Vec<_>>()));
                  if st.len() > 5 {
                    let val = format!("{} |- {} evalto ?", self.format_vectored_env(&env), &st[..st.len()-5].trim());
                    let c = EvalML4{obj: val}.solver();
                    if let Some(c) = c {
                      if let Some(vr) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
                        tp.push(c.clone());
                        let vrs = self.parse_list(&vr[1]);
                        if vrs.is_ok() {
                          let mut vrs = vrs.ok().unwrap();
                          let vl = self.parse_value(vl[1].to_string());
                          if vl.is_ok() {
                            let vl = vl.ok().unwrap();
                            if let Value::Parentheses(_) = lval {
                              vrs.insert(0, Value::Parentheses(Box::new(vl)));
                            }else{
                              if let Value::List(l) = vl {
                                vrs = l.into_iter().chain(vrs.into_iter()).collect::<Vec::<Value>>();
                              }else{
                                vrs.insert(0, vl);
                              }
                            }
                            v = Some(RuleTree{
                              obj: OBJ,
                              val: format!("{} |- {} evalto {}", self.format_vectored_env(&env), &cap[2], Value::List(vrs)),
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
            }
          } else if let Ok(rhs) = self.parse_list(&cap[3]) {
            if lhs.len() != 0 && rhs.len() != 0 {
              let mut lhs = lhs.into_iter();
              let val = format!("{} |- {} evalto ?", self.format_vectored_env(&env), self.unwrap_if_parened(lhs.next().unwrap().to_string()));
              let c = EvalML4{obj: val}.solver();
              if let Some(c) = c {
                tp.push(c.clone());
                let st = format!("{}", Value::List(lhs.collect::<Vec<_>>()));
                if st.len() > 5 {
                  let val = format!("{} |- {} evalto ?", self.format_vectored_env(&env), &st[..st.len()-5]);
                  let c = EvalML4{obj: val.clone()}.solver();
                  if let Some(c) = c {
                    tp.push(c.clone());
                    v = Some(RuleTree{
                      obj: OBJ,
                      val: format!("{} |- {} evalto {}", self.format_vectored_env(&env), &cap[2], &cap[3]),
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
    }
    (v, state)
  }

  pub fn get_tree_ematchnil(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML4(Rule::EMatchNil);
    let mut v = None;
    let mut state = false;
    let mut tp = Vec::with_capacity(3);

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let env = self.get_env(cap[1].to_string());

      let val = format!("{} |- {} evalto ?", self.format_vectored_env(&env), self.unwrap_if_parened(cap[2].to_string()));
      let c = EvalML4{obj: val}.solver();
      if let Some(c) = c {
        tp.push(c.clone());
        if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
          let e1 = self.parse_list(&tmpc[1]);
          if e1.is_ok() {
            if e1.ok().unwrap().len() == 0 {
              let val = format!("{} |- {} evalto {}", self.format_vectored_env(&env), self.unwrap_if_parened(cap[3].to_string()), &cap[6]);
              let c = EvalML4{obj: val}.solver();
              if let Some(c) = c {
                tp.push(c.clone());
                if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
                  let val = format!("{} |- match {} with [] -> {} | {} -> {} evalto {}", self.format_vectored_env(&env), &cap[2].trim(), &cap[3].trim(), &cap[4].trim(), &cap[5].trim(), &tmpc[1]);
                  v = Some(RuleTree{
                    obj: OBJ,
                    val,
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

  pub fn get_tree_ematchcons(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML4(Rule::EMatchCons);
    let mut v = None;
    let mut state = false;
    let mut tp = Vec::with_capacity(3);

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let env = self.get_env(cap[1].to_string());

      let val = format!("{} |- {} evalto ?", self.format_vectored_env(&env), self.unwrap_if_parened(cap[2].to_string()));
      let c = EvalML4{obj: val}.solver();
      if let Some(c) = c {
        tp.push(c.clone());
        if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
          let vals = self.parse_list(&(self.unwrap_if_parened(cap[4].to_string()).to_string() + " :: []"));
          let vs = self.parse_list(&tmpc[1]);
          if vals.is_ok() && vs.is_ok(){
            let mut vals = vals.unwrap().into_iter();
            let mut vs = vs.unwrap().into_iter();
            let mut added_env = env.clone();
            let x = vs.next().unwrap();
            added_env.push((vals.next().unwrap().to_string(), if let Value::Parentheses(cx) = x {*cx} else {x} ));
            added_env.push((vals.next().unwrap().to_string(), Value::List(vs.collect())));
            let val = format!("{} |- {} evalto {}", self.format_vectored_env(&added_env), self.unwrap_if_parened(cap[5].to_string()), &cap[6]);
            let c = EvalML4{obj: val}.solver();
            if let Some(c) = c {
              tp.push(c.clone());
              if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
                let val = format!("{} |- match {} with [] -> {} | {} -> {} evalto {}", self.format_vectored_env(&env), &cap[2].trim(), &cap[3].trim(), &cap[4].trim(), &cap[5].trim(), &tmpc[1]);
                v = Some(RuleTree{
                  obj: OBJ,
                  val: val,
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