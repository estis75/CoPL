use super::solver::{Solver, RuleTree, Object};
use regex::Regex;
use std::fmt::{self, Display, Formatter};
mod derivation;
#[allow(unused)]
use derivation::*;
mod error;
#[allow(unused)]
use error::*;
mod functions;
#[allow(unused)]
use functions::*;

pub struct EvalML3 {
  pub obj: String
}

#[derive(Debug, Clone)]
pub enum Rule{
  EInt,
  EBool,
  EVar1,
  EVar2,
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
  BPlus,
  BMinus,
  BTimes,
  BLt,
  EPlusBoolL,
  EPlusBoolR,
  EPlusErrorL,
  EPlusErrorR,
  EMinusBoolL,
  EMinusBoolR,
  EMinusErrorL,
  EMinusErrorR,
  ETimesBoolL,
  ETimesBoolR,
  ETimesErrorL,
  ETimesErrorR,
  ELtBoolL,
  ELtBoolR,
  ELtErrorL,
  ELtErrorR,
  EIfInt,
  EIfError,
  EIfTError,
  EIfFError,
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
      Self::EVar1 => write!(f, "E-Var1"),
      Self::EVar2 => write!(f, "E-Var2"),
      Self::BPlus => write!(f, "B-Plus"),
      Self::BMinus => write!(f, "B-Minus"),
      Self::BTimes => write!(f, "B-Times"),
      Self::BLt => write!(f, "B-Lt"),
      Self::EPlusBoolL => write!(f, "E-PlusBoolL"),
      Self::EPlusBoolR => write!(f, "E-PlusBoolR"),
      Self::EPlusErrorL => write!(f, "E-PlusErrorL"),
      Self::EPlusErrorR => write!(f, "E-PlusErrorR"),
      Self::EMinusBoolL => write!(f, "E-MinusBoolL"),
      Self::EMinusBoolR => write!(f, "E-MinusBoolR"),
      Self::EMinusErrorL => write!(f, "E-MinusErrorL"),
      Self::EMinusErrorR => write!(f, "E-MinusErrorR"),
      Self::ETimesBoolL => write!(f, "E-TimesBoolL"),
      Self::ETimesBoolR => write!(f, "E-TimesBoolR"),
      Self::ETimesErrorL => write!(f, "E-TimesErrorL"),
      Self::ETimesErrorR => write!(f, "E-TimesErrorR"),
      Self::ELtBoolL => write!(f, "E-LtBoolL"),
      Self::ELtBoolR => write!(f, "E-LtBoolR"),
      Self::ELtErrorL => write!(f, "E-LtErrorL"),
      Self::ELtErrorR => write!(f, "E-LtErrorR"),
      Self::EIfInt => write!(f, "E-IfInt"),
      Self::EIfError => write!(f, "E-IfError"),
      Self::EIfTError => write!(f, "E-IfTError"),
      Self::EIfFError => write!(f, "E-IfFError"),
    }
  }
}

impl EvalML3 {
  #[allow(unused)]
  pub fn solve(&self) -> Option<RuleTree> {
    self.solver()
  }
}

impl Solver for EvalML3 {
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

    if !state { (v, state) = self.get_tree_eplus()}
    if !state { (v, state) = self.get_tree_eminus(); }
    if !state { (v, state) = self.get_tree_etimes(); }
    if !state { (v, state) = self.get_tree_elt(); }

    if !state { (v, state) = self.get_tree_evar1(); }
    if !state { (v, state) = self.get_tree_evar2(); }

    if !state { (v, state) = self.get_tree_bplus(); }
    if !state { (v, state) = self.get_tree_bminus(); }
    if !state { (v, state) = self.get_tree_btimes(); }
    if !state { (v, state) = self.get_tree_blt(); }

    if !state { (v, state) = self.get_tree_eifint(); }
    if !state { (v, state) = self.get_tree_eiferror(); }
    if !state { (v, state) = self.get_tree_eifterror(); }
    if !state { (v, state) = self.get_tree_eifferror(); }

    if !state { (v, state) = self.get_tree_eplusbooll(); }
    if !state { (v, state) = self.get_tree_eplusboolr(); }
    if !state { (v, state) = self.get_tree_epluserrorl(); }
    if !state { (v, state) = self.get_tree_epluserrorr(); }

    if !state { (v, state) = self.get_tree_eminusbooll(); }
    if !state { (v, state) = self.get_tree_eminusboolr(); }
    if !state { (v, state) = self.get_tree_eminuserrorl(); }
    if !state { (v, state) = self.get_tree_eminuserrorr(); }

    if !state { (v, state) = self.get_tree_etimesbooll(); }
    if !state { (v, state) = self.get_tree_etimesboolr(); }
    if !state { (v, state) = self.get_tree_etimeserrorl(); }
    if !state { (v, state) = self.get_tree_etimeserrorr(); }

    if !state { (v, state) = self.get_tree_eltbooll(); }
    if !state { (v, state) = self.get_tree_eltboolr(); }
    if !state { (v, state) = self.get_tree_elterrorl(); }
    if !state { (v, state) = self.get_tree_elterrorr(); }

    if state {
      v
    }else{
      // panic!("nothing is matched: {}", self.obj);
      None
    }
  }

  fn get_regex(&self, obj: Object) -> Regex{
    if let Object::EvalML3(c) = obj {
      match c {
        Rule::EInt => Regex::new(r"(.*)\|- (.*) evalto (.*)").unwrap(),
        Rule::EBool => Regex::new(r"(.*)\|- (.*) evalto (.*)").unwrap(),
        Rule::EVar1 => Regex::new(r"(.*)\|- (.*) evalto (.*)").unwrap(),
        Rule::EVar2 => Regex::new(r"(.*)\|- (.*) evalto (.*)").unwrap(),
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
        Rule::BPlus => Regex::new(r"(.*) plus (.*) is (.*)").unwrap(),
        Rule::BMinus => Regex::new(r"(.*) minus (.*) is (.*)").unwrap(),
        Rule::BTimes => Regex::new(r"(.*) times (.*) is (.*)").unwrap(),
        Rule::BLt => Regex::new(r"(.*) less than (.*) is (.*)").unwrap(),
        Rule::EPlusBoolL => Regex::new(r"(.*) \+ (.*) evalto (.*)").unwrap(),
        Rule::EPlusBoolR => Regex::new(r"(.*) \+ (.*) evalto (.*)").unwrap(),
        Rule::EPlusErrorL => Regex::new(r"(.*) \+ (.*) evalto (.*)").unwrap(),
        Rule::EPlusErrorR => Regex::new(r"(.*) \+ (.*) evalto (.*)").unwrap(),
        Rule::EMinusBoolL => Regex::new(r"(.*) - (.*) evalto (.*)").unwrap(),
        Rule::EMinusBoolR => Regex::new(r"(.*) - (.*) evalto (.*)").unwrap(),
        Rule::EMinusErrorL => Regex::new(r"(.*) - (.*) evalto (.*)").unwrap(),
        Rule::EMinusErrorR => Regex::new(r"(.*) - (.*) evalto (.*)").unwrap(),
        Rule::ETimesBoolL => Regex::new(r"(.*) \* (.*) evalto (.*)").unwrap(),
        Rule::ETimesBoolR => Regex::new(r"(.*) \* (.*) evalto (.*)").unwrap(),
        Rule::ETimesErrorL => Regex::new(r"(.*) \* (.*) evalto (.*)").unwrap(),
        Rule::ETimesErrorR => Regex::new(r"(.*) \* (.*) evalto (.*)").unwrap(),
        Rule::ELtBoolL => Regex::new(r"(.*) < (.*) evalto (.*)").unwrap(),
        Rule::ELtBoolR => Regex::new(r"(.*) < (.*) evalto (.*)").unwrap(),
        Rule::ELtErrorL => Regex::new(r"(.*) < (.*) evalto (.*)").unwrap(),
        Rule::ELtErrorR => Regex::new(r"(.*) < (.*) evalto (.*)").unwrap(),
        Rule::EIfInt => Regex::new(r"^if (.*) then (.*) else (.*) evalto (.*)").unwrap(),
        Rule::EIfError => Regex::new(r"^if (.*) then (.*) else (.*) evalto (.*)").unwrap(),
        Rule::EIfTError => Regex::new(r"^if (.*) then (.*) else (.*) evalto (.*)").unwrap(),
        Rule::EIfFError => Regex::new(r"^if (.*) then (.*) else (.*) evalto (.*)").unwrap(),
      }
    }else{
      panic!("invalid object type: {:?}", obj)
    }
  }
}
