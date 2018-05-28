use num::{cast, Float, Num, NumCast};
use std::cmp::Ordering::Less;

pub fn min<F>(f: &[F]) -> Option<F>
where
    F: Float,
{
    f.iter().fold(None, |min, x| match min {
        None => Some(*x),
        Some(min) => Some(if *x < min { *x } else { min }),
    })
}

pub fn max<F>(f: &[F]) -> Option<F>
where
    F: Float,
{
    f.iter().fold(None, |max, x| match max {
        None => Some(*x),
        Some(max) => Some(if *x > max { *x } else { max }),
    })
}

pub fn sum<F>(f: &[F]) -> Option<F>
where
    F: Float,
{
    f.iter().fold(None, |sum, x| match sum {
        None => Some(*x),
        Some(sum) => Some(*x + sum),
    })
}

pub fn mean<F>(f: &[F]) -> Option<F>
where
    F: Float,
{
    if let Some(s) = sum(f) {
        Some(s / cast(f.len()).unwrap())
    } else {
        None
    }
}

pub fn median<T>(t: &[T]) -> Option<T>
where
    T: Copy + Num + NumCast + PartialOrd,
{
    if t.len() == 0 {
        return None;
    }

    let mut sorted = vec![];
    sorted.extend(t.iter());
    sorted.sort_by(|a: &T, b: &T| a.partial_cmp(b).unwrap_or_else(|| Less));

    let i = t.len() / 2;
    if t.len() % 2 == 1 {
        Some(sorted[i])
    } else {
        Some((sorted[i] + sorted[i - 1]) / cast(2).unwrap())
    }
}

pub fn quartiles<T>(t: &[T]) -> Option<(T, T, T)>
where
    T: Copy + Num + NumCast + PartialOrd,
{
    let mut sorted = vec![];
    sorted.extend(t.iter());
    sorted.sort_by(|a: &T, b: &T| a.partial_cmp(b).unwrap_or_else(|| Less));

    let i = t.len() / 2;
    let (lower, upper) = if t.len() % 2 == 1 {
        (&sorted[0..i], &sorted[i + 1..])
    } else {
        (&sorted[0..i], &sorted[i..])
    };

    match (median(lower), median(&sorted), median(upper)) {
        (Some(q1), Some(q2), Some(q3)) => Some((q1, q2, q3)),
        _ => None,
    }
}

pub fn sum_square_deviations<T>(v: &[T], c: Option<T>) -> Option<T>
where
    T: Float,
{
    if v.len() == 0 {
        return None;
    }

    let c = match c {
        Some(c) => c,
        None => mean(v).unwrap(), // mean is defined if v.len() > 0
    };

    let sum = v.iter()
        .map(|x| (*x - c) * (*x - c))
        .fold(T::zero(), |acc, elem| acc + elem);

    Some(sum)
}

/// (Sample variance)[http://en.wikipedia.org/wiki/Variance#Sample_variance]
pub fn variance<T>(v: &[T], vbar: Option<T>) -> Option<T>
where
    T: Float,
{
    let sum = sum_square_deviations(v, vbar);
    match sum {
        Some(sum) => {
            let len: T = cast(v.len()).unwrap();
            Some(sum / (len - T::one()))
        }
        None => None,
    }
}

///  Standard deviation is a measure that is used to quantify the amount of variation or
///  dispersion of a set of data values. (reference)[http://en.wikipedia.org/wiki/Standard_deviation]
pub fn standard_deviation<T>(v: &[T], vbar: Option<T>) -> Option<T>
    where T: Float
{
    match variance(v, vbar) {
        Some(var) => Some(var.sqrt()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_test() {
        assert_eq!(median(&vec![1, 2]), Some(1));
        assert_eq!(median(&vec![1, 2, 3]), Some(2));
    }

    #[test]
    fn quartile_test() {
        assert!(quartiles(&vec![1]).is_none());
        assert_eq!(quartiles(&vec![1, 2]), Some((1, 1, 2)));
        assert_eq!(quartiles(&vec![1, 2, 3, 4]), Some((1, 2, 3)));
        assert_eq!(quartiles(&vec![1, 2, 3, 4, 5]), Some((1, 3, 4)));
    }
}
