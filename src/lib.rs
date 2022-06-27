#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]

//! ...

use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

/// ...
pub fn pair<T>() -> (Verifier<T>, Caller<T>) {
    let calls = Arc::new(Mutex::new(Some(Vec::new())));

    let matcher = Verifier {
        calls: calls.clone(),
    };

    let spy = Caller { calls };

    (matcher, spy)
}

/// ...
pub struct Caller<T> {
    calls: Arc<Mutex<Option<Vec<T>>>>,
}

impl<T> Caller<T> {
    /// ...
    pub fn call(&self, value: T) {
        let mut guard = self.calls.lock().unwrap();

        match guard.as_mut() {
            Some(calls) => calls.push(value),
            None => panic!("verify_call received a call after the verifier was consumed"),
        }
    }
}

impl<T: Debug> Debug for Caller<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Caller")
            .field("calls", &self.calls)
            .finish()
    }
}

/// ...
pub struct Verifier<T> {
    calls: Arc<Mutex<Option<Vec<T>>>>,
}

impl<T> Verifier<T> {
    /// ...
    pub fn calls(self) -> Vec<T> {
        let mut guard = self.calls.lock().unwrap();
        guard.take().unwrap()
    }
}

impl<T: Debug> Debug for Verifier<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Verifier")
            .field("calls", &self.calls)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_verifying_never_called() {
        // Given
        let (verifier, _caller) = pair::<()>();

        // When
        let calls = verifier.calls();

        // Then
        assert_eq!(calls.len(), 0);
    }

    #[test]
    fn should_pass_verifying_calls() {
        // Given
        let (verifier, caller) = pair();
        caller.call(1);
        caller.call(2);
        caller.call(3);

        // When
        let calls = verifier.calls();

        // Then
        assert_eq!(calls, &[1, 2, 3]);
    }

    #[test]
    #[should_panic(expected = "verify_call received a call after the verifier was consumed")]
    fn should_panic_when_call_after_consuming_verifier() {
        // Given
        let (verifier, caller) = pair();
        let _calls = verifier.calls();

        // When
        caller.call(3);
    }

    #[test]
    fn should_be_thread_safe() {
        // Given
        let (verifier, caller) = pair();
        let handle = std::thread::spawn(move || {
            caller.call(1);
            caller.call(2);
            caller.call(3);
        });

        // When
        handle.join().unwrap();
        let calls = verifier.calls();

        // Then
        assert_eq!(calls, &[1, 2, 3]);
    }

    #[test]
    fn should_implement_traits() {
        use impls::impls;
        use std::fmt::Debug;

        assert!(impls!(Caller<i32>: Debug & Send & Sync & !Clone));
        assert!(impls!(Verifier<i32>: Debug & Send & Sync & !Clone));

        struct NotDebug;
        assert!(impls!(Caller<NotDebug>: !Debug & Send & Sync & !Clone));
        assert!(impls!(Verifier<NotDebug>: !Debug & Send & Sync & !Clone));
    }
}
