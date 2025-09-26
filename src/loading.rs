use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct AnimationState {
    running: bool,
    visible: bool,
}

pub struct LoadingAnimation {
    state: Arc<Mutex<AnimationState>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl LoadingAnimation {
    pub fn start() -> Self {
        let state = Arc::new(Mutex::new(AnimationState {
            running: true,
            visible: true,
        }));
        let state_clone = Arc::clone(&state);

        let handle = thread::spawn(move || {
            let mut dot_count = 0;

            loop {
                {
                    let state = state_clone.lock().unwrap();
                    if !state.running {
                        break;
                    }

                    if state.visible {
                        let dots = ".".repeat(dot_count);
                        print!("\r\x1b[36m🧵 Weaving{}\x1b[0m", dots);
                        io::stdout().flush().unwrap();
                    } else {
                        print!("\r\x1b[K");
                        io::stdout().flush().unwrap();
                    }
                }

                thread::sleep(Duration::from_secs(5));
                dot_count += 1;
            }

            print!("\r\x1b[K");
            io::stdout().flush().unwrap();
        });

        Self {
            state,
            handle: Some(handle),
        }
    }

    pub fn stop(mut self) {
        {
            let mut state = self.state.lock().unwrap();
            state.running = false;
        }

        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }

    pub fn pause(&self) {
        let mut state = self.state.lock().unwrap();
        state.visible = false;
    }

    pub fn resume(&self) {
        let mut state = self.state.lock().unwrap();
        state.visible = true;
    }
}
