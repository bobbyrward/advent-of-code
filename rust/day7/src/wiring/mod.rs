use std::collections::HashMap;


#[derive(Clone, Debug)]
pub enum NodeReference {
    Literal(u16),
    Named(String),
}


#[derive(Clone, Copy, Debug)]
pub enum Operation {
    DIRECT,
    NOT,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
}


#[derive(Clone, Debug)]
struct Connection {
    operation: Operation,
    lhs: NodeReference,
    rhs: Option<NodeReference>,
}



pub struct Circuit {
    connections: HashMap<String, Connection>,
    value_cache: HashMap<String, u16>,
}


impl Circuit {
    pub fn new() -> Circuit {
        Circuit {
            connections: HashMap::new(),
            value_cache: HashMap::new(),
        }
    }

    pub fn add_connection(&mut self, name: &str, op: Operation, lhs: NodeReference, rhs: Option<NodeReference>) {
        self.connections.insert(
            name.to_string(),
            Connection{
                operation: op,
                lhs: lhs,
                rhs: rhs,
            },
        );
    }

    pub fn cache_value(&mut self, name: &str, value: u16) {
        // println!("Stored {} for {}", value, name);
        self.value_cache.insert(name.to_string(), value);
    }

    pub fn resolve_reference(&mut self, reference: &NodeReference) -> u16 {
        let name = match reference {
            &NodeReference::Named(ref name) => name,
            &NodeReference::Literal(value) => return value,
        };

        match self.value_cache.get(name) {
            Some(&value) => return value,
            None => {},
        };

        let connection = match self.connections.get(name) {
            Some(connection) => connection.clone(),
            None => panic!("Connection {} not found", name),
        };

        let value = match connection.operation {
            Operation::DIRECT => self.resolve_reference(&connection.lhs),
            Operation::NOT => !self.resolve_reference(&connection.lhs),
            Operation::AND => {
                self.resolve_reference(&connection.lhs) & self.resolve_reference(&connection.rhs.unwrap().clone())
            }   ,
            Operation::OR => {
                self.resolve_reference(&connection.lhs) | self.resolve_reference(&connection.rhs.unwrap().clone())
            },
            Operation::LSHIFT => {
                self.resolve_reference(&connection.lhs) << self.resolve_reference(&connection.rhs.unwrap().clone())
            },
            Operation::RSHIFT => {
                self.resolve_reference(&connection.lhs) >> self.resolve_reference(&connection.rhs.unwrap().clone())
            },
        };

        self.cache_value(&name, value);

        value
    }
}
