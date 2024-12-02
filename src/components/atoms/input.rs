use leptos::*;
use svg::view;
use web_sys::EventTarget;



#[component]
pub fn Input_Element <E>(
    #[prop(optional)]
    label: String,
    #[prop(optional)]
    is_required: bool,
    value: String,
    on_change: E
)-> impl IntoView 
where 
    E: Fn(String) + 'static{
    view!{
        <div class="flex flex-col gap-[7px] w-full">
            {
                if label.is_empty() {
                    view!{
                    }.into_view()
                }else{
                    view!{
                        <label class="flex gap-1">{label} {if is_required {view! {<span class="text-[red]">*</span>}.into_view()}else{view! {}.into_view()}}</label>
                    }.into_view()
                }
            }
            <input class="rounded-[30px] p-[12px] w-full border outline-none" prop:value=value on:input=move|e|{
                on_change(event_target_value(&e))
            }/>
        </div>
    }
}