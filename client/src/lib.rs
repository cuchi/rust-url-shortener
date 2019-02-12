extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;
use yew::services::ConsoleService;

pub struct SubmissionForm {
    console: ConsoleService,
    submitted: bool,
    shortened_url: Option<String>,
    url: String
}

pub enum Msg {
    Submit,
    GotUrl(String)
}

impl Component for SubmissionForm {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        SubmissionForm {
            console: ConsoleService::new(),
            submitted: false,
            shortened_url: None,
            url: String::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit => {
                self.console.log(&self.url);
                self.submitted = true;

                // TODO: Find a way to call our API
                let result = format!("Url to be shortened: {}", self.url);
                self.shortened_url = Some(result);
                true
            }
            Msg::GotUrl(url) => {
                self.url = url;
                false
            }
        }
    }
}

fn view_shortened_url(result: &Option<String>) -> Html<SubmissionForm> {
    let url = result.clone().unwrap_or(String::from("<nothing here yet!>"));
    html! {
        <p>{ url }</p>
    }
}

impl Renderable<SubmissionForm> for SubmissionForm {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <input
                    disabled=self.submitted,
                    type="text",
                    value=&self.url,
                    oninput=|url| Msg::GotUrl(url.value),/>
                <button
                    disabled=self.submitted,
                    onclick=|_| Msg::Submit,>{ "Submit" }</button>
                <div>{ view_shortened_url(&self.shortened_url) }</div>
            </div>
        }
    }
}
