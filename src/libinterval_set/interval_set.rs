use std::fmt;
use std::cmp;

#[derive(Debug, Eq, PartialEq)]
pub struct Interval(u32, u32);

#[derive(Debug, Eq, PartialEq)]
pub struct IntervalSet {
    intervals: Vec<Interval>
}

impl Interval {
    pub fn whole() -> Interval {
        Interval(u32::min_value(), u32::max_value())
    }
}

impl IntervalSet {

    pub fn empty() -> IntervalSet {
        IntervalSet{
            intervals: vec![]
        }
    }

    pub fn from_vec(vec: Vec<Interval>) -> IntervalSet {
        IntervalSet {
            intervals: vec
        }
    }

    pub fn union(self, rhs: IntervalSet) -> IntervalSet {
        self.merge(rhs, &|a, b| -> bool {a|b})
    }

    pub fn intersection(self, rhs: IntervalSet) -> IntervalSet {
        self.merge(rhs, &|a, b| -> bool {a & b})
    }

    fn merge(self, rhs: IntervalSet, keep_operator: &Fn(bool, bool) -> bool ) -> IntervalSet {
        let mut lflat = self.flatten();
        let mut rflat = rhs.flatten();

        let sentinel : u32 = *cmp::max(lflat.iter().max(), rflat.iter().max()).unwrap() + 1;

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
                    Some( (lpos, lval) ) => (lpos, lval),
                    _ => panic!("Deal with it braw")
                };
            }
            if scan == *rhead.1 {
                rhead = match rtail.next() {
                    Some(rval) => rval,
                    _ => panic!("Deal with it braw")
                };
            }
            scan = cmp::min(*lhead.1, *rhead.1);
        }
        IntervalSet::unflatten(res)
    }

    fn flatten(self) -> Vec<u32> {
        let mut res = vec![];
        for intv  in self.intervals {
            res.extend(vec![intv.0, intv.1 + 1]);
        }
        res
    }

    fn unflatten(vec: Vec<u32>) -> IntervalSet {
        println!("unflatten: {:?}", vec);
        let mut res :  Vec<Interval> = Vec::new();
        let mut i = 0;
        while i < vec.len() {
            res.push(Interval(vec[i], vec[i+1] - 1));
            i += 2;
        }
        IntervalSet::from_vec(res)
    }

}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{})", self.0, self.1)
    }
}

impl fmt::Display for IntervalSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("{")?;
        for interval in &self.intervals {
            f.write_fmt(format_args!("{}", interval))?;
        }
        f.write_str("}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let empty_set = IntervalSet::empty();
        assert_eq!(format!("{}", empty_set), "{}");
    }

    #[test]
    fn test_from_vec() {
        let from_vec = IntervalSet::from_vec(vec![Interval(0, 32)]);
        let empty_from_vec = IntervalSet::from_vec(vec![]);
        let empty_set = IntervalSet::empty();
        assert!(from_vec != empty_set);
        assert!(empty_from_vec == empty_set);
    }

    #[test]
    fn test_flatten() {
        let simple_range = IntervalSet::from_vec(vec![Interval(0, 10)]);
        let disjoint = IntervalSet::from_vec(vec![Interval(0, 10), Interval(15, 20)]);
        assert_eq!(simple_range.flatten(), vec![0, 11]);
        assert_eq!(disjoint.flatten(), vec![0, 11, 15, 21]);
    }

    #[test]
    fn test_unflatten() {
        let simple_range = vec![0, 11];
        let disjoint = vec![0, 11, 15, 21] ;
        assert_eq!(IntervalSet::unflatten(disjoint), IntervalSet::from_vec(vec![Interval(0, 10), Interval(15, 20)]));
        assert_eq!(IntervalSet::unflatten(simple_range), IntervalSet::from_vec(vec![Interval(0, 10)]));
    }

    #[test]
    fn test_interval() {
    }

    #[test]
    fn test_intersection() {
        let sym_cases = vec![
            (26, IntervalSet::from_vec(vec![Interval(5, 10)]), IntervalSet::from_vec(vec![Interval(5, 10), Interval(15, 20)]), IntervalSet::from_vec(vec![Interval(5, 10)]))
        ];

        for (id, a, b, expected) in sym_cases {
            //assert_eq!(format!("test #{} of union", id), a, b, |x,y| x.union(y), expected);
            assert_eq!(a.intersection(b), expected);
        }
    }

    #[test]
    fn test_union() {
    // Note: the first number is the test id, so it should be easy to identify which test has failed.
    // The two first vectors are the operands and the expected result is last.
    let sym_cases = vec![
      (26, IntervalSet::from_vec(vec![Interval(5, 10)]), IntervalSet::from_vec(vec![Interval(5, 10), Interval(15, 20)]), IntervalSet::from_vec(vec![Interval(5, 10), Interval(15, 20)]))
    ];

    for (id, a, b, expected) in sym_cases {
      //assert_eq!(format!("test #{} of union", id), a, b, |x,y| x.union(y), expected);
      assert_eq!(a.union(b), expected);
    }
  }
}
