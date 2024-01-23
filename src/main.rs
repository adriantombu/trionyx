mod app;
mod scrape;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
