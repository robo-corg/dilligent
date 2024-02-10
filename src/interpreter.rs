use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::{fmt, mem};

use crate::ast::Op;
use eyre::{eyre, Result};
use itertools::Itertools;

#[derive(Clone, Default)]
struct Dict(Vec<(Value, Value)>);

impl fmt::Debug for Dict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(self.0.iter().map(|&(ref k, ref v)| (k, v)))
            .finish()
    }
}

const ORDERED_DICT_NAME: Global = Global {
    module: Cow::Borrowed("collections"),
    name: Cow::Borrowed("OrderedDict"),
};

fn ordered_dict_constructor(interp: &mut Interpreter, args: Value) -> Result<Value> {
    match args {
        Value::Tuple(args) if args.is_empty() => {
            return Ok(Value::OrderedDict(OrderedDict::default()))
        }
        other => {
            return Err(eyre!("Unexpected arguments for OrderedDict"));
        }
    }
}

#[derive(Clone, Default)]
struct OrderedDict(Vec<(Value, Value)>);

impl fmt::Debug for OrderedDict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OrderedDict ")?;
        f.debug_map()
            .entries(self.0.iter().map(|&(ref k, ref v)| (k, v)))
            .finish()
    }
}

trait FunctionDef {
    fn name(&self) -> &str;
    fn call(&self, interpreter: &mut Interpreter, value: Value) -> Result<Value>;
}

#[derive(Clone)]
struct Function(Arc<dyn FunctionDef>);

impl Function {
    fn call(&self, interpreter: &mut Interpreter, value: Value) -> Result<Value> {
        self.0.call(interpreter, value)
    }
}

struct FnFunctionDef<F>(String, F);

impl<F> FunctionDef for FnFunctionDef<F>
where
    F: Fn(&mut Interpreter, Value) -> Result<Value>,
{
    fn name(&self) -> &str {
        self.0.as_str()
    }

    fn call(&self, interpreter: &mut Interpreter, value: Value) -> Result<Value> {
        self.1(interpreter, value)
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.name())
    }
}

impl Function {
    pub fn from_fn<F>(name: String, f: F) -> Self
    where
        F: Fn(&mut Interpreter, Value) -> Result<Value> + 'static,
    {
        Function(Arc::new(FnFunctionDef(name, f)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Global {
    module: Cow<'static, str>,
    name: Cow<'static, str>,
}

#[derive(Clone)]
pub enum Value {
    U32(u32),
    I32(i32),
    String(String),
    Bool(bool),
    Dict(Dict),
    OrderedDict(OrderedDict),
    Tuple(Vec<Value>),
    List(Vec<Value>),
    Global(Global),
    PersistentLoad(Box<Value>),
    Reduce(Box<Value>, Box<Value>),
    Function(Function),
    SetState(Box<Value>, Box<Value>),
}

impl Value {
    fn set_item(&mut self, key: Value, value: Value) -> Result<()> {
        match self {
            Value::Dict(d) => {
                d.0.push((key, value));
            }
            Value::OrderedDict(d) => {
                d.0.push((key, value));
            }
            other => {
                return Err(eyre!("Type can not be have items set"));
            }
        }

        Ok(())
    }

    fn set_items<I>(&mut self, items: I) -> Result<()>
        where I: Iterator<Item=(Value, Value)>
    {
        match self {
            Value::Dict(d) => {
                d.0.extend(items)
            }
            Value::OrderedDict(d) => {
                d.0.extend(items)
            }
            other => {
                return Err(eyre!("Type can not be have items set"));
            }
        }

        Ok(())
    }

    fn extend(&mut self, items: Vec<Value>) -> Result<()> {
        match self {
            Value::List(existing_items) => {
                existing_items.extend(items.into_iter());
            }
            other => {
                return Err(eyre!("Type can not be appended/extended"));
            }
        }

        Ok(())
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::U32(arg0) => write!(f, "{:?}", arg0),
            Self::I32(arg0) => write!(f, "{:?}", arg0),
            Self::String(arg0) => write!(f, "{:?}", arg0),
            Self::Bool(b) => write!(f, "{:?}", b),
            Self::Dict(arg0) => if f.alternate() { write!(f, "{:#?}", arg0) } else { write!(f, "{:?}", arg0) },
            Self::OrderedDict(arg0) => if f.alternate() { write!(f, "{:#?}", arg0) } else { write!(f, "{:?}", arg0) },
            Self::Function(arg0) => write!(f, "{:?}", arg0),
            Self::Tuple(arg0) => f.debug_list().entries(arg0.iter()).finish(), //f.debug_tuple("Tuple").field(arg0).finish(),
            Self::List(arg0) => f.debug_list().entries(arg0.iter()).finish(), //f.debug_tuple("Tuple").field(arg0).finish(),
            Self::Global(arg0) => write!(f, "{:?}", arg0),
            Self::PersistentLoad(arg0) => f.debug_tuple("PersistentLoad").field(arg0).finish(),
            Self::Reduce(func, args) => f.debug_tuple("Reduce").field(func).field(args).finish(),
            Self::SetState(inst, state) => f.debug_tuple("SetState").field(inst).field(state).finish(),
        }
    }
}

impl From<Function> for Value {
    fn from(value: Function) -> Self {
        Value::Function(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::I32(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::I32(value as i32)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::I32(value as i32)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

#[derive(Debug)]
pub struct Interpreter {
    globals: HashMap<Global, Value>,
    stack: Vec<Value>,
    metastack: Vec<Vec<Value>>,
    memo: HashMap<u32, Value>,
    stop_value: Option<Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interp = Interpreter {
            globals: HashMap::new(),
            stack: Vec::new(),
            metastack: Vec::new(),
            memo: HashMap::new(),
            stop_value: None,
        };

        interp.set_global(
            ORDERED_DICT_NAME.clone(),
            Function::from_fn("OrderedDict".to_string(), ordered_dict_constructor).into(),
        );

        interp
    }

    pub fn set_global(&mut self, path: Global, value: Value) {
        self.globals.insert(path, value);
    }

    fn pop_mark(&mut self) -> Vec<Value> {
        let mut stack = self.metastack.pop().unwrap();
        mem::swap(&mut stack, &mut self.stack);
        stack
    }

    pub fn exec_op(&mut self, op: Op) -> Result<bool> {
        match op {
            Op::Proto(_) => {}
            Op::EmptyDict => {
                self.stack.push(Value::Dict(Dict::default()));
            }
            Op::EmptyList => {
                self.stack.push(Value::List(Vec::default()));
            }
            Op::BInput(index) => {
                self.memo
                    .insert(index as u32, self.stack.last().unwrap().clone());
            }
            Op::LongBInput(index) => {
                self.memo
                    .insert(index as u32, self.stack.last().unwrap().clone());
            }
            Op::Binunicode(s) => self.stack.push(s.into()),
            Op::Global(module, name) => {
                let global = Global {
                    module: Cow::Owned(module),
                    name: Cow::Owned(name),
                };

                if let Some(global_def) = self.globals.get(&global) {
                    self.stack.push(global_def.clone());
                }
                else {
                    self.stack.push(Value::Global(global));
                }
            }
            Op::BinInt(value) => self.stack.push(value.into()),
            Op::BinInt1(value) => self.stack.push(value.into()),
            Op::BinInt2(value) => self.stack.push(value.into()),
            Op::BinGet(u8_index) => {
                let index = u8_index as u32;
                let value = self
                    .memo
                    .get(&index)
                    .ok_or(eyre!("Memo value not found at index {index}"))?
                    .clone();
                self.stack.push(value);
            }
            Op::LongBinGet(index) => {
                let value = self
                    .memo
                    .get(&index)
                    .ok_or(eyre!("Memo value not found at index {index}"))?
                    .clone();
                self.stack.push(value)
            }
            Op::Mark => {
                let old_stack = mem::take(&mut self.stack);

                self.metastack.push(old_stack);
            }
            Op::Tuple => {
                let items = self.pop_mark();
                //println!("Tuple: {:?}", items);
                self.stack.push(Value::Tuple(items));
            }
            Op::TupleN(u8_n) => {
                let n = u8_n as usize;
                let tuple: Vec<Value> = self.stack.drain(self.stack.len() - n..).collect();
                assert_eq!(tuple.len(), n);
                self.stack.push(Value::Tuple(tuple))
            }
            Op::BinPersId => {
                let pid = self.stack.pop().unwrap();
                self.stack.push(Value::PersistentLoad(Box::new(pid)));
            }
            Op::True => self.stack.push(true.into()),
            Op::False => self.stack.push(false.into()),
            Op::Reduce => {
                let args = self.stack.pop().unwrap();
                let func = self.stack.pop().unwrap();

                if let Value::Function(func) = func {
                    let res = func.call(self, args)?;
                    self.stack.push(res);
                }
                else {
                    self.stack
                        .push(Value::Reduce(Box::new(func), Box::new(args)));
                }
            }
            Op::SetItem => {
                let value = self.stack.pop().unwrap();
                let key = self.stack.pop().unwrap();
                let last = self
                    .stack
                    .last_mut()
                    .ok_or(eyre!("Expected non-empty stack"))?;

                last.set_item(key, value)?;
            }
            Op::SetItems => {
                let items = self.pop_mark();

                if items.len() % 2 != 0 {
                    return Err(eyre!("SetItems must be an even number of values on stack"));
                }

                let last = self
                    .stack
                    .last_mut()
                    .ok_or(eyre!("Expected non-empty stack"))?;

                last.set_items(items.into_iter().tuples())?;
            }
            Op::Appends => {
                let items = self.pop_mark();
                let list = self.stack.last_mut().unwrap();
                list.extend(items)?;
            }
            Op::Build => {
                let state = self.stack.pop().unwrap();
                let last = self.stack.pop().unwrap();
                self.stack.push(Value::SetState(Box::new(last), Box::new(state)));
            }
            Op::Stop => {
                let val = self.stack.pop().ok_or(eyre!("Expected non-empty stack"))?;
                self.stop_value = Some(val);
            }
        }

        Ok(self.stop_value.is_some())
    }

    pub fn into_stop_value(self) -> Option<Value> {
        self.stop_value
    }
}
