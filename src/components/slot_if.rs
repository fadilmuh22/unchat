use leptos::*;

// Slots are created in simillar manner to components, except that they use the #[slot] macro.
#[slot]
struct Then {
    children: ChildrenFn,
}

// Props work just like component props, for example, you can specify a prop as optional by prefixing
// the type with Option<...> and marking the option as #[prop(optional)].
#[slot]
struct ElseIf {
    cond: MaybeSignal<bool>,
    children: ChildrenFn,
}

#[slot]
struct Fallback {
    children: ChildrenFn,
}

// Slots are added to components like any other prop.
#[component]
fn SlotIf(
    cx: Scope,
    cond: MaybeSignal<bool>,
    then: Then,
    #[prop(default=vec![])] else_if: Vec<ElseIf>,
    #[prop(optional)] fallback: Option<Fallback>,
) -> impl IntoView {
    move || {
        if cond() {
            (then.children)(cx).into_view(cx)
        } else if let Some(else_if) = else_if.iter().find(|i| (i.cond)()) {
            (else_if.children)(cx).into_view(cx)
        } else if let Some(fallback) = &fallback {
            (fallback.children)(cx).into_view(cx)
        } else {
            ().into_view(cx)
        }
    }
}
