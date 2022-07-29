use super::solver::{Solver, RuleTree, Object};
use regex::Regex;
use std::fmt::{self, Display, Formatter};
mod derivation;
#[allow(unused)]
use derivation::*;
mod functions;
#[allow(unused)]
use functions::*;

pub struct NamelessML3 {
  pub obj: String
}

#[derive(Debug, Clone)]
pub enum Rule{
  TrInt,
  TrBool,
  TrIf,
  TrPlus,
  TrMinus,
  TrTimes,
  TrLt,
  TrVar1,
  TrVar2,
  TrLet,
  TrFun,
  TrApp,
  TrLetRec,
}
impl Display for Rule {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::TrInt => write!(f, "Tr-Int"),
      Self::TrBool => write!(f, "Tr-Bool"),
      Self::TrIf => write!(f, "Tr-If"),
      Self::TrFun => write!(f, "Tr-Fun"),
      Self::TrApp => write!(f, "Tr-App"),
      Self::TrLetRec => write!(f, "Tr-LetRec"),
      Self::TrPlus => write!(f, "Tr-Plus"),
      Self::TrMinus => write!(f, "Tr-Minus"),
      Self::TrTimes => write!(f, "Tr-Times"),
      Self::TrLt => write!(f, "Tr-Lt"),
      Self::TrLet => write!(f, "Tr-Let"),
      Self::TrVar1 => write!(f, "Tr-Var1"),
      Self::TrVar2 => write!(f, "Tr-Var2"),
    }
  }
}

impl NamelessML3 {
  #[allow(unused)]
  pub fn solve(&self) -> Option<RuleTree> {
    self.solver()
  }
}

impl Solver for NamelessML3 {
  fn solver(&self) -> Option<RuleTree> {
    let mut v = None;
    let mut state = false;
    dbg!(&self.obj);

    if !state { (v, state) = self.get_tree_trint(); };
    if !state { (v, state) = self.get_tree_trbool(); };

    if !state { (v, state) = self.get_tree_trlet(); };
    if !state { (v, state) = self.get_tree_trfun(); };
    if !state { (v, state) = self.get_tree_trapp(); };
    if !state { (v, state) = self.get_tree_trletrec(); };

    if !state { (v, state) = self.get_tree_trif(); };

    if !state { (v, state) = self.get_tree_trplus()}
    if !state { (v, state) = self.get_tree_trminus(); }
    if !state { (v, state) = self.get_tree_trtimes(); }
    if !state { (v, state) = self.get_tree_trlt(); }

    if !state { (v, state) = self.get_tree_trvar1(); }
    if !state { (v, state) = self.get_tree_trvar2(); }

    if state {
      v
    }else{
      // panic!("nothing is matched: {}", self.obj);
      None
    }
  }

  fn get_regex(&self, obj: Object) -> Regex{
    if let Object::NamelessML3(c) = obj {
      match c {
        Rule::TrInt => Regex::new(r"(.*)\|- (.*) ==> (.*)").unwrap(),
        Rule::TrBool => Regex::new(r"(.*)\|- (.*) ==> (.*)").unwrap(),
        Rule::TrVar1 => Regex::new(r"(.*)\|- (.*) ==> #(.*)").unwrap(),
        Rule::TrVar2 => Regex::new(r"(.*)\|- (.*) ==> #(.*)").unwrap(),
        Rule::TrIf => Regex::new(r"(.*)\|- if (.*) then (.*) else (.*) ==> if (.*) then (.*) else (.*)").unwrap(),
        Rule::TrFun => Regex::new(r"(.*)\|- fun (.*?) -> (.*) ==> fun \. -> (.*)").unwrap(),
        Rule::TrLetRec => Regex::new(r"(.*)\|- let rec (.*?) = fun (.*?) -> (.*) in (.*) ==> let rec \. = fun \. -> (.*) in (.*)").unwrap(),
        Rule::TrApp => Regex::new(r"(.*)\|- (.*) ==> (.*)").unwrap(),
        Rule::TrPlus => Regex::new(r"(.*)\|- (.*) \+ (.*) ==> (.*) \+ (.*)").unwrap(),
        Rule::TrMinus => Regex::new(r"(.*)\|- (.*) - (.*) ==> (.*) - (.*)").unwrap(),
        Rule::TrTimes => Regex::new(r"(.*)\|- (.*) \* (.*) ==> (.*) \* (.*)").unwrap(),
        Rule::TrLt => Regex::new(r"(.*)\|- (.*) < (.*) ==> (.*) < (.*)").unwrap(),
        Rule::TrLet => Regex::new(r"(.*)\|- let (.*?) = (.*) in (.*) ==> let \. = (.*) in (.*)").unwrap(),
      }
    }else{
      panic!("invalid object type: {:?}", obj)
    }
  }
}
