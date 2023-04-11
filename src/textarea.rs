use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement,HtmlTextAreaElement};
//use yew::{function_component, html, Html, Properties, use_state, Callback};
//use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TextAreaProps {
    /// The link must have a target.
   pub id: String,
   pub  on_change:Callback<Event>,
    
  
}
/*pub struct TextArea{
    props: TextAreaProps,
    link: ComponentLink<Self>,
}
pub enum Msg {
    SetInputEnabled(bool)
}*/

#[function_component]
pub fn TextArea(props:&TextAreaProps)->Html{

   
    html! {
        <>
             <p>
                <textarea style="float:left" id={props.id.to_owned()} onchange={props.on_change.clone()}></textarea>
             </p>
       </>
   }
}

/* 
impl Component for TextArea {
    type Properties = TextAreaProps;
    // ...
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextArea { props, link }
    }
    fn view(&self) -> Html {

        let onchange = self.link.batch_callback(|e| {
            if let ChangeData::Value(value) = e {
                // do something with the String value
            } else {
                None
            }
        });
    
        html! {
             <div class="tab-pane fade" id="css">
                            <p>
                                <textarea style="float:left" id="cssTextarea" onchange={onchange} ></textarea>
                            </p>
            </div>
        }
    }

    // ...
}*/
