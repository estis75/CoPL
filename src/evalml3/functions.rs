use super::{EvalML3, Rule};
use crate::solver::{Solver, RuleTree, Object};
use regex::Regex;

impl EvalML3 {
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

  // これを実装する
  // Rule::EApp =>  Regex::new(r"(.*)\|- (.*) evalto (.*)").unwrap(),
  pub fn get_tree_eapp(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML3(Rule::EApp);
    let mut v = None;
    let mut state = false;
    let mut tp = Vec::with_capacity(3);

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
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
          
          if token == "+" || token == "-" || token == "*" {
            ret.clear();
            break;
          }else{
            ret.push(token);
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

      if let Ok(tokens) = token_parser(&cap[2]) {
        // dbg!(&tokens);
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
                    let val = format!("{} |- {} evalto {}", self.format_vectored_env(vec), &e1.2, &cap[3]);
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
}