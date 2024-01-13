
mod prelude;
use crate::prelude::*;
pub mod image;
use image::Image;

fn main() {
    yew::Renderer::<App>::new().render();

}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <Image src="https://batiment.imag.fr/img/imag.png" zoom={1.0} position={(0, 0)}  />
        </div>
    }
}
