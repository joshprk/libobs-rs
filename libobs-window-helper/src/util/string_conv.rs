pub trait ToUtf8String {
    fn to_utf8(&self) -> String;
}

pub trait StrLen<T> {
    fn strlen(&self) -> usize;
    fn strslice(&self) -> &[T];
}

impl<T: Default + PartialEq> StrLen<T> for [T] {
    fn strlen(&self) -> usize {
        let zero = &Default::default();
        match self.iter().position(|x| x == zero) {
            None => self.len(),
            Some(x) => x,
        }
    }

    fn strslice(&self) -> &[T] {
        &self[0..self.strlen()]
    }
}

impl<T: AsRef<[u16]>> ToUtf8String for T {
    fn to_utf8(&self) -> String {
        String::from_utf16_lossy(self.as_ref().strslice())
            .trim_end_matches("\0")
            .to_string()
    }
}
