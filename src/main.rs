mod amp;
mod libamp;

use crate::amp::app::App;

fn main() {
    let mut app: App = Default::default();
    app.run();
}
