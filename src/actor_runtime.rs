use crate::ast::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Actor runtime for message-passing concurrency
pub struct ActorRuntime {
    actors: Arc<Mutex<HashMap<String, Arc<Mutex<ActorInstance>>>>>,
}

impl ActorRuntime {
    pub fn new() -> Self {
        Self {
            actors: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Create a new actor instance
    pub fn spawn(&self, name: String, actor_def: &Stmt) -> Result<ActorRef, String> {
        let instance = ActorInstance::new(actor_def)?;
        let instance_arc = Arc::new(Mutex::new(instance));
        
        {
            let mut actors = self.actors.lock().unwrap();
            actors.insert(name.clone(), instance_arc.clone());
        }
        
        // Start actor message loop
        let actors_clone = self.actors.clone();
        let name_clone = name.clone();
        thread::spawn(move || {
            ActorRuntime::actor_loop(name_clone, actors_clone);
        });
        
        Ok(ActorRef {
            name,
            runtime: self.actors.clone(),
        })
    }
    
    fn actor_loop(name: String, actors: Arc<Mutex<HashMap<String, Arc<Mutex<ActorInstance>>>>>) {
        loop {
            if let Some(actor) = actors.lock().unwrap().get(&name) {
                let mut actor_guard = actor.lock().unwrap();
                actor_guard.process_messages();
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
    
    /// Get actor reference by name
    pub fn get_actor(&self, name: &str) -> Option<ActorRef> {
        let actors = self.actors.lock().unwrap();
        if actors.contains_key(name) {
            Some(ActorRef {
                name: name.to_string(),
                runtime: self.actors.clone(),
            })
        } else {
            None
        }
    }
}

/// Reference to an actor for sending messages
#[derive(Debug, Clone)]
pub struct ActorRef {
    name: String,
    runtime: Arc<Mutex<HashMap<String, Arc<Mutex<ActorInstance>>>>>,
}

impl ActorRef {
    /// Send a message to the actor
    pub fn send(&self, method: String, args: Vec<Value>) -> Result<(), String> {
        if let Some(actor) = self.runtime.lock().unwrap().get(&self.name) {
            let mut actor_guard = actor.lock().unwrap();
            actor_guard.enqueue_message(Message {
                method,
                args,
            });
            Ok(())
        } else {
            Err(format!("Actor {} not found", self.name))
        }
    }
    
    /// Get actor name
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Actor instance with state and message queue
#[derive(Debug)]
pub struct ActorInstance {
    pub state: HashMap<String, Value>,
    message_queue: Vec<Message>,
    methods: Vec<Stmt>,
    pub supervisor: Option<String>, // Supervisor actor name
}

impl ActorInstance {
    fn new(actor_def: &Stmt) -> Result<Self, String> {
        if let Stmt::Actor { fields, methods, .. } = actor_def {
            let mut state = HashMap::new();
            
            // Initialize state from fields
            for (name, _type_annot, value) in fields {
                // Evaluate initial value (simplified - would need VM context)
                state.insert(name.clone(), Value::None);
            }
            
            Ok(Self {
                state,
                message_queue: Vec::new(),
                methods: methods.clone(),
                supervisor: None,
            })
        } else {
            Err("Not an actor definition".to_string())
        }
    }
    
    fn enqueue_message(&mut self, message: Message) {
        self.message_queue.push(message);
    }
    
    fn process_messages(&mut self) {
        while let Some(message) = self.message_queue.pop() {
            self.handle_message(message);
        }
    }
    
    fn handle_message(&mut self, message: Message) {
        // Find method and execute (simplified - would need full VM integration)
        for method in &self.methods {
            if let Stmt::Fn { name, .. } = method {
                if name == &message.method {
                    // Execute method with message args
                    // This would integrate with VM for full execution
                    // For now, just update state if needed
                }
            }
        }
    }
    
    /// Set supervisor for fault tolerance
    pub fn set_supervisor(&mut self, supervisor: String) {
        self.supervisor = Some(supervisor);
    }
    
    /// Handle actor failure (would notify supervisor)
    pub fn handle_failure(&mut self, error: String) {
        if let Some(ref supervisor) = self.supervisor {
            // Notify supervisor (simplified)
        }
    }
}

/// Message sent to an actor
#[derive(Debug)]
struct Message {
    method: String,
    args: Vec<Value>,
}

/// Runtime value types
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Actor(ActorRef),
    None,
}


