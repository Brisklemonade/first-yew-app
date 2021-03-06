use yew::{prelude::*, html::Scope};

enum Msg {
    AddOne,
}

struct CounterComponent {
    count: i64,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => { 
                self.count += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link: &Scope<CounterComponent> = ctx.link();
        html! (
            <>
                <div class="container">
                    <h1 class="title">{"Welcome to a Rust made web app"}</h1>
                    <p>{self.count}</p>
                    <button onclick={link.callback(|_| Msg::AddOne)}>{"+1"}</button>
                </div>
            </>
        )
    }
}

fn main() {
    yew::start_app::<CounterComponent>(); 
}
