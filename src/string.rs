/// Extensions for string values.
pub trait StrExt {
    /// Returns the provided string with the first letter capitalized.
    ///
    /// ```
    /// use rust_extensions::StrExt;
    ///
    /// let s = "hello";
    ///
    /// assert_eq!(s.capitalize(), "Hello");
    /// ```
    fn capitalize(&self) -> String;

    /// Returns `true` if `self` has a length larger than zero bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_extensions::StrExt;
    ///
    /// let s = "";
    /// assert!(!s.is_not_empty());
    ///
    /// let s = "not empty";
    /// assert!(s.is_not_empty());
    /// ```
    fn is_not_empty(&self) -> bool;
}

impl StrExt for String {
    fn capitalize(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            Some(first) => {
                let mut out = String::with_capacity(self.len());
                out.extend(first.to_uppercase());
                out.push_str(chars.as_str());
                out
            }
            None => String::new(),
        }
    }

    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

impl StrExt for &str {
    fn capitalize(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            Some(first) => {
                let mut out = String::with_capacity(self.len());
                out.extend(first.to_uppercase());
                out.push_str(chars.as_str());
                out
            }
            None => String::new(),
        }
    }

    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}
