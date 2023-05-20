mod libamp;
mod amp;

use crate::amp::app::App;

fn main() {
    let mut app: App = Default::default();
    app.run();
}
