// type system for layman

use crate::ast::{Type, BasicType, CompositeType};
use std::fmt;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Environment {
    pub variables: Mutex<HashMap<String, (Value, bool)>>,
    pub parent: Option<Arc<Environment>>,
}

impl fmt::Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Environment")
         .field("variables_count", &self.variables.lock().unwrap().len())
         .field("has_parent", &self.parent.is_some())
         .finish()
    }
}

impl Environment {
    pub fn new() -> Arc<Self> {
        Arc::new(Environment {
            variables: Mutex::new(HashMap::new()),
            parent: None,
        })
    }
    
    pub fn with_parent(parent: Arc<Environment>) -> Arc<Self> {
        Arc::new(Environment {
            variables: Mutex::new(HashMap::new()),
            parent: Some(parent),
        })
    }
    
    pub fn get(&self, name: &str) -> Option<Value> {
        self.variables.lock().unwrap().get(name).map(|(v, _)| v.clone())
            .or_else(|| self.parent.as_ref().and_then(|p| p.get(name)))
    }
    
    pub fn define(&self, name: String, value: Value, is_mutable: bool) {
        self.variables.lock().unwrap().insert(name, (value, is_mutable));
    }

    pub fn assign(&self, name: String, value: Value) -> Result<bool, String> {
        let is_mutable_opt = {
            let map = self.variables.lock().unwrap();
            map.get(&name).map(|(_, m)| *m)
        };

        if let Some(is_mutable) = is_mutable_opt {
            if !is_mutable {
                return Err(format!("Cannot reassign constant '{}'", name));
            }
            self.variables.lock().unwrap().insert(name, (value, true));
            return Ok(true);
        }
        
        if let Some(parent) = &self.parent {
            return parent.assign(name, value);
        }
        
        Ok(false)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeType {
    Number,
    String,  // changed from Text
    Bool,    // changed from Boolean
    Void,    // changed from Nothing
    List(Box<RuntimeType>),
    Dictionary {
        key: Box<RuntimeType>,
        value: Box<RuntimeType>,
    },
    Function {
        parameters: Vec<RuntimeType>,
        return_type: Box<RuntimeType>,
    },
    Class(String), // class name
    Object(String), // class name of object instance
    Future(Box<RuntimeType>),
    Variant(String), // type name (e.g. "Shape")
    Maybe(Box<RuntimeType>),
    Nothing,
    Any,
}

impl RuntimeType {
    #[allow(dead_code)]
    pub fn from_ast_type(ast_type: &Type) -> Self {
        match ast_type {
            Type::BasicType(basic) => match basic {
                BasicType::Number => RuntimeType::Number,
                BasicType::String => RuntimeType::String,
                BasicType::Bool => RuntimeType::Bool,
                BasicType::Void => RuntimeType::Void,
                BasicType::Any => RuntimeType::Any,
            },
            Type::CompositeType(composite) => match composite {
                CompositeType::List(inner) => RuntimeType::List(Box::new(Self::from_ast_type(inner))),
                CompositeType::DictionaryType(key, value) => RuntimeType::Dictionary {
                    key: Box::new(Self::from_ast_type(key)),
                    value: Box::new(Self::from_ast_type(value)),
                },
                CompositeType::Maybe(inner) => RuntimeType::Maybe(Box::new(Self::from_ast_type(inner))),
                _ => RuntimeType::Void,
            },
            Type::ClassType(class_name) => RuntimeType::Object(class_name.clone()),
            _ => RuntimeType::Void,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),  // changed from Text
    Bool(bool),      // changed from Boolean
    Void,            // changed from Nothing
    List(Arc<Mutex<Vec<Value>>>),
    Dictionary(Arc<Mutex<HashMap<String, Value>>>),
    Function {
        name: String,
        parameters: Vec<String>,
        body: Box<crate::ast::Node>,
        env: Option<Arc<Environment>>,
    },
    Object(Arc<Mutex<ObjectData>>),
    Class {
        name: String,
        methods: HashMap<String, crate::ast::FunctionDeclaration>,
    },
    Return(Box<Value>), // internal use only for control flow
    Future(Arc<Mutex<FutureState>>),
    Variant(String, String, HashMap<String, Value>), // TypeName, VariantName, Fields
    Nothing,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FutureState {
    Pending,
    Completed(Value),
    Failed(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectData {
    pub class_name: String,
    pub properties: HashMap<String, Value>,
}

impl Value {
    #[allow(dead_code)]
    pub fn get_type(&self) -> RuntimeType {
        match self {
            Value::Number(_) => RuntimeType::Number,
            Value::String(_) => RuntimeType::String,
            Value::Bool(_) => RuntimeType::Bool,
            Value::Void => RuntimeType::Void,
            Value::List(items) => {
                if items.lock().unwrap().is_empty() {
                    RuntimeType::List(Box::new(RuntimeType::Any))
                } else {
                    let inner_type = items.lock().unwrap()[0].get_type();
                    RuntimeType::List(Box::new(inner_type))
                }
            },
            Value::Dictionary(items) => {
                if items.lock().unwrap().is_empty() {
                    RuntimeType::Dictionary { key: Box::new(RuntimeType::Any), value: Box::new(RuntimeType::Any) }
                } else {
                    // infer from first item
                    let key_type = RuntimeType::String; // keys are always strings
                    let value_type = items.lock().unwrap().values().next().unwrap().get_type();
                    RuntimeType::Dictionary { key: Box::new(key_type), value: Box::new(value_type) }
                }
            },
            Value::Function { parameters: _, body: _, name: _, env: _ } => RuntimeType::Function { parameters: vec![], return_type: Box::new(RuntimeType::Any) }, // simplified
            Value::Object(data) => RuntimeType::Object(data.lock().unwrap().class_name.clone()),
            Value::Class { name, .. } => RuntimeType::Class(name.clone()),
            Value::Return(val) => val.get_type(),
            Value::Future(state) => {
                match &*state.lock().unwrap() {
                    FutureState::Completed(v) => RuntimeType::Future(Box::new(v.get_type())),
                    _ => RuntimeType::Future(Box::new(RuntimeType::Any)),
                }
            },
            Value::Variant(type_name, _, _) => RuntimeType::Variant(type_name.clone()),
            Value::Nothing => RuntimeType::Nothing,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Void => "void".to_string(),
            Value::List(items) => {
                let items_str: Vec<String> = items.lock().unwrap().iter().map(|v| v.to_string()).collect();
                format!("[{}]", items_str.join(", "))
            }
            Value::Dictionary(dict) => {
                let mut s = String::from("{");
                for (i, (k, v)) in dict.lock().unwrap().iter().enumerate() {
                    if i > 0 { s.push_str(", "); }
                    s.push_str(&format!("{}: {}", k, v.to_string()));
                }
                s.push('}');
                s
            }
            Value::Function { name, .. } => format!("function({})", name),
            Value::Object(data) => {
                let data = data.lock().unwrap();
                let props: Vec<String> = data.properties.iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{} object with {}", data.class_name, props.join(", "))
            },
            Value::Class { name, .. } => format!("class {}", name),
            Value::Return(val) => val.to_string(),
            Value::Future(_) => "future".to_string(),
            Value::Variant(_, variant_name, fields) => {
                let props: Vec<String> = fields.iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                if props.is_empty() {
                    variant_name.clone()
                } else {
                    format!("{} with {}", variant_name, props.join(", "))
                }
            },
            Value::Nothing => "nothing".to_string(),
        }
    }
}


impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Void, Value::Void) => true,
            (Value::List(a), Value::List(b)) => {
                let a_vec = a.lock().unwrap();
                let b_vec = b.lock().unwrap();
                *a_vec == *b_vec
            },
            (Value::Dictionary(a), Value::Dictionary(b)) => {
                let a_map = a.lock().unwrap();
                let b_map = b.lock().unwrap();
                *a_map == *b_map
            },
            (Value::Function { name: n1, .. }, Value::Function { name: n2, .. }) => n1 == n2,
            (Value::Object(a), Value::Object(b)) => {
                let a_data = a.lock().unwrap();
                let b_data = b.lock().unwrap();
                a_data.class_name == b_data.class_name // simplified equality
            },
            (Value::Variant(t1, v1, f1), 
             Value::Variant(t2, v2, f2)) => {
                t1 == t2 && v1 == v2 && f1 == f2
            },
            (Value::Class { name: n1, .. }, Value::Class { name: n2, .. }) => n1 == n2,
            (Value::Return(a), Value::Return(b)) => a == b,
            (Value::Future(a), Value::Future(b)) => Arc::ptr_eq(a, b),
            (Value::Nothing, Value::Nothing) => true,

            _ => false,
        }
    }
}
