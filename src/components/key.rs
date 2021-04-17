use yew::prelude::*;

pub struct Key {
  props: Props,
  link: ComponentLink<Self>,
  text: String,
}

pub struct Props {}

pub enum Msg {
  KeyDown,
  KeyUp,
}

impl Component for Key {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Key { props, link, text: String::from("push"),}
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      KeyDown => self.text = String::from("pushed")
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let pointer_down_callback = self.link.callback(|_:PointerEvent| Msg:KeyDown)
    html! {
        <div onpointerdown=pointer_down_callback>{text}</div>
    }
  }
}
