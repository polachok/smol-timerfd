use smol::{Timer, Async, Task};
use timerfd::{TimerFd, ClockId, TimerState, SetTimeFlags};
use std::time::{Instant, Duration};
use std::io::{Error, ErrorKind};

async fn sleep_with_timerfd(delay: Duration) -> Duration {
    let mut timerfd = TimerFd::new_custom(ClockId::Monotonic, true, true).unwrap();
    timerfd.set_state(TimerState::Oneshot(delay), SetTimeFlags::Default);
    let mut timerfd = Async::new(timerfd).unwrap();

    let start = Instant::now();
    timerfd.with_mut(|timer| {
        let n = timer.read();
        if n == 0 {
            Err(Error::new(ErrorKind::WouldBlock, "oh boy what a weird api"))
        } else {
            Ok(n)
        }
    }).await.unwrap();

    start.elapsed()
}

async fn sleep_with_timer(delay: Duration) -> Duration {
    let start = Instant::now();
    Timer::after(delay).await;
    start.elapsed()
}

fn main() {
    smol::run(async {
        let delay = Duration::from_micros(1);
        for _ in 0..10 {
            let slept = sleep_with_timer(delay).await;
            println!("timer: requested {:?} slept {:?}", delay, slept);
        }

        for _ in 0..10 {
            let slept = sleep_with_timerfd(delay).await;
            println!("timerfd: requested {:?} slept {:?}", delay, slept);
        }
    });
}
