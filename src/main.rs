extern crate rayon;
extern crate stopwatch;

use rayon::prelude::*;

macro_rules! time {
    ($e:expr) => {{
        let mut time = stopwatch::Stopwatch::start_new();
        let result = $e;
        time.stop();
        (time.elapsed(), result)
    }};
}

// Used to provide an assertion for the range of n
#[derive(Copy, Clone)]
struct Candidate(pub u64);

impl Candidate {
    fn new(n: u64) -> Self {
        assert!(n > 1);
        Candidate(n)
    }

    fn is_prime(&self) -> bool {
        match self.0 {
            2 => true,
            n => {
                if n & 1 == 0 {
                    return false;
                }

                let max = (n as f64).sqrt() as u64;
                for witness in 3..=max {
                    if n % witness == 0 {
                        return false;
                    }
                }

                true
            }
        }
    }
}

fn main() {
    let candidates: Vec<_> = (2..=25_000_000).map(Candidate::new).collect();
    let (time, count) = time!(
        candidates
            .par_iter()
            .filter(|x| x.is_prime())
            .count()
    );

    println!("{:?}: {}", time, count);

    let (time, count) = time!(
        (2..=25_000_000)
            .map(Candidate::new)
            .filter(Candidate::is_prime)
            .count()
    );

    println!("{:?}: {}", time, count);
}
