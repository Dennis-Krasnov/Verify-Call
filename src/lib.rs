use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

pub fn spy<T>() -> (Matcher<T>, Spy<T>) {
    let state = Arc::new(Mutex::new(SpyState {
        calls: VecDeque::new(),
        finished: false,
    }));

    let matcher = Matcher {
        state: state.clone(),
    };

    let spy = Spy { state };

    (matcher, spy)
}

struct SpyState<T> {
    calls: VecDeque<T>,
    finished: bool,
}

pub struct Spy<T> {
    state: Arc<Mutex<SpyState<T>>>,
}

impl<T> Spy<T> {
    pub fn call(&self, value: T) {
        let mut guard = self.state.lock().unwrap();

        if guard.finished {
            panic!("...");
        }

        guard.calls.push_back(value);
    }
}

#[must_use]
pub struct Matcher<T> {
    state: Arc<Mutex<SpyState<T>>>,
}

impl<T: PartialEq + Debug> Matcher<T> {
    pub fn called_with(self, value: T) -> Self {
        {
            let mut guard = self.state.lock().unwrap();

            let call = guard.calls.pop_front().expect("...");
            assert_eq!(call, value);
        }

        self
    }

    pub fn called_matching(self, predicate: impl FnOnce(T) -> bool) -> Self {
        {
            let mut guard = self.state.lock().unwrap();

            let call = guard.calls.pop_front().expect("...");
            assert!(predicate(call));
        }

        self
    }

    pub fn called_no_more(self) {
        let mut guard = self.state.lock().unwrap();

        assert!(!guard.finished);
        guard.finished = true;

        if !guard.calls.is_empty() {
            panic!("...");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with() {
        // Given
        let (matcher, spy) = spy();

        // When
        spy.call(1);
        spy.call(2);

        // Then
        let _ = matcher
            .called_with(1)
            .called_with(2);
    }

    #[test]
    #[should_panic]
    fn call_with_missing() {
        // Given
        let (matcher, spy) = spy();

        // When
        spy.call(1);

        // Then
        let _ = matcher
            .called_with(1)
            .called_with(2);
    }

    #[test]
    #[should_panic]
    fn call_with_not_match() {
        // Given
        let (matcher, spy) = spy();

        // When
        spy.call(1);
        spy.call(3);

        // Then
        let _ = matcher
            .called_with(1)
            .called_with(2);
    }

    #[test]
    fn called_matching() {
        // Given
        let (matcher, spy) = spy();

        // When
        spy.call(1);
        spy.call(2);

        // Then
        let _ = matcher
            .called_matching(|n| n % 2 == 1)
            .called_matching(|n| n % 2 == 0);
    }

    #[test]
    #[should_panic]
    fn called_matching_missing() {
        // Given
        let (matcher, spy) = spy();

        // When
        spy.call(1);

        // Then
        let _ = matcher
            .called_matching(|n| n % 2 == 1)
            .called_matching(|n| n % 2 == 0);
    }

    #[test]
    #[should_panic]
    fn called_matching_not_match() {
        // Given
        let (matcher, spy) = spy();

        // When
        spy.call(1);
        spy.call(3);

        // Then
        let _ = matcher
            .called_matching(|n| n % 2 == 1)
            .called_matching(|n| n % 2 == 0);
    }

    #[test]
    fn call_no_more_immediate() {
        // Given
        let (matcher, spy) = spy::<()>();

        // When
        matcher.called_no_more();
    }

    #[test]
    fn call_no_more_after_other_matches() {
        // Given
        let (matcher, spy) = spy();

        // When
        spy.call(1);
        spy.call(2);

        // Then
        matcher
            .called_with(1)
            .called_matching(|n| n == 2)
            .called_no_more();
    }

    #[test]
    #[should_panic]
    fn call_after_no_more() {
        // Given
        let (matcher, spy) = spy();
        matcher.called_no_more();

        // When
        spy.call(1);
    }

    #[test]
    #[should_panic]
    fn too_many_calls_for_called_no_more() {
        // Given
        let (matcher, spy) = spy();
        spy.call(1);

        // When
        matcher.called_no_more();
    }

    // TODO: test that it works cross-thread
}
