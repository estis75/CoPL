use super::{EvalNamelessML3, Rule, Value};
use crate::solver::{Solver, RuleTree, Object};
use regex::Regex;

impl EvalNamelessML3 {
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
    const OBJ: Object = Object::EvalNamelessML3(Rule::EFun);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      if &self.unwrap_if_parened(cap[3].to_string()) == "?" {
        v = Some(RuleTree{
          obj: OBJ,
          val: format!("{} |- fun . -> {} evalto ({})[fun . -> {}]", &cap[1].trim(), &cap[2], &cap[1].trim(), &cap[2]),
          node: None
        });
        state = true;
      }else if let Some(tmpc) = Regex::new(r"\((.*)\)\[fun \. -> (.*)\]").unwrap().captures_iter(&self.unwrap_if_parened(cap[3].to_string())).next() {
        let lhs = self.get_env(&cap[1]);
        let rhs = self.get_env(&tmpc[1]);
        if lhs.len() == rhs.len() 
          && lhs.iter().zip(rhs.iter()).map(|(lhs, rhs)| lhs == rhs).fold(true, |lhs, rhs| lhs && rhs) 
          && cap[2] == tmpc[2] {
          v = Some(RuleTree{
            obj: OBJ,
            val: format!("{} |- fun . -> {} evalto ({})[fun . -> {}]", &cap[1].trim(), &cap[2], &cap[1].trim(), &cap[2]),
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
    const OBJ: Object = Object::EvalNamelessML3(Rule::EApp);
    let mut v = None;
    let mut state = false;
    let mut tp = Vec::with_capacity(3);

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let env = self.get_env(&cap[1]);

      if let Ok(tokens) = Self::token_parser(&cap[2]) {
        let bottom_token = tokens[tokens.len()-1].clone();
        let top_tokens = tokens[0..tokens.len()-1].into_iter().fold(String::new(), |lhs, rhs| lhs + " " + rhs);
        let tops = top_tokens.trim();
        let val = format!("{} |- {} evalto ?", self.format_vectored_env(&env), self.unwrap_if_parened(tops.to_string()));
        let c = EvalNamelessML3{obj: val}.solver();
        if let Some(c) = c {
          tp.push(c.clone());
          if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
            let e1 = self.parse_func(&tmpc[1]);
            if e1.is_ok() {
              let val = format!("{} |- {} evalto ?", self.format_vectored_env(&env), self.unwrap_if_parened(bottom_token));
              let c = EvalNamelessML3{obj: val}.solver();
              if let Some(c) = c {
                tp.push(c.clone());
                if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
                  let e1 = e1.ok().unwrap();
                  let e2 = self.parse_value(&tmpc[1]);
                  if e2.is_ok() {
                    // let e2 = e2.clone().ok().unwrap();
                    let mut vec = e1.0;
                    vec.push(e2.ok().unwrap());

                    let val = format!("{} |- {} evalto {}", self.format_vectored_env(&vec), &e1.1, &cap[3]);
                    let c = EvalNamelessML3{obj: val}.solver();
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
                        }else if let Some(tmpc) = Regex::new(r"\((.*)\)\[fun \. -> (.*)\]").unwrap().captures_iter(&self.unwrap_if_parened(cap[3].to_string())).next() {
                          let lhs = self.get_env(&cap[1]);
                          let rhs = self.get_env(&tmpc[1]);
                          if lhs.len() == rhs.len() 
                           && lhs.iter().zip(rhs.iter()).map(|(lhs, rhs)| lhs == rhs).fold(true, |lhs, rhs| lhs && rhs) 
                           && cap[2] == tmpc[2] {
                            let val = format!("{} |- {} evalto {}", &self.format_vectored_env(&env), cap[2].trim().to_string(), &cap[3].trim());
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
    const OBJ: Object = Object::EvalNamelessML3(Rule::ELetRec);
    let mut v = None;
    let mut state = false;
    let mut tp = Vec::with_capacity(1);

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let env = self.get_env(&cap[1]);
      let mut tp_env = env.clone();
      tp_env.push(Value::RecFunction((env.clone(), cap[2].to_string())));
      let val = format!("{} |- {} evalto {}", self.format_vectored_env(&tp_env), &cap[3], &cap[4]);
      let c = EvalNamelessML3{obj: val}.solver();
      if let Some(c) = c {
        tp.push(c.clone());
        if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
          if &cap[4] == "?" {
            let val = format!("{} |- let rec . = fun . -> {} in {} evalto {}", self.format_vectored_env(&env), &cap[2].trim(), &cap[3].trim(), &cap[4].trim());
            v = Some(RuleTree{
              obj: OBJ,
              val,
              node: Some(tp)
            });
            state = true;
          }else if tmpc[1].trim() == cap[4].trim() {
            let val = format!("{} |- let rec . = fun . -> {} in {} evalto {}", self.format_vectored_env(&env), &cap[2].trim(), &cap[3].trim(), &cap[4].trim());
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
    const OBJ: Object = Object::EvalNamelessML3(Rule::EAppRec);

    let mut v = None;
    let mut tp = Vec::with_capacity(3);
    let mut state = false;

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let env = self.get_env(&cap[1]);

      if let Ok(tokens) = Self::token_parser(&cap[2]) {
        let bottom_token = tokens[tokens.len()-1].clone();
        let top_tokens = tokens[0..tokens.len()-1].into_iter().fold(String::new(), |lhs, rhs| lhs + " " + rhs);
        let tops = top_tokens.trim();
        let val = format!("{} |- {} evalto ?", &self.format_vectored_env(&env), self.unwrap_if_parened(tops.to_string()));

        let c = EvalNamelessML3{obj: val}.solver();
        if let Some(c) = c {
          tp.push(c.clone());
          if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
            let e1 = self.parse_recfunc(&tmpc[1]);
            if e1.is_ok() {
              let val = format!("{} |- {} evalto ?", &self.format_vectored_env(&env), self.unwrap_if_parened(bottom_token));
              let c = EvalNamelessML3{obj: val}.solver();
              if let Some(c) = c {
                tp.push(c.clone());
                if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
                  let e1 = e1.ok().unwrap();
                  let e2 = self.parse_value(&tmpc[1]);
                  if e2.is_ok() {
                    // let e2 = e2.clone().ok().unwrap();
                    let mut vec = e1.0;
                    vec.push(Value::RecFunction((vec.clone(), e1.1.clone())));
                    vec.push(e2.ok().unwrap());

                    let val = format!("{} |- {} evalto {}", self.format_vectored_env(&vec), &e1.1, &cap[3]);
                    let c = EvalNamelessML3{obj: val}.solver();
                    if let Some(c) = c {
                      tp.push(c.clone());
                      if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {

                        if &self.unwrap_if_parened(cap[3].to_string()) == "?" {
                          let val = format!("{} |- {} evalto {}", self.format_vectored_env(&env), &cap[2].trim(), &tmpc[1]);
                          v = Some(RuleTree{
                            obj: OBJ,
                            val,
                            node: Some(tp)
                          });
                          state = true;
                        }else if self.unwrap_if_parened(cap[3].to_string()) == tmpc[1] {
                          let val = format!("{} |- {} evalto {}", self.format_vectored_env(&env), &cap[2].trim(), &cap[3].trim());
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