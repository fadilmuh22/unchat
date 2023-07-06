use leptos::*;

use crate::types::chat::Chat;

#[component]
pub fn ChatBuble(cx: Scope, chat: Chat) -> impl IntoView {
    let is_sender = chat.user_id == "1".to_string();
    match is_sender {
        true => {
            view! {
                cx,
                <div class="flex w-full mt-2 space-x-3 max-w-xs ml-auto justify-end">
                    <div>
                        <div class="p-3 text-white rounded-l-lg rounded-br-lg bg-blue-600" >
                            <p class="text-sm">{chat.content}</p>
                        </div>
                        <span class="text-xs text-gray-500 leading-none">"2 min ago"</span>
                    </div>
                    <div class="flex-shrink-0 h-10 w-10 rounded-full bg-gray-300"></div>
                </div>
            }
        }
        false => {
            view! { cx,
                <div class="flex w-full mt-2 space-x-3 max-w-xs">
                    <div class="flex-shrink-0 h-10 w-10 rounded-full bg-gray-300"></div>
                    <div>
                        <div class="p-3 text-white rounded-r-lg rounded-bl-lg bg-gray-300">
                            <p class="text-sm">{chat.content}</p>
                        </div>
                        <span class="text-xs text-gray-500 leading-none">"2 min ago"</span>
                    </div>
                </div>
            }
        }
    }
}
