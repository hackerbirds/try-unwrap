/// Trait for implementing `try_unwrap()` on the generic `Result<T, E>` type.
pub trait TryUnwrapResult {
    type T;
    type E;
    fn try_unwrap(self) -> Result<Self::T, Self::E>;
}

impl<T, E> TryUnwrapResult for Result<T, E> {
    type T = T;
    type E = E;

    /// Unwraps and returns the contained value if it is an `Ok`. 
    /// Otherwise, it returns an `Err`.
    /// ```
    /// let ok: Result<i32, ()> = Ok(3);
    /// assert_eq!(ok.try_unwrap(), Ok(3));
    /// let err: Result<(), i32> = Err(2);
    /// assert_eq!(err.try_unwrap(), Err(2));
    /// ```
    fn try_unwrap(self) -> Result<T, E> {
        if self.is_ok() {
            unsafe { Ok(self.unwrap_unchecked()) }
        } else {
            unsafe { Err(self.unwrap_err_unchecked()) }
        }
    }
}

/// Trait for implementing `try_unwrap()` on the generic `Option<T>` type.
pub trait TryUnwrapOption {
    type T;
    fn try_unwrap(self) -> Option<Self::T>;
}

impl<T> TryUnwrapOption for Option<T> {
    type T = T;

    /// Unwraps and returns the contained value if it is a `Some`. 
    /// Otherwise, it returns a `None`.
    /// ```
    /// let some: Option<i32> = Some(4);
    /// assert_eq!(some.try_unwrap(), Some(4));
    /// let none: Option<i32> = None;
    /// assert_eq!(none.try_unwrap(), None);
    /// ````
    fn try_unwrap(self) -> Option<T> {
        if self.is_some() {
            unsafe { Some(self.unwrap_unchecked()) }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result() {
        let ok: Result<i32, ()> = Ok(3);
        assert_eq!(ok.try_unwrap(), Ok(3));

        let err: Result<(), i32> = Err(2);
        assert_eq!(err.try_unwrap(), Err(2));
    }

    #[test]
    fn test_option() {
        let some: Option<i32> = Some(4);
        assert_eq!(some.try_unwrap(), Some(4));

        let none: Option<i32> = None;
        assert_eq!(none.try_unwrap(), None);
    }
}
