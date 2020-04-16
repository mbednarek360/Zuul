use cursive::Cursive;
use cursive::views::{Dialog, TextView};

fn main() {
    let mut siv = Cursive::default();

    siv.add_layer(Dialog::around(





        TextView::new("Hello World")
    )


    .title("Title")
    .button("Quit", |s| {

        s.quit();

    })


    );
    siv.run();
}
