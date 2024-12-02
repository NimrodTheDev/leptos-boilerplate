use input::Input_Element;
use leptos::*;
use leptos_dom::logging::console_log;
use crate::components::atoms::*;
// use crate::components::atoms::*;
mod components;
fn main() {
    leptos::mount_to_body(||{
        view!{
            <div>
                <Input_Element label="First name".to_string() is_required=true value="String".to_string() on_change=|arg|{
                    console_log(&arg);
                } />
                // <Input_Element />
            </div>
        }
    });
}
