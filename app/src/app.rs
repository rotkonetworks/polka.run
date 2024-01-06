use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::{Route, Router, Routes};

use crate::navigation::Navigation;
use crate::home::Home;
use crate::disassembler::Disassembler;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <body class="h-screen w-screen">
        <Navigation/>
        <Router>
        <Routes>
        <Route path="" view=  move || view! { <Home/> }/>
        <Route path="disassembler" view=  move || view! { <Disassembler/> }/>
        </Routes>
        </Router>
        </body>
    }
}
