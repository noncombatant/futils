use std::cmp::PartialEq;

pub struct SubSlicer<'a, T: PartialEq> {
    pub slice: &'a [T],
    pub input_delimiter: &'a [T],
    pub start: usize,
}

impl<'a, T: PartialEq> Iterator for SubSlicer<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let i = find_subsequence(&self.slice[self.start..], self.input_delimiter);
        match i {
            Some(i) => {
                let sub_slice = &self.slice[self.start..self.start + i];
                self.start = self.start + i + self.input_delimiter.len();
                Some(sub_slice)
            }
            None => None,
        }
    }
}

// Cribbed from https://stackoverflow.com/posts/35907071/revisions. Thanks,
// Francis Gagné!
fn find_subsequence<T: PartialEq>(haystack: &[T], needle: &[T]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
