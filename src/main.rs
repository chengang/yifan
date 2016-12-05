extern crate rocky;
use rocky::Rocky;

mod app;
use app::*;

fn main() {
    let mut rocky = Rocky::new("0.0.0.0", 4100);
    rocky.router.get("/", index::index);
    rocky.router.get("/post/new", post::new);
    rocky.run();
}
