use std::collections::HashMap;

use yew::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, EventTarget, HtmlTextAreaElement, HtmlElement};
mod textarea;
use textarea::TextArea;


#[function_component]
fn App() -> Html {


    let input_value_handle = use_state(||HashMap::<String,String>::new());
    let mut input_value = (*input_value_handle).clone();
   // let iframe_ref = use_node_ref();
   let srcdoc_value_handle = use_state(||String::default());
   let srcdoc=(*srcdoc_value_handle).clone();


   fn get_val(val:HashMap<String,String>)->String{
    let st:String= val.clone().iter().fold("".to_string(), |mut st ,(key,v)|{
        format!("{} <{}>{}</{}",st,key,v,key.clone())
    });
    return st;
}
   /* use_effect_with_deps(move |_|{
        srcdoc_value_handle.set(get_val((*input_value_handle).clone()))
    },(*input_value_handle).clone(),);*/
    let on_change = {
        let input_value_handle = input_value_handle.clone();
        //let input_values=(*input_value_handle).clone();
        Callback::from(move | e: Event| {
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

            if let Some(inpu) = input {
                let mut inp =input_value.clone();
                inp.insert(inpu.id(),inpu.value());
                input_value_handle.set(inp);
            }
        })
    };

   

    html! {
        <div class={classes!("container")}>
        <h3>{"Web Playground"}</h3>
        <div class="row">
            <div class="col-12">
                <ul id="myTab" class={classes!("nav", "nav-tabs")} data-tabs="tabs">
                    <li class={classes!("active")}><a href="#html" data-toggle="tab"> {"HTML"}</a></li>
                    <li><a href="#css" data-toggle="tab">{"CSS"}</a></li>
                    <li><a href="#js" data-toggle="tab">{"JS"}</a></li>
                </ul>
                <div id="myTabContent" class= {classes!("tab-content")}>
                    <div class={classes!("tab-pane" ,"fade", "in" ,"active")} id="html">
                        <TextArea id={"html"} on_change={on_change.clone()}/>

                    </div>

                    <div class={classes!("tab-pane" ,"fade")} id="css">
                        <TextArea id={"style"} on_change={on_change.clone()}/>

                    </div>

                    <div class={classes!("tab-pane" ,"fade")}  id="js">
                        <TextArea id={"script"} on_change={on_change.clone()}/>

                    </div>

                    


                </div>
            </div>
            <div class={classes!("col-12")}>
                <div>
                    <iframe id="iFrame" srcdoc={get_val((*input_value_handle).clone())}></iframe>
                </div>
            </div>
        </div>

    </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}