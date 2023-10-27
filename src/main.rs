use std::{thread::sleep, time::Duration, io::Write};

fn say(to_say: &str) {
    std::process::Command::new("say")
        .arg(to_say)
        .spawn()
        .expect("Say command should be installed");
}

fn wait(duration: Duration) {
    for i in 0..duration.as_secs() {
        print!("\r -- {:#?} --", Duration::from_secs(i));
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
}

fn minutes(minutes: u64) -> Duration {
    Duration::from_secs(minutes * 60)
}
fn main() {
    loop {
        say("Encoore du travai");
        wait(minutes(25));
        say("Travai terminay");
        wait(minutes(5));
    }
}
