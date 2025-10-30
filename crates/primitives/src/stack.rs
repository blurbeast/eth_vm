use crate::errors::EvmErrors;
use alloy::primitives::B256;

#[derive(Debug, Clone, Default)]
pub struct Stack {
    pub data: Vec<B256>,
}

impl Stack {
    /// Push a value onto the stack.
    /// Returns `Err(EvmErrors::StackTooDeep)` if the stack would exceed 1024 items.
    pub fn push(&mut self, value: B256) -> Result<(), EvmErrors> {
        if self.data.len() >= 1024 {
            return Err(EvmErrors::StackTooDeep);
        }
        self.data.push(value);
        Ok(())
    }

    /// Pop a value from the stack. Returns `None` if the stack is empty.
    pub fn pop(&mut self) -> Option<B256> {
        self.data.pop()
    }

    /// Return current stack size. This is useful for testing and diagnostics.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Return whether the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::EvmErrors;
    use alloy::primitives::B256;

    /// Helper to create distinct B256 values for tests.
    /// Creates a 32-byte value with the last byte set to `n`.
    fn make_b256(n: u8) -> B256 {
        let mut bytes = [0u8; 32];
        bytes[31] = n;
        // Many B256 implementations provide `From<[u8; 32]>`.
        // This keeps tests readable and produces distinct values.
        B256::from(bytes)
    }

    #[test]
    fn push_pop_lifo_behavior() {
        let mut stack = Stack::default();

        let a = make_b256(1);
        let b = make_b256(2);
        let c = make_b256(3);

        // Push values in order a, b, c
        assert!(stack.push(a.clone()).is_ok());
        assert!(stack.push(b.clone()).is_ok());
        assert!(stack.push(c.clone()).is_ok());

        // Length should reflect number of pushes
        assert_eq!(stack.len(), 3);

        // Pop should follow LIFO: c, b, a
        assert_eq!(stack.pop(), Some(c));
        assert_eq!(stack.pop(), Some(b));
        assert_eq!(stack.pop(), Some(a));

        // Now empty
        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn pop_on_empty_returns_none() {
        let mut stack = Stack::default();
        // Ensure empty to start
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.pop(), None);

        // After pushing and popping everything, pop again returns None
        stack.push(make_b256(42)).unwrap();
        assert_eq!(stack.pop(), Some(make_b256(42)));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn push_enforces_max_depth() {
        let mut stack = Stack::default();

        // Fill the stack up to the limit (1024)
        for i in 0..1024 {
            let v = make_b256((i % 256) as u8);
            stack.push(v).expect("push within capacity should succeed");
        }

        // Stack is full; length should be 1024
        assert_eq!(stack.len(), 1024);

        // Pushing one more item should return the `StackTooDeep` error
        let result = stack.push(make_b256(0xff));
        assert!(matches!(result, Err(EvmErrors::StackTooDeep)));
    }
}
