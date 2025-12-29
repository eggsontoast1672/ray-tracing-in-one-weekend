use std::io::Write;

use colored::Colorize;

const PROGRESS_BAR_WIDTH: u32 = 50;

/// Print the progress so far.
pub fn update(percent_complete: f64) {
    // Enable colored output in stderr even when stdout is redirected.
    colored::control::set_override(true);

    let percent_complete = percent_complete.clamp(0.0, 1.0);

    let progress_message = format!("Progress: [{:3}%]", (percent_complete * 100.0) as i32);
    eprint!("\r{} [", progress_message.black().on_green());

    for cell in 1..=PROGRESS_BAR_WIDTH {
        if cell as f64 / PROGRESS_BAR_WIDTH as f64 <= percent_complete {
            eprint!("#");
        } else {
            eprint!(".");
        }
    }

    eprint!("]");

    std::io::stderr().flush().unwrap();
}

pub fn finish() {
    update(1.0);
    eprintln!();
}
