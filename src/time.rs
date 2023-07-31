#[derive(Debug)]
/// Measure the time between the .start and .stop functions, can be read later
pub enum Stopwatch {
    Running {
        start_time: std::time::Instant,
    },
    Paused {
        paused_since: std::time::Instant,
        runtime: std::time::Duration,
    },
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::Paused {
            paused_since: std::time::Instant::now(),
            runtime: std::time::Duration::from_secs(0),
        }
    }
}

impl Stopwatch {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn start_new() -> Self {
        Self::Running {
            start_time: std::time::Instant::now(),
        }
    }
    pub fn is_running(&self) -> bool {
        matches![self, Stopwatch::Running { .. }]
    }
    pub fn is_stopped(&self) -> bool {
        !self.is_running()
    }
    pub fn start(&mut self) {
        *self = Stopwatch::start_new();
    }
    pub fn stop(&mut self) {
        if let Self::Running { start_time } = self {
            *self = Stopwatch::Paused {
                paused_since: std::time::Instant::now(),
                runtime: start_time.elapsed(),
            }
        }
    }
    pub fn read(&self) -> std::time::Duration {
        match self {
            Stopwatch::Running { start_time } => start_time.elapsed(),
            Stopwatch::Paused { runtime, .. } => *runtime,
        }
    }
}

impl std::fmt::Display for Stopwatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", display_duration(self.read(), ""))
    }
}

pub fn display_duration(d: std::time::Duration, separator: &str) -> String {
    let mut value: f64 = d.as_nanos() as f64;
    // debug!("d:{:?}", d);
    // if nanos == 0 {}
    // debug!("nbr: {}", nbr);

    let units: Vec<&str> = vec!["ns", "µs", "ms", "s"];
    let mut name_index = 0;

    while value > 1_000. {
        if name_index < units.len() - 1 {
            value /= 1_000.;
            name_index += 1
        } else {
            break;
        }
    }

    format!("{:.2}{}{}", value, separator, units[name_index])
}

pub fn timeit<F: Fn() -> T, T>(f: F) -> (T, std::time::Duration) {
    //! Used to time the execution of a function with immutable parameters
    //! # Example
    //! ```
    //! let (output, duration) = timeit( || my_function() )
    //! ```
    let start = std::time::Instant::now();
    // let output = f();
    (f(), start.elapsed())
}

pub fn timeit_mut<F: FnMut() -> T, T>(mut f: F) -> (T, std::time::Duration) {
    //! Used to time the execution of a function with mutable parameters
    //! # Example
    //! ```
    //! let (output, duration) = timeit_mut( || my_function() )
    //! ```

    // the order of the output is important as it's also the order that it's cumputed
    // if you output (start.elapsed(), f()), the timer is stopped before the function actually starts
    // you'll need to compute f() before and store it in an ouput variable
    let start = std::time::Instant::now();
    // let output = f();
    (f(), start.elapsed())
}
