use std::fmt;
use std::cmp;

use std::str::FromStr;

/// Struct `Interval` containing two values representing the limit of the interval.
///
/// The `Interval` is incluse which means that `Interval(0, 10)` is [0, 10].
/// The value 0 is supposed to be equals or greater than the second value.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Interval(u32, u32);

/// Struct `IntervalSet` representing a set of sorted not overllaping intervals.
/// Be aware that the validity of the interval set is not checked.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntervalSet {
    intervals: Vec<Interval>,
}

/// Struct to iterate through an `IntervalSet`
pub struct IntervalSetIterator<'a> {
    pos: usize,
    inner: &'a IntervalSet,
}

impl<'a> Iterator for IntervalSetIterator<'a> {
    type Item = &'a Interval;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.inner.intervals.len() {
            None
        } else {
            self.pos += 1;
            self.inner.intervals.get(self.pos - 1)
        }
    }
}

impl Interval {
    pub fn new(begin: u32, end: u32) -> Interval {
        let res = Interval(begin, end);
        if !res.is_valid() {
            panic!("Call constructor of Interval with invalid endpoints: Interval({}, {})",
                   begin,
                   end);
        }
        res
    }

    /// Return the maximum interval possible (with u32 var)
    pub fn whole() -> Interval {
        Interval(u32::min_value(), u32::max_value())
    }

    /// Because the trait Order is needed to sort the IntervalSet I dont what to change the
    /// native order. This function coud be considered as the `len` of the interval.
    pub fn range_size(&self) -> u32 {
        self.1 - self.0 + 1
    }

    /// Simply return an equivalent interval as tuple.
    pub fn as_tuple(&self) -> (u32, u32) {
        (self.0, self.1)
    }

    /// I am not sure about those two function, maybe set the field as public could be a better
    /// idea...
    pub fn get_inf(&self) -> u32 {
        self.0
    }

    pub fn get_sup(&self) -> u32 {
        self.1
    }

    /// Utility function check if the interval is valid.
    ///
    /// # Examples
    /// The following intervals are valids:
    ///
    /// ```
    /// use interval_set::Interval;
    /// Interval::new(0, 0);
    /// Interval::new(10, 100);
    /// ```
    ///
    /// The following intervals ae not valid:
    ///
    /// ```rust,should_panic
    /// use interval_set::Interval;
    /// Interval::new(10, 0);
    /// ```
    pub fn is_valid(&self) -> bool {
        self.0 <= self.1
    }
}

/// Trait `ToIntervalSet` allows to write a function to convert type into an IntervalSet.
pub trait ToIntervalSet {
    /// Consume `self` to create an IntervalSet
    fn to_interval_set(self) -> IntervalSet;
}

impl ToIntervalSet for Interval {
    /// Convert a simple interval into an intervalset.
    /// Note that the validity of the interval is checked.
    fn to_interval_set(self) -> IntervalSet {
        if self.is_valid() {
            IntervalSet { intervals: vec![self] }
        } else {
            panic!("CReate interval set from an unvalid interval");
        }
    }
}

impl ToIntervalSet for Vec<Interval> {
    /// Convert an array of interval into an intervalset.
    /// Note that the validity of the intervals are checked.
    ///
    /// # Example
    ///
    /// ```
    /// use interval_set::interval_set::ToIntervalSet;
    /// use interval_set::Interval;
    /// vec![Interval::new(5, 10), Interval::new(15, 20)].to_interval_set();
    /// ```
    fn to_interval_set(self) -> IntervalSet {
        let mut res: IntervalSet = IntervalSet::empty();
        for intv in self {
            if !intv.is_valid() {
                panic!("Invalid interval: {}-{}", intv.0, intv.1)
            }
            res.insert(intv);
        }
        res
    }
}

impl ToIntervalSet for Vec<(u32, u32)> {
    /// Convert an array of tuples into an intervalset.
    /// Note that the validity of the intervals are checked.
    ///
    /// # Example
    ///
    /// ```
    /// use interval_set::interval_set::ToIntervalSet;
    /// vec![(5, 10), (15, 20)].to_interval_set();
    /// ```
    fn to_interval_set(self) -> IntervalSet {
        let mut res: IntervalSet = IntervalSet::empty();
        for (begin, end) in self {
            if begin > end {
                panic!("Invalid interval: {}-{}", begin, end)
            }
            res.insert(Interval(begin, end));
        }
        res
    }
}

impl ToIntervalSet for String {
    /// Convert a string formatted into an
    /// interval set.
    /// The rules are simple for the string to be
    /// valid.
    /// - Each intervals are separated by a space.
    /// - Each bounds of the interval are separated by
    ///   a dash(-).
    /// - If an interval is of size 1, it is sufficient to
    ///   write only one integer.
    /// # Example
    /// ```
    /// use interval_set::interval_set::ToIntervalSet;
    /// use interval_set::Interval;
    /// let interval = String::from("3-4 7-19").to_interval_set();
    /// assert_eq!(interval, vec![(3, 4), (7, 19)].to_interval_set());
    ///
    /// let interval = String::from("3-4 6 7-19").to_interval_set();
    /// assert_eq!(interval, vec![(3, 4), (6, 6) ,(7, 19)].to_interval_set());
    ///
    ///
    /// let interval = String::from("3-4 7-19 6").to_interval_set();
    /// assert_eq!(interval, vec![(3, 4), (6, 6), (7, 19)].to_interval_set());
    ///
    ///
    /// let interval = String::from("3-4 7-19 6").to_interval_set();
    /// let interval_bis = String::from("3-3 4 7-7 8 9-19 6").to_interval_set();
    /// assert_eq!(interval, interval_bis);
    ///
    /// ```
    fn to_interval_set(self) -> IntervalSet {
        let mut iter = self.split_whitespace();
        let mut result = IntervalSet::empty();
        for interval in iter {
            // Handles the case where we have two specified bounds.
            if interval.contains("-") {
                // split by - and use map to transform the string into u32
                let bounds: Vec<u32> =
                    interval.split('-').map(|b| u32::from_str(b).unwrap()
                                            ).collect();

                let interval = Interval::new(bounds[0], bounds[1]);
                result = result.union(interval.to_interval_set());
            } else {
                let bound = u32::from_str(interval).unwrap();
                result = result.union(Interval::new(bound, bound).to_interval_set());
            }
        }
        result
    }
}

impl IntervalSet {
    /// Function to create an empty interval set.
    pub fn empty() -> IntervalSet {
        IntervalSet { intervals: vec![] }
    }

    /// Return `true` if the interval is empty.
    pub fn is_empty(&self) -> bool {
        self.intervals.len() == 0
    }

    /// Return the union of two intervals.
    ///
    /// # Example
    ///
    /// ```
    /// use interval_set::interval_set::ToIntervalSet;
    ///
    /// let a = vec![(5, 10)].to_interval_set();
    /// let b = vec![(15, 20)].to_interval_set();
    /// a.union(b); // [5-10, 15-20]
    /// ```
    pub fn union(self, rhs: IntervalSet) -> IntervalSet {
        self.merge(rhs, &|a, b| -> bool { a | b })
    }

    /// Return the intersection of two intervals.
    ///
    /// # Example
    ///
    /// ```
    /// use interval_set::interval_set::ToIntervalSet;
    ///
    /// let a = vec![(5, 10)].to_interval_set();
    /// let b = vec![(5, 10), (15, 20)].to_interval_set();
    /// a.intersection(b); //[5-10]
    /// ```
    pub fn intersection(self, rhs: IntervalSet) -> IntervalSet {
        self.merge(rhs, &|a, b| -> bool { a & b })
    }

    /// Return the difference between two intervals.
    ///
    /// # Example
    ///
    /// ```
    /// use interval_set::interval_set::ToIntervalSet;
    ///
    /// let a = vec![(5, 10), (15, 20)].to_interval_set();
    /// let b = vec![(5, 10)].to_interval_set();
    /// a.difference(b); //[15-20]
    /// ```
    pub fn difference(self, rhs: IntervalSet) -> IntervalSet {
        self.merge(rhs, &|a, b| -> bool { a & !b })
    }

    /// Return the symetric difference of two intervals.
    ///
    /// # Example
    ///
    /// ```
    /// use interval_set::interval_set::ToIntervalSet;
    ///
    /// let a = vec![(5, 10), (15, 20)].to_interval_set();
    /// let b = vec![(0, 10)].to_interval_set();
    /// a.difference(b); //[0-5, 15-20]
    /// ```
    pub fn symetric_difference(self, rhs: IntervalSet) -> IntervalSet {
        self.merge(rhs, &|a, b| -> bool { a ^ b })
    }

    /// Return the greater interval from the set.
    /// Note that the function return a cloned interval, so I will be easier to manipulate.
    /// Moreover, in the case where many intervals have the same size,
    /// the function will return the first element.
    /// # Example
    ///
    /// ```
    /// use interval_set::interval_set::ToIntervalSet;
    /// use interval_set::interval_set::IntervalSet;
    /// use interval_set::interval_set::Interval;
    ///
    /// let a = vec![(5, 10), (15, 25)].to_interval_set();
    /// let b = vec![(5, 10), (15, 20)].to_interval_set();
    /// let c = vec![(5, 10), (15, 20), (100, 1000)].to_interval_set();
    ///
    /// assert_eq!(a.max().unwrap(), Interval::new(15, 25));
    /// assert_eq!(b.max().unwrap(), Interval::new(5, 10));
    /// assert_eq!(c.max().unwrap(), Interval::new(100, 1000));
    /// assert_eq!(IntervalSet::empty().max(), None);
    ///
    /// ```
    pub fn max(&self) -> Option<Interval> {
        let mut max = usize::min_value();
        let mut res = None;

        if self.is_empty() {
            return None;
        }

        for intv in self.iter() {
            let curr_: usize = (intv.1 - intv.0) as usize;
            if curr_ > max {
                max = curr_ as usize;
                res = Some(intv.clone());
            }
        }
        res
    }

    /// Return the size of the interval set. The sie is defined by the sum of the len of each
    /// intervals contained into the set.
    ///
    /// # Example
    ///
    /// ```
    /// use interval_set::interval_set::ToIntervalSet;
    ///
    /// let a = vec![(5, 10), (15, 20)].to_interval_set();
    /// let b = vec![(0, 10), (15, 20)].to_interval_set();
    /// assert_eq!(a.size(), 12);
    /// assert_eq!(b.size(), 17);
    /// ```
    pub fn size(&self) -> u32 {
        if self.is_empty() {
            return 0;
        }
        self.iter().fold(0, |acc, ref x| acc + (x.range_size()))
    }

    /// Get an iterator over an IntervalSet
    ///
    /// # Example
    ///
    /// ```
    /// use interval_set::interval_set::ToIntervalSet;
    ///
    /// let a = vec![(5, 10), (15, 20)].to_interval_set();
    /// for intv in a.iter() {
    ///     let tuple = intv.as_tuple();
    ///     println!("{}--{}", tuple.0, tuple.1);
    /// }
    ///
    /// ```
    pub fn iter<'a>(&'a self) -> IntervalSetIterator<'a> {
        IntervalSetIterator {
            inner: self,
            pos: 0,
        }
    }

    /// Generate the (flat) list of interval bounds of the requested merge.
    /// The implementation is inspired by  http://stackoverflow.com/a/20062829.
    fn merge(self, rhs: IntervalSet, keep_operator: &Fn(bool, bool) -> bool) -> IntervalSet {
        if self.is_empty() & rhs.is_empty() {
            return self;
        }

        let mut lflat = self.flatten();
        let mut rflat = rhs.flatten();

        let sentinel: u32 = *cmp::max(lflat.iter().max(), rflat.iter().max()).unwrap() + 1;

        lflat.push(sentinel);
        rflat.push(sentinel);

        let mut ltail = lflat.iter().enumerate();
        let mut rtail = rflat.iter().enumerate();

        let mut res = vec![];

        //Because both vec are supposed to be sorted we could only take the min of vec[0].
        let mut scan: u32 = *cmp::min(lflat.iter().min(), rflat.iter().min()).unwrap();

        let mut lhead = ltail.next().unwrap();
        let mut rhead = rtail.next().unwrap();

        while scan < sentinel {
            let lin = !((scan < *lhead.1) ^ (lhead.0 % 2 != 0));
            let rin = !((scan < *rhead.1) ^ (rhead.0 % 2 != 0));

            let inres = keep_operator(lin, rin);

            if inres ^ (res.len() % 2 != 0) {
                res.push(scan);
            }

            if scan == *lhead.1 {
                lhead = match ltail.next() {
                    Some((lpos, lval)) => (lpos, lval),
                    _ => panic!("Deal with it braw"),
                };
            }
            if scan == *rhead.1 {
                rhead = match rtail.next() {
                    Some(rval) => rval,
                    _ => panic!("Deal with it braw"),
                };
            }
            scan = cmp::min(*lhead.1, *rhead.1);
        }
        IntervalSet::unflatten(res)
    }

    /// Generate a vector of endpoints.
    /// For example with the interval set `[0-5, 9-9,]`
    /// The resulting array would be: [0, 5, 9]
    fn flatten(self) -> Vec<u32> {
        let mut res = vec![];
        for intv in self.intervals {
            res.extend(vec![intv.0, intv.1 + 1]);
        }
        res
    }

    /// From an array of endpoints generate an `IntervalSet`.
    fn unflatten(vec: Vec<u32>) -> IntervalSet {
        let mut res: Vec<Interval> = Vec::new();
        let mut i = 0;
        while i < vec.len() {
            res.push(Interval(vec[i], vec[i + 1] - 1));
            i += 2;
        }
        res.to_interval_set()
    }

    pub fn insert(&mut self, element: Interval) {
        let mut newinf = element.0;
        let mut newsup = element.1;

        // Because we may remove one interval from self while we loop through its clone, we need to
        // adjuste the position.
        let mut idx_shift = 0;
        for (pos, intv) in self.intervals.clone().iter().enumerate() {
            if newinf > intv.1 + 1 {
                continue;
            }
            if newsup + 1 < intv.0 {
                break;
            }

            self.intervals.remove(pos - idx_shift);
            idx_shift += 1;

            newinf = cmp::min(newinf, intv.0);
            newsup = cmp::max(newsup, intv.1);
        }
        self.intervals.push(Interval::new(newinf, newsup));
        self.intervals.sort();
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 == self.1 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}-{}", self.0, self.1)
        }
    }
}

impl fmt::Display for IntervalSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (pos, interval) in self.intervals.iter().enumerate() {
            if pos == self.intervals.len() - 1 {
                f.write_fmt(format_args!("{}", interval))?;
            } else {
                f.write_fmt(format_args!("{} ", interval))?;
            }
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let empty_set = IntervalSet::empty();
        assert_eq!(format!("{}", empty_set), "");
    }

    fn assert_to_interval_set(tes_id: u32, v: Vec<(u32, u32)>, expected: IntervalSet) {
        assert_eq!(v.to_interval_set(), expected, "Test {} failed", tes_id);
    }

    #[test]
    fn test_to_interval_set() {
        let sym_cases =
            vec![(1, vec![(5, 10)], IntervalSet { intervals: vec![Interval(5, 10)] }),
                 (2, vec![(0, 100), (5, 10)], IntervalSet { intervals: vec![Interval(0, 100)] }),
                 (3,
                  vec![(1, 1), (2, 2), (3, 3), (4, 10), (10, 20)],
                  IntervalSet { intervals: vec![Interval(1, 20)] })];

        for (id, v, expected) in sym_cases {
            //assert_eq!(format!("test #{} of union", id), a, b, |x,y| x.union(y), expected);
            assert_to_interval_set(id, v, expected);
        }
    }

    fn assert_insertion(tes_id: u32, a: IntervalSet, element: Interval, expected: IntervalSet) {
        let mut a_ = a.clone();
        a_.insert(element);
        assert_eq!(a_, expected, "Test {} failed", tes_id);
    }

    #[test]
    fn test_insertion() {
        let sym_cases: Vec<(u32, IntervalSet, Interval, IntervalSet)> =
            vec![(1,
                  IntervalSet { intervals: vec![Interval(0, 0)] },
                  Interval(1, 1),
                  IntervalSet { intervals: vec![Interval(0, 1)] }),
                 (2,
                  IntervalSet { intervals: vec![Interval(0, 0), Interval(2, 2)] },
                  Interval(1, 1),
                  IntervalSet { intervals: vec![Interval(0, 2)] }),
                 (3,
                  IntervalSet { intervals: vec![Interval(0, 3)] },
                  Interval(1, 1),
                  IntervalSet { intervals: vec![Interval(0, 3)] }),
                 (4,
                  IntervalSet { intervals: vec![Interval(1, 1)] },
                  Interval(0, 3),
                  IntervalSet { intervals: vec![Interval(0, 3)] }),
                 (5,
                  IntervalSet { intervals: vec![Interval(0, 100)] },
                  Interval(1, 3),
                  IntervalSet { intervals: vec![Interval(0, 100)] }),
                 (6,
                  IntervalSet { intervals: vec![Interval(10, 20)] },
                  Interval(40, 80),
                  IntervalSet { intervals: vec![Interval(10, 20), Interval::new(40, 80)] })];

        for (id, a, element, expected) in sym_cases {
            //assert_eq!(format!("test #{} of union", id), a, b, |x,y| x.union(y), expected);
            assert_insertion(id, a, element, expected);
        }
    }

    #[test]
    fn test_flatten() {
        let simple_range = vec![Interval(0, 10)].to_interval_set();
        let disjoint = vec![Interval(0, 10), Interval(15, 20)].to_interval_set();
        assert_eq!(simple_range.flatten(), vec![0, 11]);
        assert_eq!(disjoint.flatten(), vec![0, 11, 15, 21]);
    }

    #[test]
    fn test_unflatten() {
        let simple_range = vec![0, 11];
        let disjoint = vec![0, 11, 15, 21];
        assert_eq!(IntervalSet::unflatten(disjoint),
                   vec![Interval(0, 10), Interval(15, 20)].to_interval_set());
        assert_eq!(IntervalSet::unflatten(simple_range),
                   vec![Interval(0, 10)].to_interval_set());
    }

    fn assert_difference(tes_id: u32, a: IntervalSet, b: IntervalSet, expected: IntervalSet) {
        assert_eq!(a.difference(b), expected, "Test {} failed", tes_id);
    }

    #[test]
    fn test_difference() {
        let sym_cases: Vec<(u32, IntervalSet, IntervalSet, IntervalSet)> =
            vec![(1,
                  vec![Interval(5, 10)].to_interval_set(),
                  vec![Interval(5, 10), Interval(15, 20)].to_interval_set(),
                  IntervalSet::empty()),
                 (2,
                  vec![(5, 10)].to_interval_set(),
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  IntervalSet::empty()),
                 (3,
                  IntervalSet::empty(),
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  IntervalSet::empty()),
                 (4,
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  IntervalSet::empty(),
                  vec![(5, 10), (15, 20)].to_interval_set()),
                 (5,
                  vec![(0, 100)].to_interval_set(),
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  vec![(0, 4), (11, 14), (21, 100)].to_interval_set()),
                 (6,
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  vec![(0, 100)].to_interval_set(),
                  IntervalSet::empty()),
                 (7, IntervalSet::empty(), IntervalSet::empty(), IntervalSet::empty())];

        for (id, a, b, expected) in sym_cases {
            //assert_eq!(format!("test #{} of union", id), a, b, |x,y| x.union(y), expected);
            assert_difference(id, a, b, expected);
        }
    }

    fn assert_intersection(tes_id: u32, a: IntervalSet, b: IntervalSet, expected: IntervalSet) {
        assert_eq!(a.intersection(b), expected, "Test {} failed", tes_id);
    }

    #[test]
    fn test_intersection() {
        let sym_cases: Vec<(u32, IntervalSet, IntervalSet, IntervalSet)> =
            vec![(1,
                  vec![Interval(5, 10)].to_interval_set(),
                  vec![Interval(5, 10), Interval(15, 20)].to_interval_set(),
                  vec![Interval(5, 10)].to_interval_set()),
                 (2,
                  vec![(5, 10)].to_interval_set(),
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  vec![(5, 10)].to_interval_set()),
                 (3,
                  IntervalSet::empty(),
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  IntervalSet::empty()),
                 (4,
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  IntervalSet::empty(),
                  IntervalSet::empty()),
                 (5,
                  vec![(0, 100)].to_interval_set(),
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  vec![(5, 10), (15, 20)].to_interval_set()),
                 (6, IntervalSet::empty(), IntervalSet::empty(), IntervalSet::empty())];

        for (id, a, b, expected) in sym_cases {
            //assert_eq!(format!("test #{} of union", id), a, b, |x,y| x.union(y), expected);
            assert_intersection(id, a, b, expected);
        }
    }

    fn assert_union(tes_id: u32, a: IntervalSet, b: IntervalSet, expected: IntervalSet) {
        assert_eq!(a.union(b), expected, "Test {} failed", tes_id);
    }

    #[test]
    fn test_union() {
        // Note: the first number is the test id, so it should be easy
        // to identify which test has failed.
        // The two first vectors are the operands and the expected result is last.
        let sym_cases: Vec<(u32, IntervalSet, IntervalSet, IntervalSet)> =
            vec![(1,
                  vec![Interval(5, 10)].to_interval_set(),
                  vec![Interval(5, 10), Interval(15, 20)].to_interval_set(),
                  vec![Interval(5, 10), Interval(15, 20)].to_interval_set()),
                 (2,
                  vec![(5, 10)].to_interval_set(),
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  vec![(5, 10), (15, 20)].to_interval_set()),
                 (3,
                  IntervalSet::empty(),
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  vec![(5, 10), (15, 20)].to_interval_set()),
                 (4,
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  IntervalSet::empty(),
                  vec![(5, 10), (15, 20)].to_interval_set()),
                 (5,
                  vec![(0, 100)].to_interval_set(),
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  vec![(0, 100)].to_interval_set()),
                 (6, IntervalSet::empty(), IntervalSet::empty(), IntervalSet::empty()),
                 (7,
                  vec![(0, 0), (2, 2), (3, 3)].to_interval_set(),
                  vec![(1, 1)].to_interval_set(),
                  vec![(0, 3)].to_interval_set())];

        for (id, a, b, expected) in sym_cases {
            //assert_eq!(format!("test #{} of union", id), a, b, |x,y| x.union(y), expected);
            assert_union(id, a, b, expected);
        }
    }

    fn assert_symetric_difference(tes_id: u32,
                                  a: IntervalSet,
                                  b: IntervalSet,
                                  expected: IntervalSet) {
        assert_eq!(a.symetric_difference(b), expected, "Test {} failed", tes_id);
    }

    #[test]
    fn test_symetric_difference() {
        let sym_cases: Vec<(u32, IntervalSet, IntervalSet, IntervalSet)> =
            vec![(1,
                  vec![Interval(5, 10)].to_interval_set(),
                  vec![Interval(5, 10), Interval(15, 20)].to_interval_set(),
                  vec![(15, 20)].to_interval_set()),
                 (2,
                  vec![(5, 10)].to_interval_set(),
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  vec![(15, 20)].to_interval_set()),
                 (3,
                  IntervalSet::empty(),
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  vec![(5, 10), (15, 20)].to_interval_set()),
                 (4,
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  IntervalSet::empty(),
                  vec![(5, 10), (15, 20)].to_interval_set()),
                 (5,
                  vec![(0, 100)].to_interval_set(),
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  vec![(0, 4), (11, 14), (21, 100)].to_interval_set()),
                 (6,
                  vec![(5, 10), (15, 20)].to_interval_set(),
                  vec![(0, 100)].to_interval_set(),
                  vec![(0, 4), (11, 14), (21, 100)].to_interval_set()),
                 (7, IntervalSet::empty(), IntervalSet::empty(), IntervalSet::empty())];

        for (id, a, b, expected) in sym_cases {
            //assert_eq!(format!("test #{} of union", id), a, b, |x,y| x.union(y), expected);
            assert_symetric_difference(id, a, b, expected);
        }
    }
}
