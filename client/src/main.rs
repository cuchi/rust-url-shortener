extern crate yew;
extern crate url_shortener_client;

use yew::prelude::*;
use url_shortener_client::SubmissionForm;

fn main() {
    yew::initialize();
    App::<SubmissionForm>::new().mount_to_body();
    yew::run_loop();
}
