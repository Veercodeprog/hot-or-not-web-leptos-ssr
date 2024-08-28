use super::nav_icons::*;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[component]
fn NavIcon(
    idx: usize,
    #[prop(into)] href: MaybeSignal<String>,
    #[prop(into)] icon: icondata_core::Icon,
    #[prop(optional)] filled_icon: Option<icondata_core::Icon>,
    cur_selected: Memo<usize>,
) -> impl IntoView {
    view! {
        <a href=href class="flex items-center justify-center">
            <Show
                when=move || cur_selected() == idx
                fallback=move || {
                    view! {
                        <div class="py-5">
                            <Icon icon=icon class="text-white text-2xl md:text-3xl"/>
                        </div>
                    }
                }
            >

                <div class="py-5 border-t-2 border-t-pink-500">
                    <Icon
                        icon=filled_icon.unwrap_or(icon)
                        class="text-white aspect-square text-2xl md:text-3xl"
                    />
                </div>
            </Show>
        </a>
    }
}

#[component]
fn TrophyIcon(idx: usize, cur_selected: Memo<usize>) -> impl IntoView {
    view! {
        <a href="/leaderboard" class="flex items-center justify-center">
            <Show
                when=move || cur_selected() == idx
                fallback=move || {
                    view! {
                        <div class="py-5">
                            <Icon icon=TrophySymbol class="text-white fill-none text-2xl md:text-3xl"/>
                        </div>
                    }
                }
            >

                <div class="py-5 border-t-2 border-t-pink-500">
                    <Icon
                        icon=TrophySymbolFilled
                        class="text-white fill-none aspect-square text-2xl md:text-3xl"
                    />
                </div>
            </Show>
        </a>
    }
}

#[component]
fn UploadIcon(idx: usize, cur_selected: Memo<usize>) -> impl IntoView {
    view! {
        <a href="/upload" class="flex items-center justify-center rounded-full text-white">
            <Show
                when=move || cur_selected() == idx
                fallback=move || {
                    view! {
                        <Icon
                            icon=icondata::AiPlusOutlined
                            class="rounded-full bg-transparent h-10 w-10 border-2 p-2"
                        />
                    }
                }
            >

                <div class="border-t-2 border-transparent">
                    <Icon
                        icon=icondata::AiPlusOutlined
                        class="bg-primary-600 rounded-full aspect-square h-10 w-10 p-2"
                    />
                    <div class="absolute bottom-0 bg-primary-600 w-10 blur-md"></div>
                </div>
            </Show>
        </a>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    let cur_location = use_location();
    let home_path = create_rw_signal("/".to_string());
    let cur_selected = create_memo(move |_| {
        let path = cur_location.pathname.get();
        match path.as_str() {
            "/" => 0,
            "/leaderboard" => 1,
            "/upload" => 2,
            "/wallet" | "/transactions" => 3,
            "/menu" => 4,
            "my-profile" => 5,
            s if s.starts_with("/your-profile") => 4,
            s if s.starts_with("/hot-or-not") => {
                home_path.set(path);
                0
            }
            s if s.starts_with("/profile") => 0,
            _ => 4,
        }
    });

    view! {
        <div class="fixed z-50 bottom-0 left-0 flex flex-row justify-between px-6 items-center w-full bg-black/80">
            <NavIcon
                idx=0
                href=home_path
                icon=HomeSymbol
                filled_icon=HomeSymbolFilled
                cur_selected=cur_selected
            />
             <NavIcon
                idx=3
                href="/wallet"
                icon=WalletSymbol
                filled_icon=WalletSymbolFilled
                cur_selected=cur_selected
            />
            <UploadIcon idx=2 cur_selected/>
            <NavIcon
                idx=1
                href="/leaderboard"
                icon=ProfileIcon
                filled_icon=ProfileIconFilled
                cur_selected=cur_selected
            />
            <NavIcon idx=4 href="/menu" icon=MenuSymbol cur_selected=cur_selected/>
        </div>
    }
}
