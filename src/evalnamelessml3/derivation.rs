use super::{EvalNamelessML3, Rule};
use crate::solver::{Solver, RuleTree, Object};
use regex::Regex;
use std::collections::BTreeMap;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Int(i64),
  Bool(bool),
  Function((Vec<Value>, String)),
  RecFunction((Vec<Value>, String)),
}

impl Display for Value {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Value::Bool(b) => write!(f, "{}", if *b {"true"} else {"false"}),
      Value::Int(n) => write!(f, "{}", n),
      Value::Function(fnc) => write!(f, "({})[fun . -> {}]", (|f: &Vec<Value>|{
        let mut mpiter = f.into_iter();
        let head_of_head = mpiter.next();
        if head_of_head == None {
          String::from("")
        }else{
          let head_of_head = head_of_head.unwrap();
          let head = mpiter.into_iter()
            .map(|value| format!("{}", value))
            .fold(format!("{}", head_of_head), |lhs, rhs| lhs + ", " + &rhs);
          head
        }
      })(&fnc.0), fnc.1),
      Value::RecFunction(rfnc) => write!(f, "({})[rec . = fun . -> {}]", (|f: &Vec<Value>|{
        let mut mpiter = f.into_iter();
        let head_of_head = mpiter.next();
        if head_of_head == None {
          String::from("")
        }else{
          let head_of_head = head_of_head.unwrap();
          let head = mpiter.into_iter()
            .map(|value| format!("{}", value))
            .fold(format!("{}", head_of_head), |lhs, rhs| lhs + ", " + &rhs);
          head
        }
      })(&rfnc.0), rfnc.1),
    }
  }

}

impl EvalNamelessML3 {
  pub fn parse_int(&self, val: &str) -> Result<i64, std::num::ParseIntError> {
    val.parse()
  }

  pub fn parse_bool(&self, val: &str) -> Result<bool, String> {
    if val == "true" { 
      Ok(true)
    }else if val == "false" {
      Ok(false)
    }else{
      Err(format!("this is not bool value: {}", val))
    }
  }

  pub fn parse_value(&self, val: &str) -> Result<Value, String> {
    let intvalue = self.parse_int(val);
    let boolvalue = self.parse_bool(val);
    let function = self.parse_func(val);
    let recfunction = self.parse_recfunc(val);
    if intvalue.is_ok() {
      Ok(Value::Int(intvalue.ok().unwrap()))
    }else if boolvalue.is_ok() {
      Ok(Value::Bool(boolvalue.ok().unwrap()))
    }else if function.is_ok() {
      Ok(Value::Function(function.ok().unwrap()))
    }else if recfunction.is_ok() {
      Ok(Value::RecFunction(recfunction.ok().unwrap()))
    }else{
      Err(String::from("this is not value"))
    }
  }

  pub fn parse_func(&self, val: &str) -> Result<(Vec<Value>, String), ()> {
    let reg = Regex::new(r"\((.*)\)\[fun \. -> (.*)\]").unwrap();
    let ret = if let Some(cap) = reg.captures_iter(val).next() {
      Ok((self.get_env(&cap[1]), self.unwrap_if_parened(cap[2].to_string())))
    }else{
      Err(())
    };
    ret
  }

  pub fn parse_recfunc(&self, val: &str) -> Result<(Vec<Value>, String), ()> {
    let reg = Regex::new(r"\((.*)\)\[rec \. = fun \. -> (.*)\]").unwrap();
    let ret = if let Some(cap) = reg.captures_iter(val).next() {
      Ok((self.get_env(&cap[1]), self.unwrap_if_parened(cap[2].to_string())))
    }else{
      Err(())
    };
    ret
  }

  fn is_value(&self, val: &str) -> bool {
    self.parse_int(val).is_ok() 
     || self.parse_bool(val).is_ok() 
     || self.parse_func(val).is_ok()
     || self.parse_recfunc(val).is_ok()
  }

  fn is_function(&self, val: &str) -> bool {
    self.parse_func(val).is_ok()
  }

  fn parse_assign(&self, e: &str) -> Result<Option<Value>, ()>{
    let rhs = e.trim();
    if let Ok(num) = self.parse_int(rhs) {
      Ok(Some(Value::Int(num)))
    }else if let Ok(num) = self.parse_bool(rhs) {
      Ok(Some(Value::Bool(num)))
    }else if let Ok(num) = self.parse_func(&rhs) {
      Ok(Some(Value::Function(num)))
    }else if let Ok(num) = self.parse_recfunc(&rhs) {
      Ok(Some(Value::RecFunction(num)))
    }else{
      Err(())
    }
    
  }

  //
  // (val) |- hoge evalto huga
  // のように与えてほしい
  //
  pub fn get_env(&self, val: &str) -> Vec<Value> {

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
        if let Some(rhs) = c {
          ret.push(rhs);
        }
      }
    }

    ret
  }

  pub fn format_vectored_env(&self, mp: &Vec<Value>) -> String {
    let mut mpiter = mp.into_iter();
    let head_of_head = mpiter.next();
    if head_of_head == None {
      String::new()
    }else{
      let head_of_head = head_of_head.unwrap();
      let head = mpiter.into_iter()
        .map(|key| format!("{}", key))
        .fold(format!("{}", head_of_head), |lhs, rhs| lhs + ", " + &rhs);
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

  pub fn get_tree_eint(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalNamelessML3(Rule::EInt)).captures_iter(&self.obj).next() {
      let i1 = self.parse_int(&cap[2]);
      let i2 = &cap[3];
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
              obj: Object::EvalNamelessML3(Rule::EInt),
              val: format!("{} |- {} evalto {}", &cap[1].trim(), i1, i2),
              node: None
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_ebool(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalNamelessML3(Rule::EBool)).captures_iter(&self.obj).next() {
      let i1 = self.parse_bool(&cap[2]);
      let i2 = &cap[3];
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
              obj: Object::EvalNamelessML3(Rule::EBool),
              val: format!("{} |- {} evalto {}", &cap[1], i1, i2),
              node: None
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_evar(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalNamelessML3(Rule::EVar);
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let env = self.get_env(cap[1].trim());
      if let Ok(index) = self.parse_int(&cap[2]) {
        if cap[3].trim() == "?" {
          let w = &env[env.len() - index.clone() as usize];
          v = Some(RuleTree{
            obj: OBJ,
            val: format!("{} |- #{} evalto {}", self.format_vectored_env(&env), index, w),
            node: None
          });
          state = true;
        }else{
          if let Ok(rhs) = self.parse_value(cap[3].trim()) {
            if (index as usize) <= env.len() {
              let w = &env[env.len() - index.clone() as usize];
              if w == &rhs {
                v = Some(RuleTree{
                  obj: OBJ,
                  val: format!("{} |- #{} evalto {}", self.format_vectored_env(&env), index, w),
                  node: None
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

  pub fn get_tree_eift(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalNamelessML3(Rule::EIfT)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} |- {} evalto true", &cap[1].trim(), self.unwrap_if_parened(cap[2].to_string()).trim());
      let c = EvalNamelessML3{obj: val}.solver();
      if let Some(c) = c {
        tp.push(c);
        if self.is_value(&cap[5]) || &cap[5] == "?" {
          let val = format!("{} |- {} evalto {}", &cap[1].trim(), self.unwrap_if_parened(cap[3].to_string()), &cap[5]);
          let c = EvalNamelessML3{obj: val}.solver();
          if let Some(c) = c {
            tp.push(c.clone());
            if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
              let val = if cap[5].to_string() == String::from("?") {
                tmpc[1].to_string()
              }else{
                cap[5].to_string()
              };
              if tmpc[1].to_string() == val {
                v = Some(RuleTree{
                  obj: Object::EvalNamelessML3(Rule::EIfT),
                  val: format!("{} |- if {} then {} else {} evalto {}", &cap[1], &cap[2], &cap[3], &cap[4], val),
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

  pub fn get_tree_eiff(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalNamelessML3(Rule::EIfF)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} |- {} evalto false", &cap[1].trim(), self.unwrap_if_parened(cap[2].to_string()).trim());
      let c = EvalNamelessML3{obj: val}.solver();
      if let Some(c) = c {
        tp.push(c);
        if self.is_value(&cap[5]) || &cap[5] == "?" {
          let val = format!("{} |- {} evalto {}", &cap[1].trim(), self.unwrap_if_parened(cap[4].to_string()), &cap[5]);
          let c = EvalNamelessML3{obj: val}.solver();
          if let Some(c) = c {
            tp.push(c.clone());
            if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
              let val = if cap[5].to_string() == String::from("?") {
                tmpc[1].to_string()
              }else{
                cap[5].to_string()
              };
              if tmpc[1].to_string() == val {
                v = Some(RuleTree{
                  obj: Object::EvalNamelessML3(Rule::EIfF),
                  val: format!("{} |- if {} then {} else {} evalto {}", &cap[1], &cap[2], &cap[3], &cap[4], val),
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

  pub fn get_tree_elet(&self) -> (Option<RuleTree>, bool) {
    const OBJ: Object = Object::EvalNamelessML3(Rule::ELet);
    let mut v = None;
    let mut state = false;
    let mut tp = Vec::with_capacity(3);

    if let Some(cap) = self.get_regex(OBJ).captures_iter(&self.obj).next() {
      let env = self.get_env(&cap[1]);

      let (parsed0, parsed1) = Self::iflet_parser(&self.unwrap_if_parened(cap[2].to_string()));
      let after_in = if parsed1 == String::from("") {
        self.unwrap_if_parened(cap[3].to_string())
      }else{
        parsed1 + " in " + &self.unwrap_if_parened(cap[3].to_string())
      };

      let val = format!("{} |- {} evalto ?", self.format_vectored_env(&env), parsed0);
      let c = EvalNamelessML3{obj: val}.solver();
      if let Some(c) = c {
        tp.push(c.clone());
        if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
          let v1 = self.parse_value(&tmpc[1]);
          if v1.is_ok() {
            let mut added_env = env.clone();
            added_env.push(v1.ok().unwrap());
            let val = format!("{} |- {} evalto {}", self.format_vectored_env(&added_env), after_in, &cap[4]);
            let c = EvalNamelessML3{obj: val}.solver();
            if let Some(c) = c {
              tp.push(c.clone());
              if let Some(tmpc) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&c.val).next() {
                let val = if cap[4].to_string() == String::from("?") {
                  tmpc[1].to_string()
                }else if self.is_function(&cap[4]) && self.is_function(&tmpc[1]) {
                  let lhs = self.parse_func(&cap[4]).ok().unwrap();
                  let rhs = self.parse_func(&tmpc[1]).ok().unwrap();
                  if lhs.0.len() == rhs.0.len() 
                   && lhs.0.iter().zip(rhs.0.iter()).map(|(lhs, rhs)| lhs == rhs).fold(true, |lhs, rhs| lhs && rhs) 
                   && lhs.1 == rhs.1 {
                    tmpc[1].to_string()
                  }else{
                    "".to_string()
                  }
                }else{
                  cap[4].to_string()
                };
                if tmpc[1].to_string() == val {
                  v = Some(RuleTree{
                    obj: OBJ,
                    val: format!("{} |- let . = {} in {} evalto {}", self.format_vectored_env(&env), &cap[2], &cap[3], val),
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

  pub fn get_tree_eplus(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalNamelessML3(Rule::EPlus)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} |- {} evalto ?", &cap[1].trim(), self.unwrap_if_parened(cap[2].to_string()));
      let c = EvalNamelessML3{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let val = format!("{} |- {} evalto ?", &cap[1].trim(), self.unwrap_if_parened(cap[3].to_string()));
          let c = EvalNamelessML3{obj: val}.solver();
          if let Some(rhs) = c {
            tp.push(rhs.clone());
            if let Some(rhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&rhs.val).next() {
              let lhs = self.parse_int(&lhs[1]);
              let rhs = self.parse_int(&rhs[1]);
              if lhs.is_ok() && rhs.is_ok() {
                let lhs = lhs.ok().unwrap();
                let rhs = rhs.ok().unwrap();
                let val = if cap[4].to_string() == String::from("?") {
                  Ok(lhs + rhs)
                }else{
                  let ret = self.parse_int(&cap[4]);
                  if ret.is_ok() {
                    Ok(ret.ok().unwrap())
                  }else{
                    Err(())
                  }
                };
                if val.is_ok() && lhs + rhs == val.ok().unwrap() {
                  let val = val.ok().unwrap();
                  let tpval = format!("{} plus {} is {}", lhs, rhs, val);
                  let c = EvalNamelessML3{obj: tpval}.solver();
                  if let Some(c) = c {
                    tp.push(c);

                    v = Some(RuleTree{
                      obj: Object::EvalNamelessML3(Rule::EPlus),
                      val: format!("{} |- {} + {} evalto {}", &cap[1].trim(), &cap[2], &cap[3], val),
                      node: Some(tp)
                    });
                    state = true;
                  }
                }else{
                  // panic!("{} + {} = {}", lhs, rhs, val)
                }
              }
            }
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_eminus(&self) -> (Option<RuleTree>, bool) {
    // check: 値がマイナスになるパターンは大丈夫なんか？
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalNamelessML3(Rule::EMinus)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} |- {} evalto ?", &cap[1].trim(), self.unwrap_if_parened(cap[2].to_string()));
      let c = EvalNamelessML3{obj: val.clone()}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let val = format!("{} |- {} evalto ?", &cap[1].trim(), self.unwrap_if_parened(cap[3].to_string()));
          let c = EvalNamelessML3{obj: val}.solver();
          if let Some(rhs) = c {
            tp.push(rhs.clone());
            if let Some(rhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&rhs.val).next() {
              let lhs = self.parse_int(&lhs[1]);
              let rhs = self.parse_int(&rhs[1]);
              if lhs.is_ok() && rhs.is_ok() {
                let lhs = lhs.ok().unwrap();
                let rhs = rhs.ok().unwrap();
                let val = if cap[4].to_string() == String::from("?") {
                  lhs - rhs
                }else{
                  self.parse_int(&cap[4]).ok().unwrap()
                };
                if lhs - rhs == val {
                  let tpval = format!("{} minus {} is {}", lhs, rhs, val);
                  let c = EvalNamelessML3{obj: tpval}.solver();
                  if let Some(c) = c {
                    tp.push(c);

                    v = Some(RuleTree{
                      obj: Object::EvalNamelessML3(Rule::EMinus),
                      val: format!("{} |- {} - {} evalto {}", &cap[1], &cap[2], &cap[3], val),
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

  pub fn get_tree_etimes(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalNamelessML3(Rule::ETimes)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} |- {} evalto ?", &cap[1].trim(), self.unwrap_if_parened(cap[2].to_string()));
      let c = EvalNamelessML3{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let val = format!("{} |- {} evalto ?", &cap[1].trim(), self.unwrap_if_parened(cap[3].to_string()));
          let c = EvalNamelessML3{obj: val}.solver();
          if let Some(rhs) = c {
            tp.push(rhs.clone());
            if let Some(rhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&rhs.val).next() {
              let lhs = self.parse_int(&lhs[1]);
              let rhs = self.parse_int(&rhs[1]);
              if lhs.is_ok() && rhs.is_ok() {
                let lhs = lhs.ok().unwrap();
                let rhs = rhs.ok().unwrap();
                let val = if cap[4].to_string() == String::from("?") {
                  lhs * rhs
                }else{
                  self.parse_int(&cap[4]).ok().unwrap()
                };
                if lhs * rhs == val {
                  let tpval = format!("{} times {} is {}", lhs, rhs, val);
                  let c = EvalNamelessML3{obj: tpval}.solver();
                  if let Some(c) = c {
                    tp.push(c);

                    v = Some(RuleTree{
                      obj: Object::EvalNamelessML3(Rule::ETimes),
                      val: format!("{} |- {} * {} evalto {}", &cap[1].trim(), &cap[2], &cap[3], val),
                      node: Some(tp)
                    });
                    state = true;
                  }
                }else{
                  // panic!("{} * {} = {}", lhs, rhs, val)
                }
              }
            }
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_elt(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalNamelessML3(Rule::ELt)).captures_iter(&self.obj).next() {
      let mut tp = Vec::with_capacity(3);

      let val = format!("{} |- {} evalto ?", &cap[1].trim(), self.unwrap_if_parened(cap[2].to_string()));
      let c = EvalNamelessML3{obj: val}.solver();
      if let Some(lhs) = c {
        tp.push(lhs.clone());
        if let Some(lhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&lhs.val).next() {
          let val = format!("{} |- {} evalto ?", &cap[1].trim(), self.unwrap_if_parened(cap[3].to_string()));
          let c = EvalNamelessML3{obj: val}.solver();
          if let Some(rhs) = c {
            tp.push(rhs.clone());
            if let Some(rhs) = Regex::new(r".* evalto (.*)").unwrap().captures_iter(&rhs.val).next() {
              let lhs = self.parse_int(&lhs[1]);
              let rhs = self.parse_int(&rhs[1]);
              if lhs.is_ok() && rhs.is_ok() {
                let lhs = lhs.ok().unwrap();
                let rhs = rhs.ok().unwrap();
                let val = if cap[4].to_string() == String::from("?") {
                  lhs < rhs
                }else{
                  self.parse_bool(&cap[4]).ok().unwrap()
                };
                if (lhs < rhs) == val {
                  let tpval = format!("{} less than {} is {}", lhs, rhs, val);
                  let c = EvalNamelessML3{obj: tpval}.solver();
                  if let Some(c) = c {
                    tp.push(c);

                    v = Some(RuleTree{
                      obj: Object::EvalNamelessML3(Rule::ELt),
                      val: format!("{} |- {} < {} evalto {}", &cap[1], &cap[2], &cap[3], val),
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

  pub fn get_tree_bplus(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalNamelessML3(Rule::BPlus)).captures_iter(&self.obj).next() {
      let i1 = self.parse_int(&cap[1]);
      let i2 = self.parse_int(&cap[2]);
      let i3 = &cap[3];
      if i1.is_ok() && i2.is_ok() {
        let i3 = if i3 == "?" {
          Ok(i1.clone().unwrap() + i2.clone().unwrap())
        }else {
          self.parse_int(i3)
        };


        if i3.is_ok(){
          let i1 = i1.ok().unwrap();
          let i2 = i2.ok().unwrap();
          let i3 = i3.ok().unwrap();

          if i1 + i2 == i3 {
            v = Some(RuleTree{
              obj: Object::EvalNamelessML3(Rule::BPlus),
              val: format!("{} plus {} is {}", i1, i2, i3),
              node: None
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_bminus(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalNamelessML3(Rule::BMinus)).captures_iter(&self.obj).next() {
      let i1 = self.parse_int(&cap[1]);
      let i2 = self.parse_int(&cap[2]);
      let i3 = &cap[3];
      if i1.is_ok() && i2.is_ok() {
        let i3 = if i3 == "?" {
          Ok(i1.clone().unwrap() - i2.clone().unwrap())
        }else {
          self.parse_int(i3)
        };

        if i3.is_ok(){
          let i1 = i1.ok().unwrap();
          let i2 = i2.ok().unwrap();
          let i3 = i3.ok().unwrap();
          if i1 - i2 == i3 {
            v = Some(RuleTree{
              obj: Object::EvalNamelessML3(Rule::BMinus),
              val: format!("{} minus {} is {}", i1, i2, i3),
              node: None
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_btimes(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalNamelessML3(Rule::BTimes)).captures_iter(&self.obj).next() {
      let i1 = self.parse_int(&cap[1]);
      let i2 = self.parse_int(&cap[2]);
      let i3 = &cap[3];
      if i1.is_ok() && i2.is_ok() {
        let i3 = if i3 == String::from("?") {
          Ok(i1.clone().unwrap() * i2.clone().unwrap())
        }else {
          self.parse_int(i3)
        };

        if i3.is_ok(){
          let i1 = i1.ok().unwrap();
          let i2 = i2.ok().unwrap();
          let i3 = i3.ok().unwrap();
          if i1 * i2 == i3 {
            v = Some(RuleTree{
              obj: Object::EvalNamelessML3(Rule::BTimes),
              val: format!("{} times {} is {}", i1, i2, i3),
              node: None
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }

  pub fn get_tree_blt(&self) -> (Option<RuleTree>, bool) {
    let mut v = None;
    let mut state = false;
    if let Some(cap) = self.get_regex(Object::EvalNamelessML3(Rule::BLt)).captures_iter(&self.obj).next() {
      let i1 = self.parse_int(&cap[1]);
      let i2 = self.parse_int(&cap[2]);
      let i3 = &cap[3];
      if i1.is_ok() && i2.is_ok() {
        let i3 = if i3 == "?" {
          Ok(i1.clone().unwrap() < i2.clone().unwrap())
        }else {
          self.parse_bool(i3)
        };

        if i3.is_ok() {
          let i1 = i1.ok().unwrap();
          let i2 = i2.ok().unwrap();
          let i3 = i3.ok().unwrap();
          if (i1 < i2) == i3 {
            v = Some(RuleTree{
              obj: Object::EvalNamelessML3(Rule::BLt),
              val: format!("{} less than {} is {}", i1, i2, i3),
              node: None
            });
            state = true;
          }
        }
      }
    }
    (v, state)
  }
}