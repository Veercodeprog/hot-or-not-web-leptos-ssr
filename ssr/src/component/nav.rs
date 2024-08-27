use super::nav_icons::*;
// use crate::state::auth::user_details;
use crate::component::canisters_prov::{AuthCansProvider, WithAuthCans};
use crate::state::canisters::Canisters;

use crate::utils::profile::ProfileDetails;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[component]
fn ProfileLoading() -> impl IntoView {
    view! {
        <div class="basis-4/12 aspect-square overflow-clip rounded-full bg-white/20 animate-pulse"></div>
        <div class="basis-8/12 flex flex-col gap-2 animate-pulse">
            <div class="w-full h-4 bg-white/20 rounded-full"></div>
            <div class="w-full h-4 bg-white/20 rounded-full"></div>
        </div>
    }
}

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
                        <div class="py-3">
                            <Icon icon=icon class="text-white h-6 w-6"/>
                        </div>
                    }
                }
            >

                <div class="py-3 border-t-2 border-t-pink-500">
                    <Icon
                        icon=filled_icon.unwrap_or(icon)
                        class="text-white aspect-square h-6 w-6"
                    />
                </div>
            // <div class="absolute bottom-0 bg-primary-600 py-1 w-6 blur-md"></div>
            </Show>
        </a>
    }
}
#[component]
fn ProfileNavIcon(canisters: Canisters<true>, cur_selected: Memo<usize>) -> impl IntoView {
    // Retrieve the profile details
    let profile_details = canisters.profile_details();

    // Create a reactive href based on the user's details
    let profile_href =
        create_memo(move |_| format!("/your-profile/{}", profile_details.username_or_principal()));

    view! {
        <NavIcon
            idx=5
            href=profile_href
            icon=ProfileIcon0
            filled_icon=ProfileIcon0
            cur_selected=cur_selected
        />
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
                        <div class="py-3">
                            <Icon icon=TrophySymbol class="text-white fill-none h-6 w-6"/>
                        </div>
                    }
                }
            >

                <div class="py-3 border-t-2 border-t-pink-500">
                    <Icon
                        icon=TrophySymbolFilled
                        class="text-white fill-none aspect-square h-6 w-6"
                    />
                </div>
            // <div class="absolute bottom-0 bg-primary-600 py-1 w-6 blur-md"></div>
            </Show>
        </a>
    }
}

#[component]
fn UploadIcon(idx: usize, cur_selected: Memo<usize>) -> impl IntoView {
    view! {
        <a href="/upload" class="flex items-center justify-center rounded-fullt text-white">
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
            "/your-profile" => 5,
            s if s.starts_with("/your-profile") => 5,
            s if s.starts_with("/hot-or-not") => {
                home_path.set(path);
                0
            }
            s if s.starts_with("/profile") => 0,
            _ => 4,
        }
    });
    let bg_color = move || {
        if cur_selected() == 0
            || cur_location
                .pathname
                .get()
                .as_str()
                .starts_with("/your-profile")
        {
            "bg-transparent"
        } else {
            "bg-black"
        }
    };

    view! {
        <AuthCansProvider fallback=ProfileLoading let:canisters>

        <div class=move || {
            format!(
                "flex flex-row justify-between px-6 py-2 w-full {} fixed left-0 bottom-0 z-50",
                bg_color(),
            )
        }>
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
                idx=5
                href="  "
                icon=ProfileIcon0
                filled_icon=ProfileIcon0
                cur_selected=cur_selected
            />
            <NavIcon idx=4 href="/menu" icon=MenuSymbol cur_selected=cur_selected/>
        </div>
        </AuthCansProvider>

    }
}
