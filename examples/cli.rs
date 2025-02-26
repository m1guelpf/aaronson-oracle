use aaronson_oracle::{Choice, Predictor};
use std::io::{Write, stdin, stdout};
use termion::{color, event::Key, input::TermRead, raw::IntoRawMode};

const KEYS: [char; 2] = ['f', 'j'];

fn main() {
    let mut predictor = Predictor::new(5);

    let mut keys = stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "Press 'f' or 'j'\r\n").unwrap();
    stdout.flush().unwrap();

    loop {
        let Some(key) = keys.next().and_then(|key| key.ok()) else {
            continue;
        };

        let input = match key {
            Key::Char('f') => Choice::Left,
            Key::Char('j') => Choice::Right,
            Key::Ctrl('c') | Key::Char('\n') => break,
            _ => continue,
        };

        if let Some(result) = predictor.predict(input) {
            write!(
                stdout,
                "{}predicted: {}, observed: {}{}  Accuracy: {:.2}%\r\n",
                if result == input {
                    color::Fg(color::Red).to_string()
                } else {
                    String::new()
                },
                result.display(KEYS),
                input.display(KEYS),
                color::Fg(color::Reset),
                (predictor.correct_predictions as f64 / predictor.total_predictions as f64) * 100.0
            )
            .unwrap();
            stdout.flush().unwrap();
        };
    }
}
