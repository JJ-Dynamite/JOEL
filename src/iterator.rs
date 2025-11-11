use crate::vm::Value;

/// Iterator trait for lazy evaluation
/// Note: This is a custom iterator trait, not std::iter::Iterator
pub trait JoelIterator {
    fn next(&mut self) -> Option<Value>;
    fn has_next(&self) -> bool;
}

/// Range iterator
pub struct RangeIterator {
    current: i64,
    end: i64,
    step: i64,
}

impl RangeIterator {
    pub fn new(start: i64, end: i64, step: i64) -> Self {
        Self {
            current: start,
            end,
            step,
        }
    }
}

impl JoelIterator for RangeIterator {
    fn next(&mut self) -> Option<Value> {
        if self.current < self.end {
            let value = Value::Number(self.current as f64);
            self.current += self.step;
            Some(value)
        } else {
            None
        }
    }
    
    fn has_next(&self) -> bool {
        self.current < self.end
    }
}

/// List iterator
pub struct ListIterator {
    items: Vec<Value>,
    index: usize,
}

impl ListIterator {
    pub fn new(items: Vec<Value>) -> Self {
        Self {
            items,
            index: 0,
        }
    }
}

impl JoelIterator for ListIterator {
    fn next(&mut self) -> Option<Value> {
        if self.index < self.items.len() {
            let value = self.items[self.index].clone();
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
    
    fn has_next(&self) -> bool {
        self.index < self.items.len()
    }
}

/// Generator iterator (simplified - would need full generator runtime)
pub struct GeneratorIterator {
    values: Vec<Value>,
    index: usize,
}

impl GeneratorIterator {
    pub fn new(values: Vec<Value>) -> Self {
        Self {
            values,
            index: 0,
        }
    }
}

impl JoelIterator for GeneratorIterator {
    fn next(&mut self) -> Option<Value> {
        if self.index < self.values.len() {
            let value = self.values[self.index].clone();
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
    
    fn has_next(&self) -> bool {
        self.index < self.values.len()
    }
}

/// Iterator utilities
pub struct IteratorUtils;

impl IteratorUtils {
    /// Map over iterator
    pub fn map<I, F>(iter: I, f: F) -> MapIterator<I, F>
    where
        I: JoelIterator,
        F: Fn(Value) -> Value,
    {
        MapIterator { iter, f }
    }
    
    /// Filter iterator
    pub fn filter<I, F>(iter: I, f: F) -> FilterIterator<I, F>
    where
        I: JoelIterator,
        F: Fn(&Value) -> bool,
    {
        FilterIterator { iter, f }
    }
    
    /// Take n items
    pub fn take<I>(iter: I, n: usize) -> TakeIterator<I>
    where
        I: JoelIterator,
    {
        TakeIterator { iter, count: 0, limit: n }
    }
}

struct MapIterator<I, F> {
    iter: I,
    f: F,
}

impl<I, F> JoelIterator for MapIterator<I, F>
where
    I: JoelIterator,
    F: Fn(Value) -> Value,
{
    fn next(&mut self) -> Option<Value> {
        self.iter.next().map(&self.f)
    }
    
    fn has_next(&self) -> bool {
        self.iter.has_next()
    }
}

struct FilterIterator<I, F> {
    iter: I,
    f: F,
}

impl<I, F> JoelIterator for FilterIterator<I, F>
where
    I: JoelIterator,
    F: Fn(&Value) -> bool,
{
    fn next(&mut self) -> Option<Value> {
        while let Some(item) = self.iter.next() {
            if (self.f)(&item) {
                return Some(item);
            }
        }
        None
    }
    
    fn has_next(&self) -> bool {
        self.iter.has_next()
    }
}

struct TakeIterator<I> {
    iter: I,
    count: usize,
    limit: usize,
}

impl<I> JoelIterator for TakeIterator<I>
where
    I: JoelIterator,
{
    fn next(&mut self) -> Option<Value> {
        if self.count < self.limit {
            self.count += 1;
            self.iter.next()
        } else {
            None
        }
    }
    
    fn has_next(&self) -> bool {
        self.count < self.limit && self.iter.has_next()
    }
}

