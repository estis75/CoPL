use super::{EvalML3, Rule};
use crate::solver::{Solver, RuleTree, Object};
use regex::Regex;
use std::collections::BTreeMap;
use std::fmt::{self, Display, Formatter};

impl EvalML3 {
  // これを実装する
  // Rule::EFun =>  Regex::new(r"(.*)\|- fun (.*?) -> (.*) evalto (.*)").unwrap(),
  pub fn get_tree_efun(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalML3(Rule::EFun);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      // dbg!(&cap);
      if &self.unwrap_if_parened(cap[4].to_string()) == "?" {
        v = Some(RuleTree{
          obj: OBJ,
          val: format!("{} |- fun {} -> {} evalto ({})[fun {} -> {}]", &cap[1], &cap[2], &cap[3], &cap[1], &cap[2], &cap[3]),
          node: None
        });
        state = true;
      }else if let Some(tmpc) = Regex::new(r"\((.*)\)\[fun (.*) -> (.*)\]").unwrap().captures_iter(&self.unwrap_if_parened(cap[4].to_string())).next() {
        let lhs = self.get_env(cap[1].to_string());
        let rhs = self.get_env(tmpc[1].to_string());
        dbg!(&lhs, &rhs);
        dbg!(&cap, &tmpc);
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
}