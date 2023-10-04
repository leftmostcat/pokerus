use core::{cell::UnsafeCell, ptr};

/// The inner representation of lazily-loaded data in its several states.
enum LazyState<F, T, E> {
    /// The data in its raw state, as provided on creation.
    Raw(F),

    /// The provided data when processing has failed.
    Invalid(F, E),

    /// The processed representation of the data.
    Processed(T),
}

impl<F, T, E> LazyState<F, T, E> {
    fn mark_invalid(&mut self, error: E) {
        unsafe {
            let this: *mut Self = self;
            ptr::write(
                this,
                match ptr::read(this) {
                    LazyState::Raw(value) => LazyState::Invalid(value, error),
                    other => other,
                },
            );
        }
    }
}

/// `LazyData` is a mechanism for processing data from a stored form into a
/// more useful in-memory representation on demand but without repeated
/// processing.
// This is inspired by and based on the design of `OnceCell`, swapping the
// internal `Option` for an enum which directly encodes the possible states of
// loading while maintaining the requirement that the underlying data can only
// be referenced once the decoded form has been realized and only directly
// modified with a mutable reference.
pub(crate) struct LazyData<F, T, E> {
    // INVARIANT: Inner cell will only be written to when in the "raw" state or
    // if there is a mutable reference to the struct, and cannot be returned to
    // the raw state. Because it is not `Send`, only one mutable pointer to the
    // inner state can exist at any one time and an external reference to it
    // will only exist in the case that decoding has already successfully
    // occurred.
    inner: UnsafeCell<LazyState<F, T, E>>,
}

impl<F, T, E> LazyData<F, T, E> {
    /// Gets the processed representation of the data, or attempts to process it
    /// if it has not already been processed.
    pub fn get_or_try_process<P>(&self, process: P) -> Result<&T, &E>
    where
        P: for<'a> FnOnce(&'a F) -> Result<T, E>,
    {
        let result = if let LazyState::Raw(data) = unsafe { &*self.inner.get() } {
            Some(process(data))
        } else {
            None
        };

        if let Some(result) = result {
            match result {
                Ok(processed) => {
                    match unsafe { &*self.inner.get() } {
                        LazyState::Raw(_) => {
                            // SAFETY: The inner cell will only be written to from a raw
                            // state and cannot be used concurrently.
                            let slot = unsafe { &mut *self.inner.get() };
                            *slot = LazyState::Processed(processed);
                        }
                        LazyState::Invalid(_, _) => {
                            panic!("Tried to set processed value of invalid data")
                        }
                        LazyState::Processed(_) => {
                            panic!("Tried to set processed value of already-processed data")
                        }
                    }
                }
                Err(err) => {
                    unsafe { &mut *self.inner.get() }.mark_invalid(err);

                    return Err(self.unwrap_err());
                }
            }
        }

        Ok(self.unwrap())
    }

    /// Sets a new already-processed value.
    pub fn _set(&mut self, value: T) {
        let slot = self.inner.get_mut();
        *slot = LazyState::Processed(value);
    }

    /// Gets a reference to the processed value. Panics if processing has not
    /// occurred or has failed.
    fn unwrap(&self) -> &T {
        match unsafe { &*self.inner.get() } {
            LazyState::Processed(value) => value,
            _ => panic!("Unwrapping unprocessed lazy data"),
        }
    }

    fn unwrap_err(&self) -> &E {
        match unsafe { &*self.inner.get() } {
            LazyState::Invalid(_, err) => err,
            _ => panic!("Unwrapping error on unprocessed or valid lazy data"),
        }
    }
}

impl<F, T, E> From<F> for LazyData<F, T, E> {
    fn from(value: F) -> Self {
        Self {
            inner: UnsafeCell::new(LazyState::Raw(value)),
        }
    }
}
