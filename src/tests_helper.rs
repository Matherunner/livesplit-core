use crate::{TimeSpan, Timer, TimingMethod};

crate fn start_run(timer: &mut Timer) {
    timer.set_current_timing_method(TimingMethod::GameTime);
    timer.start();
    timer.initialize_game_time();
    timer.pause_game_time();
}

crate fn run_with_splits(timer: &mut Timer, splits: &[f64]) {
    start_run(timer);

    for &split in splits {
        timer.set_game_time(TimeSpan::from_seconds(split));
        timer.split();
    }

    timer.reset(true);
}

crate fn run_with_splits_opt(timer: &mut Timer, splits: &[Option<f64>]) {
    start_run(timer);

    for &split in splits {
        if let Some(split) = split {
            timer.set_game_time(TimeSpan::from_seconds(split));
            timer.split();
        } else {
            timer.skip_split();
        }
    }

    timer.reset(true);
}
