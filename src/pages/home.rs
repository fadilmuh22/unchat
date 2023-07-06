use leptos::*;

use crate::components::chat_box::ChatBox;

/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col items-center justify-center w-screen min-h-screen bg-gray-100 text-gray-800 p-10">
            <ChatBox />
        </div>
    }
}
