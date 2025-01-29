use std::time::{Duration, Instant};

pub struct ReportData {
    pub accuracy: Option<f64>,
    pub epoch: usize,
    pub epoch_count: usize,
    pub iteration: usize,
    pub iteration_count: usize,
}

pub struct Reporter {
    start_time: Instant,
}

impl Reporter {
    pub fn new() -> Reporter {
        Reporter {
            start_time: Instant::now(),
        }
    }

    pub fn report(&self, report_data: ReportData) {
        let ReportData {
            accuracy,
            epoch,
            epoch_count,
            iteration,
            iteration_count,
        } = report_data;

        let elapsed = self.start_time.elapsed();
        let progress = ((epoch - 1) * iteration_count + iteration) as f64
            / (epoch_count * iteration_count) as f64;
        let remaining =
            Duration::from_secs((elapsed.as_secs_f64() / progress) as u64 - elapsed.as_secs());

        print!("{esc}c", esc = 27 as char);
        println!("training...");
        println!("epoch:     {:0>3} / {:0>3}", epoch, epoch_count);
        println!("batch:     {:0>4} / {:0>4}", iteration, iteration_count);
        println!("progress:  {:.2?}%", progress * 100.0);
        println!("elapsed:   {}", format_duration(elapsed));
        println!("remaining: {}", format_duration(remaining));
        match accuracy {
            Some(a) => println!("accuracy:  {:.3}%", a * 100.0),
            _ => println!("accuracy:  ?? %"),
        }
    }
}

fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
