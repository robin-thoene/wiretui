use adapters::inbound::tui::App;
use std::io;

fn main() -> io::Result<()> {
    let mut tui_app = App::default();
    tui_app.run()
}

