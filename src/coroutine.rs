use crate::vm::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Coroutine runtime for cooperative multitasking
pub struct CoroutineRuntime {
    coroutines: Arc<Mutex<HashMap<usize, CoroutineState>>>,
    next_id: Arc<Mutex<usize>>,
}

impl CoroutineRuntime {
    pub fn new() -> Self {
        Self {
            coroutines: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(0)),
        }
    }
    
    /// Create a new coroutine
    pub fn create(&self) -> usize {
        let id = {
            let mut next = self.next_id.lock().unwrap();
            *next += 1;
            *next
        };
        
        let state = CoroutineState {
            id,
            stack: Vec::new(),
            variables: HashMap::new(),
            suspended: false,
            completed: false,
        };
        
        self.coroutines.lock().unwrap().insert(id, state);
        id
    }
    
    /// Suspend a coroutine
    pub fn suspend(&self, id: usize) -> Result<(), String> {
        if let Some(state) = self.coroutines.lock().unwrap().get_mut(&id) {
            state.suspended = true;
            Ok(())
        } else {
            Err(format!("Coroutine {} not found", id))
        }
    }
    
    /// Resume a coroutine
    pub fn resume(&self, id: usize) -> Result<Value, String> {
        if let Some(state) = self.coroutines.lock().unwrap().get_mut(&id) {
            if state.completed {
                return Err(format!("Coroutine {} already completed", id));
            }
            state.suspended = false;
            Ok(Value::None) // Simplified - would return actual value
        } else {
            Err(format!("Coroutine {} not found", id))
        }
    }
    
    /// Check if coroutine is suspended
    pub fn is_suspended(&self, id: usize) -> bool {
        if let Some(state) = self.coroutines.lock().unwrap().get(&id) {
            state.suspended
        } else {
            false
        }
    }
    
    /// Check if coroutine is completed
    pub fn is_completed(&self, id: usize) -> bool {
        if let Some(state) = self.coroutines.lock().unwrap().get(&id) {
            state.completed
        } else {
            false
        }
    }
    
    /// Cancel a coroutine
    pub fn cancel(&self, id: usize) -> Result<(), String> {
        if let Some(state) = self.coroutines.lock().unwrap().get_mut(&id) {
            state.completed = true;
            Ok(())
        } else {
            Err(format!("Coroutine {} not found", id))
        }
    }
}

/// Coroutine state
pub struct CoroutineState {
    pub id: usize,
    pub stack: Vec<HashMap<String, Value>>,
    pub variables: HashMap<String, Value>,
    pub suspended: bool,
    pub completed: bool,
}

