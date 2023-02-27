
use dominator::{html, Dom, class};
use futures_signals::signal::Signal;
use std::sync::Arc;

pub struct DominatorApp {
}

impl DominatorApp {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }

    pub fn render(state: Arc<Self>) -> Dom {
        html!("div", {
            .children(&mut [
                html!("div", {
                    .attr("id", "consolelog-div")
                    .attr("display", "hidden")
                    .class(["block"])
                }),
                html!("canvas", {
                    .attr("id", "game")
                    .class(["relative", "z-20", "p-16"])
                }),
            ])
        })
    }
}
