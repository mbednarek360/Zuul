use cursive::Cursive;
use cursive::views::{Dialog, TextView};
mod core;

// init
fn main() {

    for room in core::room::get_rooms() {
        println!("{}", room.to_string());
    }


    /*
    let mut siv = Cursive::default();
    siv.add_layer(Dialog::around(
        TextView::new("Hello World")
    )
    .title("Title")
    .button("Quit", |s| {
        s.quit();
    })

    );
    siv.run();*/
}
