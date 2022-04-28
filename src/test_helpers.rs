use rand::Rng;

pub fn rand_array(n: usize, m: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();

    let mut a = std::iter::repeat_with(|| rng.gen_range(0..m))
        .take(n)
        .collect::<Vec<_>>();

    heap_sort(a.as_mut_slice());
    dedup_ints(a)
}

fn heap_sort(a: &mut [u32]) {
    let mut hi = a.len() as u32;

    if hi < 2 {
        return;
    }

    let mut lo = ((hi - 2) / 2) as i32;

    while lo >= 0 {
        sift_down(a, lo as usize, hi as usize);
        lo -= 1;
    }

    hi -= 1;

    while hi > 0 {
        a.swap(0, hi as usize);
        sift_down(a, 0, hi as usize);
        hi -= 1;
    }
}

fn sift_down(a: &mut [u32], mut lo: usize, hi: usize) {
    let mut pos = (lo * 2) + 1;
    let mut ext = pos + 1;

    while pos < hi {
        if ext < hi && a[pos] < a[ext] {
            pos += 1;
        }

        if a[lo] >= a[pos] {
            return;
        }

        a.swap(lo, pos);
        lo = pos;
        pos = (lo * 2) + 1;
        ext = pos + 1;
    }
}

fn dedup_ints(mut r: Vec<u32>) -> Vec<u32> {
    let n = r.len();

    if n < 2 {
        return r;
    }

    let mut i = 0;
    let mut j = 1;

    while j < n {
        if r[i] != r[j] {
            i += 1;
            r[i] = r[j];
        }

        j += 1;
    }

    i += 1;
    r.truncate(i);
    r
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_rand_array() {
        let a = rand_array(100, 1000);
        assert!(a.len() <= 100);
        let mut prev = 0;

        for (idx, av) in a.into_iter().enumerate() {
            if idx > 0 && prev >= av {
                panic!("{} >= {}", prev, av);
            }

            prev = av;
        }
    }
}
