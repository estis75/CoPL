use super::{NamelessML3, Rule, Value};
use crate::solver::{Solver, RuleTree, Object};

impl NamelessML3 {
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
  // Rule::TrFun =>  Regex::new(r"(.*)\|- fun (.*?) -> (.*) ==> fun \. -> (.*)").unwrap(),
  pub fn get_tree_trfun(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::NamelessML3(Rule::TrFun);
    let mut v = None;
    let mut state = false;

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(1);
      let original_env = self.get_env(cap[1].to_string());

      let mut env = original_env.clone();
      env.push((cap[2].to_string(), Value::Variable()));
      let val = format!("{} |- {} ==> {}", self.format_vectored_env(&env), self.unwrap_if_parened(cap[3].to_string()), self.unwrap_if_parened(cap[4].to_string()));
      let c = NamelessML3{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        v = Some(RuleTree{
          obj: OBJ,
          val: format!("{} |- fun {} -> {} ==> fun . -> {}", self.format_vectored_env(&original_env), &cap[2], &cap[3], &cap[4]),
          node: Some(tp),
        });
        state = true;
      }
    }
    (v, state)
  }

  // Rule::EApp =>  Regex::new(r"(.*)\|- (.*) ==> (.*)").unwrap(),
  pub fn get_tree_trapp(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::NamelessML3(Rule::TrApp);
    let mut v = None;
    let mut state = false;
    let mut tp = Vec::with_capacity(2);

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let env = self.get_env(cap[1].to_string());

      if let Ok(tokens) = Self::token_parser(&cap[2]) {
        let lbottom_token = tokens[tokens.len()-1].clone();
        let ltop_tokens = tokens[0..tokens.len()-1].into_iter().fold(String::new(), |lhs, rhs| lhs + " " + rhs);
        let ltops = ltop_tokens.trim();

        if let Ok(tokens) = Self::token_parser(&cap[3]) {
          let rbottom_token = tokens[tokens.len()-1].clone();
          let rtop_tokens = tokens[0..tokens.len()-1].into_iter().fold(String::new(), |lhs, rhs| lhs + " " + rhs);
          let rtops = rtop_tokens.trim();

          let val = format!("{} |- {} ==> {}", &self.format_vectored_env(&env), self.unwrap_if_parened(ltops.to_string()), self.unwrap_if_parened(rtops.to_string()));
          let c = NamelessML3{obj: val}.solver();
          if let Some(c) = c {
            tp.push(c.clone());
            let val = format!("{} |- {} ==> {}", &self.format_vectored_env(&env), self.unwrap_if_parened(lbottom_token.to_string()), self.unwrap_if_parened(rbottom_token.to_string()));
            let c = NamelessML3{obj: val}.solver();
            if let Some(c) = c {
              tp.push(c.clone());
              let val = format!("{} |- {} ==> {}", &self.format_vectored_env(&env), &cap[2].trim(), &cap[3]);
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
    (v, state)
  }

  // Rule::ELetRec =>  Regex::new(r"(.*)\|- let rec (.*?) = fun (.*?) -> (.*) in (.*) ==> (.*)").unwrap(),
  pub fn get_tree_trletrec(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::NamelessML3(Rule::TrLetRec);

    let mut v = None;
    let mut tp = Vec::with_capacity(1);
    let mut state = false;

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let env = self.get_env(cap[1].to_string());
      let mut lhs_env = env.clone();
      lhs_env.push((cap[2].to_string(), Value::Variable()));
      lhs_env.push((cap[3].to_string(), Value::Variable()));

      let mut rhs_env = env.clone();
      rhs_env.push((cap[2].to_string(), Value::Variable()));

      let val = format!("{} |- {} ==> {}", self.format_vectored_env(&lhs_env), &cap[4], &cap[6]);
      let c = NamelessML3{obj: val}.solver();
      if let Some(c) = c {
        tp.push(c.clone());
        let val = format!("{} |- {} ==> {}", self.format_vectored_env(&rhs_env), &cap[5], &cap[7]);
        let c = NamelessML3{obj: val}.solver();
        if let Some(c) = c {
          tp.push(c.clone());
          let val = format!("{} |- let rec {} = fun {} -> {} in {} ==> let rec . = fun . -> {} in {}", &self.format_vectored_env(&env), &cap[2].trim(), &cap[3].trim(), &cap[4].trim(), &cap[5].trim(), &cap[6].trim(), &cap[7].trim());
          v = Some(RuleTree{
            obj: OBJ,
            val,
            node: Some(tp)
          });
          state = true;
        }
      }
    }
    (v, state)
  }
}