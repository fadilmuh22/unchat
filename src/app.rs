use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::modal::Modal;
use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::home::HomePage;

#[derive(Copy, Clone, Debug)]
pub struct ModalContext {
    pub open: ReadSignal<bool>,
    pub set_open: WriteSignal<bool>,
    pub set_title: WriteSignal<String>,
    pub set_content: WriteSignal<String>,
}

#[derive(Copy, Clone)]
pub struct ValueSetter(pub WriteSignal<i32>);

#[component]
pub fn ModalProvider(cx: Scope, children: ChildrenFn) -> impl IntoView {
    let (open, set_open) = create_signal(cx, false);
    let (title, set_title) = create_signal(cx, "Title".to_string());
    let (content, set_content) = create_signal(cx, "Content".to_string());

    // the newtype pattern isn't *necessary* here but is a good practice
    // it avoids confusion with other possible future `WriteSignal<bool>` contexts
    // and makes it easier to refer to it in Button
    provide_context(
        cx,
        ModalContext {
            open,
            set_open,
            set_title,
            set_content,
        },
    );

    view! { cx,
        <div>
            <Modal
                open=open
                title=title
                content=content
                set_open=set_open
            />
            {(children)(cx).into_view(cx)}
        </div>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    // the newtype pattern isn't *necessary* here but is a good practice
    // it avoids confusion with other possible future `WriteSignal<bool>` contexts
    // and makes it easier to refer to it in ButtonD

    view! {
        cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
                <ModalProvider>
                    <Routes>
                        <Route path="" view=|cx| view! {cx, <HomePage/> }/>
                    </Routes>
                </ModalProvider>
            </main>
        </Router>
    }
}
