use std::iter::Iterator;
use std::time::{Duration, Instant};

use statistics::{max, mean, median, min, quartiles, standard_deviation, variance};

#[derive(Debug, Clone, PartialEq)]
enum BencherState {
    Warmup,
    Bench,
    Abort,
}

#[derive(Debug, Clone)]
pub struct Bencher {
    name: String,
    durations: Vec<Duration>,
    bench_min_dur: Option<Duration>,
    bench_min_iters: Option<u64>,
    warmup_dur: Option<Duration>,
    warmup_iters: Option<u64>,
    independent_variable: Option<u64>,
    state: BencherState,
    cpu_time: Duration,
    wall_time: Duration,
}

fn display(d: &Duration) -> String {
    let s = d.as_secs() as f64 + d.subsec_nanos() as f64 / 1e9;
    format!("{:.2}s", s)
}

impl Bencher {
    pub fn default(name: &str) -> Bencher {
        Bencher {
            name: name.to_owned(),
            durations: vec![],
            bench_min_iters: None,
            bench_min_dur: None,
            warmup_dur: None,
            warmup_iters: None,
            state: BencherState::Warmup,
            independent_variable: None,
            cpu_time: Duration::new(0,0),
            wall_time: Duration::new(0,0),
        }
    }

    /// Warmup for at least `secs` seconds.
    pub fn warmup_secs(&mut self, secs: u64) -> Bencher {
        self.warmup_dur = Some(Duration::from_secs(secs));
        self.clone()
    }

    /// Warmup for at least `i` runs.
    pub fn warmup_iters(&mut self, i: u64) -> Bencher {
        self.warmup_iters = Some(i);
        self.clone()
    }

    /// Bench for at least `secs` seconds.
    pub fn bench_min_secs(&mut self, secs: u64) -> Bencher {
        self.bench_min_dur = Some(Duration::from_secs(secs));
        self.clone()
    }

    /// Bench for at least `i` runs.
    pub fn bench_min_iters(&mut self, i: u64) -> Bencher {
        self.bench_min_iters = Some(i);
        self.clone()
    }

    pub fn independent_variable(&mut self, u: u64) -> Bencher {
        self.independent_variable = Some(u);
        self.clone()
    }

    pub fn manual_millis(&mut self, millis: u64) {
        match self.state {
            BencherState::Bench => self.durations.push(Duration::from_millis(millis)),
            _ => (),
        };
    }

    pub fn manual_dur(&mut self, d: Option<Duration>) {
        match d {
            None => self.state = BencherState::Abort,
            Some(d) => {
                if self.state == BencherState::Bench {
                    self.durations.push(d);
                }
            }
        }
    }

    pub fn abort_or_run<F>(&mut self, f: &F)
    where
        F: Fn(&mut Bencher),
    {
        match self.state.clone() {
            BencherState::Abort => panic!("Aborting benchmark!"),
            _ => f(self),
        }
    }

    pub fn run_manual<F>(&mut self, f: F)
    where
        F: Fn(&mut Bencher),
    {
        let mut num_warmups = 0;
        // run warmup for at least so long
        if let Some(warmup_dur) = self.warmup_dur {
            let warmup_start = Instant::now();
            while warmup_dur > warmup_start.elapsed() {
                // eprintln!("warmup run (minimum duration)");
                self.abort_or_run(&f);
                num_warmups += 1;
            }
        }

        // run at least so many warmup iters
        if let Some(warmup_iters) = self.warmup_iters {
            for i in num_warmups..warmup_iters {
                // eprintln!("warmup run (iters)");
                self.abort_or_run(&f);
            }
        }

        self.state = BencherState::Bench;

        // Bench for at least so long
        let mut num_benches = 0;
        if let Some(bench_min_dur) = self.bench_min_dur {
            let bench_start = Instant::now();
            while bench_min_dur > bench_start.elapsed() {
                let remaining = bench_min_dur - bench_start.elapsed();
                // eprintln!("benchmark run ({} left)", display(&remaining));
                self.abort_or_run(&f);
                num_benches += 1;
            }
        }

        // Bench for so many runs
        if let Some(bench_min_iters) = self.bench_min_iters {
            for i in num_benches..bench_min_iters {
                // eprintln!("benchmark run (minimum iters)");
                self.abort_or_run(&f);
            }
        }
    }

    pub fn iter<F>(&mut self, f: F)
    where
        F: Fn(),
    {
        unimplemented!();
    }

    pub fn summary(&self) -> Summary {
        let secs: Vec<f64> = self.durations
            .iter()
            .map(|d| d.as_secs() as f64 + d.subsec_nanos() as f64 / 1e9_f64)
            .collect();

        let quartiles = quartiles(&secs).unwrap_or_else(|| (0.0, 0.0, 0.0));
        let iqr = {
            let (q1, _, q3) = quartiles;
            q3 - q1
        };

        Summary {
            name: self.name.clone(),
            n: secs.len() as u64,
            mean: mean(&secs).unwrap_or_else(|| 0.0),
            min: min(&secs).unwrap_or_else(|| 0.0),
            max: max(&secs).unwrap_or_else(|| 0.0),
            median: median(&secs).unwrap_or_else(|| 0.0),
            quartiles: quartiles,
            iqr: iqr,
            var: variance(&secs, None).unwrap_or_else(|| 0.0),
            std_dev: standard_deviation(&secs, None).unwrap_or_else(|| 0.0),
            independent_variable: self.independent_variable,
        }
    }
}

impl Iterator for Bencher {
    type Item = i32;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<i32> {
        unimplemented!();
        // if self.num_iters == 0 {
        //     None
        // } else {
        //     self.num_iters -= 1;
        //     Some(0)
        // }
    }
}

#[derive(Debug, Serialize)]
pub struct Summary {
    name: String,
    /// Number of entries summarized
    n: u64,
    // sum: f64,
    min: f64,
    max: f64,
    mean: f64,
    median: f64,
    var: f64,
    std_dev: f64,
    // std_dev_pct: f64,
    // median_abs_dev: f64,
    // median_abs_dev_pct: f64,
    quartiles: (f64, f64, f64),
    // /// Interquartile Range
    iqr: f64,
    independent_variable: Option<u64>,
}

/*

fn routine(b: &mut Bencher) {
    // Setup (construct data, allocate memory, etc)

    b.iter(|| {
        // Code to benchmark goes here
    })

    // Teardown (free resources)
}

fn routine2(b: &mut Bencher) {
    // Setup (construct data, allocate memory, etc)

    // measure something
    b.manual_millis(2);

    // Teardown (free resources)
}

*/
