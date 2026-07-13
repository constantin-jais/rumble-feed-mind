#[cfg(feature = "web")]
fn main() {
    dioxus::launch(feedmind_app::App);
}

#[cfg(not(feature = "web"))]
fn main() {}
