use std::io::Write;

use colored::Colorize;

/// Print the progress so far.
pub fn update(percent_complete: f64) {
    let progress_bar_width: u16 = termsize::get().unwrap().cols - 22;
    let percent_complete = percent_complete.clamp(0.0, 1.0);
    let progress_message = format!("Progress: [{:3}%]", (percent_complete * 100.0) as i32);

    eprint!("\r{} [", progress_message.black().on_green());

    for cell in 1..=progress_bar_width {
        if cell as f64 / progress_bar_width as f64 <= percent_complete {
            eprint!("█");
        } else {
            eprint!(" ");
        }
    }

    eprint!("]");

    std::io::stderr().flush().unwrap();
}

pub fn finish() {
    update(1.0);
    eprintln!();
}
