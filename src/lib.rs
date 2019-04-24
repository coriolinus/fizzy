use std::ops::Rem;

pub type MatchFn<T> = Box<dyn Fn(T) -> bool>;

pub struct Matcher<T> {
    matcher: MatchFn<T>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: AsRef<str>,
    {
        Matcher {
            matcher: Box::new(matcher),
            subs: subs.as_ref().to_string(),
        }
    }
}

pub struct FizzType<T>(pub Vec<Matcher<T>>);

impl<T> FizzType<T> {
    pub fn apply<I>(self, iter: I) -> Fizzy<T, I>
    where
        I: Iterator<Item = T>,
    {
        Fizzy {
            matchers: self,
            iter,
        }
    }
}

pub struct Fizzy<T, I> {
    matchers: FizzType<T>,
    iter: I,
}

impl<T, I> Fizzy<T, I>
where
    I: Iterator<Item = T>,
{
    pub fn wrap(iter: I) -> Fizzy<T, I> {
        Fizzy {
            matchers: FizzType(Vec::new()),
            iter: iter,
        }
    }

    pub fn add_matcher(&mut self, matcher: Matcher<T>) {
        let FizzType(ref mut matchers) = self.matchers;
        matchers.push(matcher);
    }
}

impl<T, I> Iterator for Fizzy<T, I>
where
    T: Copy + ToString,
    I: Iterator<Item = T>,
{
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let n = self.iter.next()?;
        let mut out = String::new();
        let FizzType(ref matchers) = self.matchers;

        for matcher in matchers {
            if (matcher.matcher)(n) {
                out += &matcher.subs.clone();
            }
        }
        if out.is_empty() {
            out = n.to_string();
        }
        Some(out)
    }
}

pub fn fizz_buzz<T>() -> FizzType<T>
where
    T: Copy + Default + From<u8> + PartialEq + Rem<Output = T> + 'static,
{
    let three: T = 3.into();
    let five: T = 5.into();

    FizzType(vec![
        Matcher::new(move |n| n % three == T::default(), "fizz"),
        Matcher::new(move |n| n % five == T::default(), "buzz"),
    ])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        let expect = vec![
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        let got = fizz_buzz().apply(1..=16).collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn test_fizz_buzz_u8() {
        let expect = vec![
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        let got = fizz_buzz().apply(1_u8..=16).collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn test_fizz_buzz_u64() {
        let expect = vec![
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        let got = fizz_buzz().apply(1_u64..=16).collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn test_fizz_buzz_nonsequential() {
        let collatz_12 = &[12, 6, 3, 10, 5, 16, 8, 4, 2, 1];
        let expect = vec![
            "fizz", "fizz", "fizz", "buzz", "buzz", "16", "8", "4", "2", "1",
        ];
        let got = fizz_buzz()
            .apply(collatz_12.into_iter().cloned())
            .collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn test_fizz_buzz_custom() {
        let expect = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "Bam", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "Bam", "BuzzFizz", "16",
        ];
        let mut fizzer = Fizzy::wrap(1..=16);
        fizzer.add_matcher(Matcher::new(|n| n % 5 == 0, "Buzz"));
        fizzer.add_matcher(Matcher::new(|n| n % 3 == 0, "Fizz"));
        fizzer.add_matcher(Matcher::new(|n| n % 7 == 0, "Bam"));
        let got = fizzer.collect::<Vec<_>>();
        assert_eq!(expect, got);
    }
}
