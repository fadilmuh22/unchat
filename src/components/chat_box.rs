use leptos::*;

use super::chat_buble::ChatBuble;
use crate::{app::ModalContext, types::chat::Chat};

#[component]
pub fn ChatBox(cx: Scope) -> impl IntoView {
    let modal = use_context::<ModalContext>(cx).unwrap();

    let show_modal = move |_| {
        (modal.set_title)("Hello".to_string());
        (modal.set_content)("World".to_string());
        (modal.set_open).update(|value| *value = !(*value));
    };

    let (chats, _) = create_signal::<Vec<Chat>>(
        cx,
        vec![
            Chat {
                uuid: "1".to_string(),
                content: "Hello".to_string(),
                user: crate::types::user::User {
                    uuid: "1".to_string(),
                    email: "user1@mail.com".to_string(),
                    avatar: "https://i.pravatar.cc/150?img=1".to_string(),
                    username: "user1".to_string(),
                    created_at: "2021-08-01T00:00:00Z".to_string(),
                    updated_at: "2021-08-01T00:00:00Z".to_string(),
                },
                user_id: "1".to_string(),
                to_user_id: "2".to_string(),
                created_at: "2021-08-01T00:00:00Z".to_string(),
                updated_at: "2021-08-01T00:00:00Z".to_string(),
            },
            Chat {
                uuid: "2".to_string(),
                content: "Hello".to_string(),
                user: crate::types::user::User {
                    uuid: "2".to_string(),
                    email: "user2@mail.com".to_string(),
                    avatar: "https://i.pravatar.cc/150?img=1".to_string(),
                    username: "user2".to_string(),
                    created_at: "2021-08-01T00:00:00Z".to_string(),
                    updated_at: "2021-08-01T00:00:00Z".to_string(),
                },
                user_id: "2".to_string(),
                to_user_id: "1".to_string(),
                created_at: "2021-08-01T00:00:00Z".to_string(),
                updated_at: "2021-08-01T00:00:00Z".to_string(),
            },
            Chat {
                uuid: "1".to_string(),
                content: "Hello".to_string(),
                user: crate::types::user::User {
                    uuid: "1".to_string(),
                    email: "user1@mail.com".to_string(),
                    avatar: "https://i.pravatar.cc/150?img=1".to_string(),
                    username: "user1".to_string(),
                    created_at: "2021-08-01T00:00:00Z".to_string(),
                    updated_at: "2021-08-01T00:00:00Z".to_string(),
                },
                user_id: "1".to_string(),
                to_user_id: "2".to_string(),
                created_at: "2021-08-01T00:00:00Z".to_string(),
                updated_at: "2021-08-01T00:00:00Z".to_string(),
            },
        ],
    );

    view! { cx,
        <div class="flex flex-col flex-grow w-full max-w-xl bg-white shadow-xl rounded-lg overflow-hidden">
            <div class="flex flex-col flex-grow h-0 p-4 overflow-auto">
                <For
                    each=move || chats.get()
                    key=|chat| chat.uuid.clone()
                    view=move |cx, chat: Chat| {
                        view! {
                            cx,
                            <ChatBuble chat=chat />
                        }
                    }
                />
            </div>

            <div class="flex bg-gray-300 p-4 items-center">
                <input class="flex items-center h-10 w-full rounded px-3 text-sm" type="text" placeholder="Type your message…" />
                <div class="ml-4">
                    <button
                        on:click=show_modal
                        class="flex items-center justify-center bg-indigo-500 hover:bg-indigo-600 rounded-xl text-white px-4 py-1 flex-shrink-0"
                    >
                        <span>"Send"</span>
                        <span class="ml-2">
                        <svg
                            class="w-4 h-4 transform rotate-45 -mt-px"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"
                            ></path>
                        </svg>
                        </span>
                    </button>
                </div>
            </div>
        </div>
    }
}
