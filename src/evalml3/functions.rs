use super::{EvalML3, Rule, Value};
use crate::solver::{Solver, RuleTree, Object};
use regex::Regex;

impl EvalML3 {
  fn token_parser(input: &str) -> Result<Vec<String>, ()> {
    let mut input_iter = input.chars().peekable();
    let mut ret = Vec::new();

    while let Some(_) = input_iter.peek() {
      let mut token = String::new();
      let mut parens = 0;
      while let Some(c) = input_iter.next() {
        if c == '(' {
          parens += 1;
          token.push(c);
        }else if c == ')'{
          parens -= 1;
          token.push(c);
        }else if parens == 0 && c == ' ' {
          break;
        }else{
          token.push(c);
        }
      }
      
      if token == "+" || token == "-" || token == "*" || token == "<" || parens != 0 {
        ret.clear();
        break;
      }else{
        ret.push(token.trim().to_string());
      }
    }

    if ret.len() >= 2 {
      if ret[0] == "let" || ret[0] == "fun" {
        Err(())
      }else{
        Ok(ret)
      }
    }else{
      Err(())
    }
  }
  // Rule::EFun =>  Regex::new(r"(.*)\|- fun (.*?) -> (.*) evalto (.*)").unwrap(),
  pub fn get_tree_efun(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML3(Rule::EFun);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      if &self.unwrap_if_parened(cap[4].to_string()) == "?" {
        v = Some(RuleTree{
          obj: OBJ,
          val: format!("{} |- fun {} -> {} evalto ({})[fun {} -> {}]", &cap[1].trim(), &cap[2], &cap[3], &cap[1].trim(), &cap[2], &cap[3]),
          node: None
        });
        state = true;
      }else if let Some(tmpc) = Regex::new(r"\((.*)\)\[fun (.*) -> (.*)\]").unwrap().captures_iter(&self.unwrap_if_parened(cap[4].to_string())).next() {
        let lhs = self.get_env(cap[1].to_string());
        let rhs = self.get_env(tmpc[1].to_string());
        if lhs.len() == rhs.len() 
          && lhs.iter().zip(rhs.iter()).map(|(lhs, rhs)| lhs == rhs).fold(true, |lhs, rhs| lhs && rhs) 
          && cap[2] == tmpc[2] 
          && cap[3] == tmpc[3] {
          v = Some(RuleTree{
            obj: OBJ,
            val: format!("{} |- fun {} -> {} evalto ({})[fun {} -> {}]", &cap[1].trim(), &cap[2], &cap[3], &cap[1].trim(), &cap[2], &cap[3]),
            node: None
          });
          state = true;
        }
      }
    }
    (v, state)
  }

  // Rule::EApp =>  Regex::new(r"(.*)\|- (.*) evalto (.*)").unwrap(),
  pub fn get_tree_eapp(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML3(Rule::EApp);
    let mut v = None;
    let mut state = false;
    let mut tp = Vec::with_capacity(3);

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {

      if let Ok(tokens) = Self::token_parser(&cap[2]) {
        let bottom_token = tokens[tokens.len()-1].clone();
        let top_tokens = tokens[0..tokens.len()-1].into_iter().fold(String::new(), |lhs, rhs| lhs + " " + rhs);
        let tops = top_tokens.trim();
        let val = format!("{} |- {} evalto ?", &cap[1].trim(), self.unwrap_if_parened(tops.to_string()));
        let c = EvalML3{obj: val}.solver();
        if let Some(c) = c {
          tp.push(c.clone());
          if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
            let e1 = self.parse_func(&tmpc[1]);
            if e1.is_ok() {
              let val = format!("{} |- {} evalto ?", &cap[1].trim(), self.unwrap_if_parened(bottom_token));
              let c = EvalML3{obj: val}.solver();
              if let Some(c) = c {
                tp.push(c.clone());
                if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
                  let e1 = e1.ok().unwrap();
                  let e2 = self.parse_value(tmpc[1].to_string());
                  if e2.is_ok() {
                    // let e2 = e2.clone().ok().unwrap();
                    let mut vec = e1.0;
                    vec.push((e1.1 ,e2.ok().unwrap()));
                    let val = format!("{} |- {} evalto {}", self.format_vectored_env(&vec), &e1.2, &cap[3]);
                    let c = EvalML3{obj: val}.solver();
                    if let Some(c) = c {
                      tp.push(c.clone());
                      if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
                        if &self.unwrap_if_parened(cap[3].to_string()) == "?" {
                          let val = format!("{} |- {} evalto {}", &cap[1].trim(), &cap[2].trim(), &tmpc[1]);
                          v = Some(RuleTree{
                            obj: OBJ,
                            val,
                            node: Some(tp)
                          });
                          state = true;
                        }else if let Some(tmpc) = Regex::new(r"\((.*)\)\[fun (.*) -> (.*)\]").unwrap().captures_iter(&self.unwrap_if_parened(cap[3].to_string())).next() {
                          let lhs = self.get_env(cap[1].to_string());
                          let rhs = self.get_env(tmpc[1].to_string());
                          if lhs.len() == rhs.len() 
                           && lhs.iter().zip(rhs.iter()).map(|(lhs, rhs)| lhs == rhs).fold(true, |lhs, rhs| lhs && rhs) 
                           && cap[2] == tmpc[2] 
                           && cap[3] == tmpc[3] {
                            let val = format!("{} |- {} evalto {}", &cap[1].trim(), cap[2].trim().to_string(), &cap[3].trim());
                            v = Some(RuleTree{
                              obj: OBJ,
                              val,
                              node: Some(tp)
                            });
                            state = true;
                          }
                        }else if self.unwrap_if_parened(cap[3].to_string()) == tmpc[1] {
                          let val = format!("{} |- {} evalto {}", &cap[1].trim(), &cap[2].trim(), &cap[3].trim());
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
          }

        }
      }
    }
    (v, state)
  }

  // Rule::ELetRec =>  Regex::new(r"(.*)\|- let rec (.*?) = fun (.*?) -> (.*) in (.*) evalto (.*)").unwrap(),
  pub fn get_tree_eletrec(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML3(Rule::ELetRec);

    let mut v = None;
    let mut tp = Vec::with_capacity(1);
    let mut state = false;

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut env = self.get_env(cap[1].to_string());
      env.push((cap[2].to_string(), Value::RecFunction((env.clone(), cap[2].to_string(), cap[3].to_string(), cap[4].to_string()))));
      let val = format!("{} |- {} evalto {}", self.format_vectored_env(&env), &cap[5], &cap[6]);
      let c = EvalML3{obj: val}.solver();
      if let Some(c) = c {
        tp.push(c.clone());
        if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
          if &self.unwrap_if_parened(cap[3].to_string()) == "?" {
            let val = format!("{} |- let rec {} = fun {} -> {} in {} evalto {}", &cap[1].trim(), &cap[2].trim(), &cap[3].trim(), &cap[4].trim(), &cap[5].trim(), &tmpc[1]);
            v = Some(RuleTree{
              obj: OBJ,
              val,
              node: Some(tp)
            });
            state = true;
          }else if tmpc[1].trim() == cap[6].trim() {
            let val = format!("{} |- let rec {} = fun {} -> {} in {} evalto {}", &cap[1].trim(), &cap[2].trim(), &cap[3].trim(), &cap[4].trim(), &cap[5].trim(), &cap[6]);
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
    (v, state)
  }

  // Rule::EAppRec =>  Regex::new(r"(.*)\|- (.*) evalto (.*)").unwrap(),
  pub fn get_tree_eapprec(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML3(Rule::EAppRec);

    let mut v = None;
    let mut tp = Vec::with_capacity(3);
    let mut state = false;

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      if let Ok(tokens) = Self::token_parser(&cap[2]) {
        let bottom_token = tokens[tokens.len()-1].clone();
        let top_tokens = tokens[0..tokens.len()-1].into_iter().fold(String::new(), |lhs, rhs| lhs + " " + rhs);
        let tops = top_tokens.trim();
        let val = format!("{} |- {} evalto ?", &cap[1].trim(), self.unwrap_if_parened(tops.to_string()));

        let c = EvalML3{obj: val}.solver();
        if let Some(c) = c {
          tp.push(c.clone());
          if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
            let e1 = self.parse_recfunc(&tmpc[1]);
            if e1.is_ok() {
              let val = format!("{} |- {} evalto ?", &cap[1].trim(), self.unwrap_if_parened(bottom_token));
              let c = EvalML3{obj: val}.solver();
              if let Some(c) = c {
                tp.push(c.clone());
                if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
                  let e1 = e1.ok().unwrap();
                  let e2 = self.parse_value(tmpc[1].to_string());
                  if e2.is_ok() {
                    // let e2 = e2.clone().ok().unwrap();
                    let mut vec = e1.0;
                    vec.push((e1.1.clone(), Value::RecFunction((vec.clone(), e1.1, e1.2.clone(), e1.3.clone()))));
                    vec.push((e1.2, e2.ok().unwrap()));

                    let val = format!("{} |- {} evalto {}", self.format_vectored_env(&vec), &e1.3, &cap[3]);
                    let c = EvalML3{obj: val}.solver();
                    if let Some(c) = c {
                      tp.push(c.clone());
                      if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {

                        if &self.unwrap_if_parened(cap[3].to_string()) == "?" {
                          let val = format!("{} |- {} evalto {}", &cap[1].trim(), &cap[2].trim(), &tmpc[1]);
                          v = Some(RuleTree{
                            obj: OBJ,
                            val,
                            node: Some(tp)
                          });
                          state = true;
                        }else if self.unwrap_if_parened(cap[3].to_string()) == tmpc[1] {
                          let val = format!("{} |- {} evalto {}", &cap[1].trim(), &cap[2].trim(), &cap[3].trim());
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
          }

        }
      }
    }
    (v, state)
  }
}