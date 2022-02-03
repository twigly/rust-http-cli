use indicatif::{ProgressBar, ProgressStyle};

#[derive(Clone)]
pub struct Spinner {
    pb: ProgressBar,
}

impl Spinner {
    pub fn new() -> Spinner {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(120);
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&[".  ", ".. ", "...", " ..", "  .", " ..", "...", ".. "])
                .template("{msg} {spinner}"),
        );
        pb.set_message("Running");
        Spinner { pb }
    }

    pub fn done(self) {
        self.pb.finish_and_clear();
    }
}
