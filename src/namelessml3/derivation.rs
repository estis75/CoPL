use super::{NamelessML3, Rule};
use crate::solver::{Solver, RuleTree, Object};
use regex::Regex;
use std::collections::BTreeMap;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Int(i64),
  Bool(bool),
  Function((Vec<(String, Value)>, String, String)),
  RecFunction((Vec<(String, Value)>, String, String, String)),
  Variable(),
}

impl Display for Value {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Value::Bool(b) => write!(f, "{}", if *b {"true"} else {"false"}),
      Value::Int(n) => write!(f, "{}", n),
      Value::Function(fnc) => write!(f, "({})[fun {} -> {}]", (|f: &Vec<(String, Value)>|{
        let mut mpiter = f.into_iter();
        let head_of_head = mpiter.next();
        if head_of_head == None {
          String::from("")
        }else{
          let head_of_head = head_of_head.unwrap();
          let head = mpiter.into_iter()
            .map(|(key, value)| format!("{} = {}", key, value))
            .fold(format!("{} = {}", head_of_head.0, head_of_head.1), |lhs, rhs| lhs + ", " + &rhs);
          head
        }
      })(&fnc.0), fnc.1, fnc.2),
      Value::RecFunction(rfnc) => write!(f, "({})[rec {} = fun {} -> {}]", (|f: &Vec<(String, Value)>|{
        let mut mpiter = f.into_iter();
        let head_of_head = mpiter.next();
        if head_of_head == None {
          String::from("")
        }else{
          let head_of_head = head_of_head.unwrap();
          let head = mpiter.into_iter()
            .map(|(key, value)| format!("{} = {}", key, value))
            .fold(format!("{} = {}", head_of_head.0, head_of_head.1), |lhs, rhs| lhs + ", " + &rhs);
          head
        }
      })(&rfnc.0), rfnc.1, rfnc.2, rfnc.3),
      Value::Variable() => write!(f, ""),
    }
  }

}

impl NamelessML3 {
  pub fn parse_int(&self, val: String) -> Result<i64, std::num::ParseIntError> {
    val.parse()
  }

  pub fn parse_bool(&self, val: String) -> Result<bool, String> {
    if val == String::from("true") { 
      Ok(true)
    }else if val == String::from("false"){
      Ok(false)
    }else{
      Err(format!("this is not bool value: {}", val))
    }
  }

  pub fn parse_func(&self, val: &str) -> Result<(Vec<(String, Value)>, String, String), ()> {
    let reg = Regex::new(r"\((.*)\)\[fun (.*?) -> (.*)\]").unwrap();
    let ret = if let Some(cap) = reg.captures_iter(val).next() {
      Ok((self.get_env(cap[1].to_string()), cap[2].trim().to_string(), self.unwrap_if_parened(cap[3].to_string())))
    }else{
      Err(())
    };
    ret
  }

  pub fn parse_recfunc(&self, val: &str) -> Result<(Vec<(String, Value)>, String, String, String), ()> {
    let reg = Regex::new(r"\((.*)\)\[rec (.*?) = fun (.*?) -> (.*)\]").unwrap();
    let ret = if let Some(cap) = reg.captures_iter(val).next() {
      Ok((self.get_env(cap[1].to_string()), cap[2].trim().to_string(), cap[3].trim().to_string(), self.unwrap_if_parened(cap[4].to_string())))
    }else{
      Err(())
    };
    ret
  }

  fn parse_assign(&self, e: &str) -> Result<Option<(String, Value)>, ()>{
    let mut decr = e.split("=");
    let lhs = decr.next();
    let rhs = decr.next();
    if lhs == None {
      Ok(None)
    }else if rhs == None {
      let lhs = lhs.unwrap().trim().to_string();
      if !lhs.contains(' ') {
        Ok(Some((lhs, Value::Variable())))
      }else{
        Err(())
      }
    }else{
      let rhs = decr.fold(rhs.unwrap().to_string(), |lhs, rhs| lhs.trim().to_string() + " = " + rhs.trim());
      let lhs = lhs.unwrap().to_string().trim().to_string();
      let rhs = rhs.trim().to_string();
      if let Ok(num) = self.parse_int(rhs.clone()) {
        Ok(Some((lhs, Value::Int(num))))
      }else if let Ok(num) = self.parse_bool(rhs.clone()) {
        Ok(Some((lhs, Value::Bool(num))))
      }else if let Ok(num) = self.parse_func(&rhs) {
        Ok(Some((lhs, Value::Function(num))))
      }else if let Ok(num) = self.parse_recfunc(&rhs) {
        Ok(Some((lhs, Value::RecFunction(num))))
      }else{
        Err(())
      }
    }
  }

  //
  // (val) |- hoge evalto huga
  // のように与えてほしい
  //
  pub fn get_env(&self, val: String) -> Vec<(String, Value)> {

    let mut input_iter = val.chars().peekable();
    let mut ret = Vec::new();

    while let Some(_) = input_iter.peek() {
      let mut statement = String::new();
      let mut parens = 0;
      let mut sqbracket = 0;
      while let Some(c) = input_iter.next() {
        if c == '(' {
          parens += 1;
          statement.push(c);
        }else if c == ')'{
          parens -= 1;
          statement.push(c);
        }else if c == '[' {
          sqbracket += 1;
          statement.push(c);
        }else if c == ']'{
          sqbracket -= 1;
          statement.push(c);
        }else if parens == 0 && sqbracket == 0 && c == ',' {
          break;
        }else{
          statement.push(c);
        }
      }
      if let Ok(c) = self.parse_assign(&statement) {
        if let Some((lhs, rhs)) = c {
          ret.push((lhs, rhs));
        }
      }
    }

    ret
  }

  pub fn format_vectored_env(&self, mp: &Vec<(String, Value)>) -> String {
    let mut mpiter = mp.into_iter();
    let head_of_head = mpiter.next();
    if head_of_head == None {
      String::new()
    }else{
      let head_of_head = head_of_head.unwrap();
      let head = mpiter.into_iter()
        .map(|(key, value)| if let Value::Variable() = value { format!("{}", key) }else{ format!("{} = {}", key, value) })
        .fold(if let Value::Variable() = head_of_head.1 { format!("{}", head_of_head.0) }else{ format!("{} = {}", head_of_head.0, head_of_head.1) }, |lhs, rhs| lhs + ", " + &rhs);
      head
    }
  }
  
  #[allow(unused)]
  fn format_env_with_last(&self, mut mp: BTreeMap<String, Value>, key: String) -> String {
    let val = mp.remove(&key);
    let tail = format!("{} = {}", key, val.unwrap());

    let mut mpiter = mp.into_iter();
    let head_of_head = mpiter.next();
    if head_of_head == None {
      tail
    }else{
      let head_of_head = head_of_head.unwrap();
      let head = mpiter.into_iter()
        .map(|(key, value)| format!("{} = {}", key, value))
        .fold(format!("{} = {}", head_of_head.0, head_of_head.1), |lhs, rhs| lhs + ", " + &rhs);
      head + ", " + &tail
    }
  }

  pub fn unwrap_if_parened(&self, exp: String) -> String {
    if let Some(cap) = Regex::new(r"^\((.*)\)$").unwrap().captures_iter(&exp).next() {
      format!("{}", &cap[1])
    }else{
      exp
    }
  }

  pub fn get_tree_trint(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::NamelessML3(Rule::TrInt);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let i1 = self.parse_int(cap[2].to_string());
      let i2 = cap[3].to_string();
      if i1.is_ok() {
        let i2 = if i2 == String::from("?") {
          Ok(i1.clone().unwrap())
        }else {
          self.parse_int(i2)
        };

        if i2.is_ok() {
          let i1 = i1.ok().unwrap();
          let i2 = i2.ok().unwrap();
          if i1 == i2 {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} |- {} ==> {}", &cap[1].trim(), i1, i2),
              node: None
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_trbool(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::NamelessML3(Rule::TrBool);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let i1 = self.parse_bool(cap[2].to_string());
      let i2 = cap[3].to_string();
      if i1.is_ok() {
        let i2 = if i2 == String::from("?") {
          Ok(i1.clone().unwrap())
        }else {
          self.parse_bool(i2)
        };

        if i2.is_ok(){
          let i1 = i1.ok().unwrap();
          let i2 = i2.ok().unwrap();
          if i1 == i2 {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} |- {} ==> {}", &cap[1], i1, i2),
              node: None
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_trvar1(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::NamelessML3(Rule::TrVar1);
    let mut v = None;
    let mut state = false;

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let env = self.get_env(cap[1].trim().to_string());
      if let Some((x, _)) = env.last() {
        if x.trim() == cap[2].trim() {
          if cap[3].trim() == "1" {
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} |- {} ==> #1", self.format_vectored_env(&env), x),
              node: None
            });
            state = true;
          }
        }
      }
    }

    (v, state)
  }

  pub fn get_tree_trvar2(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::NamelessML3(Rule::TrVar2);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let env = self.get_env(cap[1].to_string());
      if let Some((key, _)) = env.last() {
        if key != &cap[2].trim() {
          if let Ok(n2) = self.parse_int(cap[3].to_string()) {
            let n1 = n2 - 1;
            let length = env.len();
            let val = format!("{} |- {} ==> #{}", self.format_vectored_env(&env.into_iter().take(length-1).collect()).trim(), self.unwrap_if_parened(cap[2].to_string()), n1);
            let c = NamelessML3{obj: val}.solver();
            if let Some(lhs) = c {
              tp.push(lhs.clone());
              if let Some(lhs) = Regex::new(r".* ==> #(.*)").unwrap().captures_iter(&lhs.val).next() {
                let lhs = self.parse_int(lhs[1].to_string());
                if lhs.is_ok() {
                  let lhs = lhs.ok().unwrap();
                  if lhs == n1 {
                    v = Some(RuleTree{
                      obj: OBJ,
                      val: format!("{} |- {} ==> #{}", &cap[1].trim(), cap[2].to_string(), &cap[3]),
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

  fn iflet_parser(sent: &str) -> (String, String) {
    let mut letnum = 1;
    let mut innum = 0;
    let mut index = 1;
    for i in 0..sent.len() {
      if i+3 <= sent.len() && &sent[i..i+3] == "let" {
        letnum += 1;
      }else if i+2 <= sent.len() && &sent[i..i+2] == "in" {
        innum += 1;
      }

      if letnum == innum {
        index = i;
        break;
      }
    }

    if index == 1 {
      (sent.to_string(), String::new())
    }else{
      (sent[..index-1].to_string(), sent[index+3..].to_string())
    }
  }

  pub fn get_tree_trlet(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::NamelessML3(Rule::TrLet);
    let mut v = None;
    let mut state = false;
    let mut tp = Vec::with_capacity(2);

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let env = self.get_env(cap[1].to_string());
      if !cap[2].contains(' ') {
        let (lparsed0, lparsed1) = Self::iflet_parser(&self.unwrap_if_parened(cap[3].to_string()));
        let lafter_in = if lparsed1 == String::from("") {
          self.unwrap_if_parened(cap[4].to_string())
        }else{
          lparsed1 + " in " + &self.unwrap_if_parened(cap[4].to_string())
        };

        let (rparsed0, rparsed1) = Self::iflet_parser(&self.unwrap_if_parened(cap[5].to_string()));
        let rafter_in = if rparsed1 == String::from("") {
          self.unwrap_if_parened(cap[6].to_string())
        }else{
          rparsed1 + " in " + &self.unwrap_if_parened(cap[6].to_string())
        };

        let val = format!("{} |- {} evalto {}", self.format_vectored_env(&env), &lparsed0, &rparsed0);
        let c = NamelessML3{obj: val}.solver();
        if let Some(c) = c {
          tp.push(c.clone());
          let val = format!("{} |- {} evalto {}", self.format_vectored_env(&env), &lafter_in, &rafter_in);
          let c = NamelessML3{obj: val}.solver();
          if let Some(c) = c {
            tp.push(c.clone());
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} |- let {} = {} in {} ==> let . {} in {}", &cap[1].trim(), &cap[2], &lparsed0, &lafter_in, &rparsed0, &rafter_in),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_trplus(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::NamelessML3(Rule::TrPlus);
    let mut v = None;
    let mut state = false;

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(2);
      let env = self.get_env(cap[1].to_string());

      let val = format!("{} |- {} ==> {}", self.format_vectored_env(&env), self.unwrap_if_parened(cap[2].to_string()), self.unwrap_if_parened(cap[4].to_string()));
      let c = NamelessML3{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        let val = format!("{} |- {} ==> {}", self.format_vectored_env(&env), self.unwrap_if_parened(cap[3].to_string()), self.unwrap_if_parened(cap[5].to_string()));
        let c = NamelessML3{obj: val}.solver();
        if let Some(rhs) = c {
          tp.push(rhs.clone());
          v = Some(RuleTree{
            obj: OBJ,
            val: format!("{} |- {} + {} ==> {} + {}", self.format_vectored_env(&env), &cap[2], &cap[3], &cap[4], &cap[5]),
            node: Some(tp)
          });
          state = true;
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_trminus(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::NamelessML3(Rule::TrMinus);
    let mut v = None;
    let mut state = false;

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(2);
      let env = self.get_env(cap[1].to_string());

      let val = format!("{} |- {} ==> {}", self.format_vectored_env(&env), self.unwrap_if_parened(cap[2].to_string()), self.unwrap_if_parened(cap[4].to_string()));
      let c = NamelessML3{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        let val = format!("{} |- {} ==> {}", self.format_vectored_env(&env), self.unwrap_if_parened(cap[3].to_string()), self.unwrap_if_parened(cap[5].to_string()));
        let c = NamelessML3{obj: val}.solver();
        if let Some(rhs) = c {
          tp.push(rhs.clone());
          v = Some(RuleTree{
            obj: OBJ,
            val: format!("{} |- {} - {} ==> {} - {}", self.format_vectored_env(&env), &cap[2], &cap[3], &cap[4], &cap[5]),
            node: Some(tp)
          });
          state = true;
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_trtimes(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::NamelessML3(Rule::TrTimes);
    let mut v = None;
    let mut state = false;

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(2);
      let env = self.get_env(cap[1].to_string());

      let val = format!("{} |- {} ==> {}", self.format_vectored_env(&env), self.unwrap_if_parened(cap[2].to_string()), self.unwrap_if_parened(cap[4].to_string()));
      let c = NamelessML3{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        let val = format!("{} |- {} ==> {}", self.format_vectored_env(&env), self.unwrap_if_parened(cap[3].to_string()), self.unwrap_if_parened(cap[5].to_string()));
        let c = NamelessML3{obj: val}.solver();
        if let Some(rhs) = c {
          tp.push(rhs.clone());
          v = Some(RuleTree{
            obj: OBJ,
            val: format!("{} |- {} * {} ==> {} * {}", self.format_vectored_env(&env), &cap[2], &cap[3], &cap[4], &cap[5]),
            node: Some(tp)
          });
          state = true;
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_trlt(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::NamelessML3(Rule::TrLt);
    let mut v = None;
    let mut state = false;

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(2);
      let env = self.get_env(cap[1].to_string());

      let val = format!("{} |- {} ==> {}", self.format_vectored_env(&env), self.unwrap_if_parened(cap[2].to_string()), self.unwrap_if_parened(cap[4].to_string()));
      let c = NamelessML3{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        let val = format!("{} |- {} ==> {}", self.format_vectored_env(&env), self.unwrap_if_parened(cap[3].to_string()), self.unwrap_if_parened(cap[5].to_string()));
        let c = NamelessML3{obj: val}.solver();
        if let Some(rhs) = c {
          tp.push(rhs.clone());
          v = Some(RuleTree{
            obj: OBJ,
            val: format!("{} |- {} < {} ==> {} < {}", self.format_vectored_env(&env), &cap[2], &cap[3], &cap[4], &cap[5]),
            node: Some(tp)
          });
          state = true;
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_trif(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::NamelessML3(Rule::TrIf);
    let mut v = None;
    let mut state = false;

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(2);
      let env = self.get_env(cap[1].to_string());
      dbg!(&env);

      let val = format!("{} |- {} ==> {}", self.format_vectored_env(&env), self.unwrap_if_parened(cap[2].to_string()), self.unwrap_if_parened(cap[5].to_string()));
      let c = NamelessML3{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs);
        let val = format!("{} |- {} ==> {}", self.format_vectored_env(&env), self.unwrap_if_parened(cap[3].to_string()), self.unwrap_if_parened(cap[6].to_string()));
        let c = NamelessML3{obj: val}.solver();
        if let Some(rhs) = c {
          tp.push(rhs);
          let val = format!("{} |- {} ==> {}", self.format_vectored_env(&env), self.unwrap_if_parened(cap[4].to_string()), self.unwrap_if_parened(cap[7].to_string()));
          let c = NamelessML3{obj: val}.solver();
          if let Some(rhs) = c {
            tp.push(rhs.clone());
            v = Some(RuleTree{
              obj: OBJ,
              val: format!("{} |- if {} then {} else {} ==> if {} then {} else {}", self.format_vectored_env(&env), &cap[2], &cap[3], &cap[4], &cap[5], &cap[6], &cap[7]),
              node: Some(tp)
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }
}
