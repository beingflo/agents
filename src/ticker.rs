pub struct Ticker {
    // ticks since last fire
    ticks: usize,

    // every period-th tick should fire
    period: usize,

    // Fire on first tick always
    first: bool
}

impl Ticker {
    pub fn new(period: usize) -> Ticker {
        Ticker{ ticks: 0, period: period, first: true }
    }

    pub fn tick(&mut self) -> bool {
        if self.first {
            self.first = false;
            return true;
        }

        self.ticks += 1;

        if self.ticks >= self.period {
            self.ticks -= self.period;
            true
        } else {
            false
        }
    }
}
