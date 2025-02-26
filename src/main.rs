use free_will_cli::Predictor;
use std::io::{Write, stdin, stdout};
use termion::{color, event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    let mut keys = stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut predictor = Predictor::new(5);

    write!(stdout, "Press 'f' or 'j'\r\n").unwrap();
    stdout.flush().unwrap();

    loop {
        let Some(key) = keys.next().and_then(|key| key.ok()) else {
            continue;
        };

        let input = match key {
            Key::Char('f') => 'f',
            Key::Char('j') => 'j',
            Key::Ctrl('c') | Key::Char('\n') => break,
            _ => continue,
        };

        if let Some(result) = predictor.predict(input) {
            write!(
                stdout,
                "{}predicted: {result}, observed: {input}{}  Accuracy: {:.2}%\r\n",
                if result == input {
                    color::Fg(color::Red).to_string()
                } else {
                    String::new()
                },
                color::Fg(color::Reset),
                (predictor.correct_predictions as f64 / predictor.total_predictions as f64) * 100.0
            )
            .unwrap();
            stdout.flush().unwrap();
        };
    }
}
