/// Extensions for boolean values.
pub trait BoolExt {
    /// Inverts the boolean value.
    ///
    /// ```
    /// use rust_extensions::BoolExt;
    ///
    /// let mut condition = false;
    /// condition.toggle();
    /// assert!(condition);
    /// ```
    fn toggle(&mut self);
}

impl BoolExt for bool {
    fn toggle(&mut self) {
        *self = !*self;
    }
}
