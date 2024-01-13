use crate::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ImageProps {
    pub src: String,
    pub zoom: f64,
    pub position: (i32, i32),
}


pub struct Image {
}

impl Component for Image {
    type Message = ();
    type Properties = ImageProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let src = ctx.props().src.clone();
        yew_template::template_html! {
            "/src/templates/image.html",
            src = src,
        }
    }
}