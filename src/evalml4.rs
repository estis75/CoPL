use super::solver::{Solver, RuleTree, Object};
use regex::Regex;
use std::fmt::{self, Display, Formatter};
mod derivation;
#[allow(unused)]
use derivation::*;
mod functions;
#[allow(unused)]
use functions::*;
mod list;
#[allow(unused)]
use list::*;

pub struct EvalML4 {
  pub obj: String
}

#[derive(Debug, Clone)]
pub enum Rule{
  EInt,
  EBool,
  EVar,
  EFun,
  EApp,
  ELetRec,
  EAppRec,
  EIfT,
  EIfF,
  EPlus,
  EMinus,
  ETimes,
  ELt,
  ELet,
  ENil,
  ECons,
  EMatchNil,
  EMatchCons,
  BPlus,
  BMinus,
  BTimes,
  BLt,
}
impl Display for Rule {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::EInt => write!(f, "E-Int"),
      Self::EBool => write!(f, "E-Bool"),
      Self::EFun => write!(f, "E-Fun"),
      Self::EApp => write!(f, "E-App"),
      Self::ELetRec => write!(f, "E-LetRec"),
      Self::EAppRec => write!(f, "E-AppRec"),
      Self::EIfT => write!(f, "E-IfT"),
      Self::EIfF => write!(f, "E-IfF"),
      Self::EPlus => write!(f, "E-Plus"),
      Self::EMinus => write!(f, "E-Minus"),
      Self::ETimes => write!(f, "E-Times"),
      Self::ELt => write!(f, "E-Lt"),
      Self::ELet => write!(f, "E-Let"),
      Self::EVar => write!(f, "E-Var"),
      Self::ENil =>  write!(f, "E-Nil"),
      Self::ECons =>  write!(f, "E-Cons"),
      Self::EMatchNil =>  write!(f, "E-MatchNil"),
      Self::EMatchCons =>  write!(f, "E-MatchCons"),
      Self::BPlus => write!(f, "B-Plus"),
      Self::BMinus => write!(f, "B-Minus"),
      Self::BTimes => write!(f, "B-Times"),
      Self::BLt => write!(f, "B-Lt"),
    }
  }
}

impl EvalML4 {
  #[allow(unused)]
  pub fn solve(&self) -> Option<RuleTree> {
    self.solver()
  }
}

impl Solver for EvalML4 {
  fn solver(&self) -> Option<RuleTree> {
    let mut v = None;
    let mut state = false;
    dbg!(&self.obj);

    if !state { (v, state) = self.get_tree_eint(); }
    if !state { (v, state) = self.get_tree_ebool(); }

    if !state { (v, state) = self.get_tree_elet(); }
    if !state { (v, state) = self.get_tree_efun(); }
    if !state { (v, state) = self.get_tree_eapp(); }
    if !state { (v, state) = self.get_tree_eletrec(); }
    if !state { (v, state) = self.get_tree_eapprec(); }

    if !state { (v, state) = self.get_tree_eift(); }
    if !state { (v, state) = self.get_tree_eiff(); }

    if !state { (v, state) = self.get_tree_enil(); }
    if !state { (v, state) = self.get_tree_econs(); }
    if !state { (v, state) = self.get_tree_ematchnil(); }
    if !state { (v, state) = self.get_tree_ematchcons(); }

    if !state { (v, state) = self.get_tree_eplus()}
    if !state { (v, state) = self.get_tree_eminus(); }
    if !state { (v, state) = self.get_tree_etimes(); }
    if !state { (v, state) = self.get_tree_elt(); }

    if !state { (v, state) = self.get_tree_evar(); }

    if !state { (v, state) = self.get_tree_bplus(); }
    if !state { (v, state) = self.get_tree_bminus(); }
    if !state { (v, state) = self.get_tree_btimes(); }
    if !state { (v, state) = self.get_tree_blt(); }


    if state {
      v
    }else{
      // panic!("nothing is matched: {}", self.obj);
      None
    }
  }

  fn get_regex(&self, obj: Object) -> Regex{
    if let Object::EvalML4(c) = obj {
      match c {
        Rule::EInt => Regex::new(r"(.*)\|- (.*) evalto (.*)").unwrap(),
        Rule::EBool => Regex::new(r"(.*)\|- (.*) evalto (.*)").unwrap(),
        Rule::EVar => Regex::new(r"(.*)\|- (.*) evalto (.*)").unwrap(),
        Rule::EFun =>  Regex::new(r"(.*)\|- fun (.*?) -> (.*) evalto (.*)").unwrap(),
        Rule::ELetRec =>  Regex::new(r"(.*)\|- let rec (.*?) = fun (.*?) -> (.*) in (.*) evalto (.*)").unwrap(),
        Rule::EApp =>  Regex::new(r"(.*)\|- (.*) evalto (.*)").unwrap(),
        Rule::EAppRec =>  Regex::new(r"(.*)\|- (.*) evalto (.*)").unwrap(),
        Rule::EIfT => Regex::new(r"(.*)\|- if (.*) then (.*) else (.*) evalto (.*)").unwrap(),
        Rule::EIfF => Regex::new(r"(.*)\|- if (.*) then (.*) else (.*) evalto (.*)").unwrap(),
        Rule::EPlus => Regex::new(r"(.*)\|- (.*) \+ (.*) evalto (.*)").unwrap(),
        Rule::EMinus => Regex::new(r"(.*)\|- (.*) - (.*) evalto (.*)").unwrap(),
        Rule::ETimes => Regex::new(r"(.*)\|- (.*) \* (.*) evalto (.*)").unwrap(),
        Rule::ELt => Regex::new(r"(.*)\|- (.*) < (.*) evalto (.*)").unwrap(),
        Rule::ELet => Regex::new(r"(.*)\|- let (.*?) = (.*) in (.*) evalto (.*)").unwrap(),
        Rule::ENil => Regex::new(r"(.*)\|- \[\] evalto (.*)").unwrap(),
        Rule::ECons => Regex::new(r"(.*)\|- (.*) evalto (.*)").unwrap(),
        Rule::EMatchNil => Regex::new(r"(.*)\|- match (.*?) with \[\] -> (.*) \| (.*) -> (.*) evalto (.*)").unwrap(), // matchの仕方がまずそう
        Rule::EMatchCons => Regex::new(r"(.*)\|- match (.*?) with \[\] -> (.*) \| (.*) -> (.*) evalto (.*)").unwrap(),
        Rule::BPlus => Regex::new(r"(.*) plus (.*) is (.*)").unwrap(),
        Rule::BMinus => Regex::new(r"(.*) minus (.*) is (.*)").unwrap(),
        Rule::BTimes => Regex::new(r"(.*) times (.*) is (.*)").unwrap(),
        Rule::BLt => Regex::new(r"(.*) less than (.*) is (.*)").unwrap(),
      }
    }else{
      panic!("invalid object type: {:?}", obj)
    }
  }
}
