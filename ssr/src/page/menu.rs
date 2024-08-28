use crate::component::back_btn::BackButton;
use crate::component::canisters_prov::{AuthCansProvider, WithAuthCans};
use crate::component::content_upload::YoutubeUpload;
use crate::component::modal::Modal;
use crate::component::spinner::Spinner;
use crate::component::title::Title;
use crate::component::{connect::ConnectLogin, social::*, toggle::Toggle};
use crate::consts::{social, NSFW_TOGGLE_STORE};
use crate::state::auth::account_connected_reader;
use crate::state::canisters::Canisters;
use crate::state::content_seed_client::ContentSeedClient;
use crate::utils::notifications::get_token_for_principal;
use crate::utils::profile::ProfileDetails;
use candid::Principal;
use codee::string::FromToStringCodec;
use leptos::html::Input;
use leptos::*;
use leptos_icons::*;
use leptos_router::use_query_map;
use leptos_use::storage::use_local_storage;
use leptos_use::use_event_listener;

#[derive(Default, Clone, Copy)]
pub struct AuthorizedUserToSeedContent(pub RwSignal<Option<(bool, Principal)>>);

#[component]
fn MenuItem(
    #[prop(into)] text: String,
    #[prop(into)] href: String,
    #[prop(into)] icon: icondata::Icon,
    #[prop(into, optional)] target: String,
) -> impl IntoView {
    view! {
        <a href=href class="grid grid-cols-3 items-center w-full" target=target>
            <div class="flex flex-row gap-4 items-center col-span-2">
                <Icon class="text-2xl" icon=icon/>
                <span class="text-wrap">{text}</span>
            </div>
            <Icon class="text-2xl justify-self-end" icon=icondata::AiRightOutlined/>
        </a>
    }
}

#[component]
fn MenuFooter() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center w-full gap-4 pt-10 pb-8">
            <span class="text-white/50 text-sm">Follow us on</span>
            <div class="flex flex-row gap-4">
                <Telegram/>
                <Discord/>
                <Twitter/>
                <IcWebsite/>
            </div>
            <svg class="h-14 rounded-md outline outline-primary-600 outline-1" viewBox="0 0 228 49">
                <path
                    fill="#F15A24"
                    d="M51.4 12c-3 0-6.1 1.5-9.5 4.5l-4 4.1 3.5 3.8c1-1.2 2.3-2.8 4-4.2 3-2.7 4.9-3.2 6-3.2 4.2 0 7.5 3.3 7.5 7.5 0 4.1-3.4 7.5-7.5 7.5h-.8c1.2.5 2.6.8 3.8.8 7.7 0 9.2-5 9.3-5.4l.3-3C64 17.7 58.3 12 51.4 12Z"
                ></path>
                <path
                    fill="#9E1EED"
                    d="M24.6 37c3 0 6.1-1.5 9.5-4.5l4-4.1-3.5-3.8c-1 1.2-2.3 2.8-4 4.2-3 2.7-4.9 3.2-6 3.2a7.6 7.6 0 0 1-7.5-7.5c0-4.1 3.4-7.5 7.5-7.5h.8c-1.2-.5-2.6-.8-3.8-.8-7.7 0-9.1 5-9.3 5.4A12.6 12.6 0 0 0 24.6 37Z"
                ></path>
                <path
                    fill="#29ABE2"
                    d="M54.4 32.7c-4 0-8-3.2-8.8-4a207 207 0 0 1-7.5-8c-3.7-4-8.6-8.7-13.5-8.7-6 0-11 4.1-12.3 9.6.1-.4 2.1-5.5 9.3-5.4 4 .1 8 3.3 8.8 4.1 2.2 2 7.1 7.5 7.5 8 3.7 4 8.6 8.7 13.5 8.7 6 0 11-4.1 12.3-9.6-.2.4-2.1 5.5-9.3 5.3Z"
                ></path>
                <path
                    fill="#fff"
                    d="M73 33.8c.3 0 .5-.2.5-.5v-6.6c0-.3-.2-.5-.5-.5h-.5c-.3 0-.5.2-.5.5v6.6c0 .3.2.5.5.5h.5ZM83.2 33.8c.3 0 .5-.2.5-.5v-6.6c0-.3-.2-.5-.5-.5h-.5c-.3 0-.5.2-.5.5v4.5l-3-4.6a1 1 0 0 0-.8-.4h-.8c-.3 0-.5.2-.5.5v6.6c0 .3.2.5.5.5h.5c.3 0 .5-.2.5-.5v-5l3.4 5.3.4.2h.8ZM92.5 27.6c.2 0 .5-.2.5-.5v-.4c0-.3-.3-.5-.5-.5H87c-.2 0-.5.2-.5.5v.4c0 .3.3.5.5.5h2v5.7c0 .3.2.5.5.5h.5c.3 0 .5-.2.5-.5v-5.7h2ZM100.1 33.8c.3 0 .5-.2.5-.5V33c0-.3-.2-.5-.5-.5h-2.8v-1.8h2.5c.3 0 .5-.2.5-.5v-.3c0-.3-.2-.5-.5-.5h-2.5v-1.7h2.8c.3 0 .5-.3.5-.5v-.4c0-.3-.2-.5-.5-.5h-3.8c-.3 0-.5.2-.5.5v6.6c0 .3.2.5.5.5h3.8ZM107.5 33.6l.5.2h.5c.4 0 .7-.4.5-.7l-1.3-2.4c1-.3 1.7-1.1 1.7-2.2 0-1.3-1-2.3-2.5-2.3h-2.5c-.3 0-.5.2-.5.5v6.6c0 .3.2.5.5.5h.5c.3 0 .5-.2.5-.5V31h.8l1.3 2.7Zm-2.1-4v-2.1h1.2c.8 0 1.2.4 1.2 1 0 .7-.4 1-1.2 1h-1.2ZM118.6 33.8c.3 0 .5-.2.5-.5v-6.6c0-.3-.2-.5-.5-.5h-.5c-.3 0-.5.2-.5.5v4.5l-3-4.6a1 1 0 0 0-.8-.4h-.8c-.3 0-.5.2-.5.5v6.6c0 .3.2.5.5.5h.5c.3 0 .5-.2.5-.5v-5l3.4 5.3.4.2h.8ZM127 33.8c.3 0 .5-.2.5-.5V33c0-.3-.2-.5-.5-.5h-2.8v-1.8h2.5c.3 0 .5-.2.5-.5v-.3c0-.3-.2-.5-.5-.5h-2.5v-1.7h2.8c.3 0 .5-.3.5-.5v-.4c0-.3-.2-.5-.5-.5h-3.8c-.3 0-.5.2-.5.5v6.6c0 .3.2.5.5.5h3.8ZM136 27.6c.2 0 .4-.2.4-.5v-.4c0-.3-.2-.5-.5-.5h-5.4c-.3 0-.5.2-.5.5v.4c0 .3.2.5.5.5h2v5.7c0 .3.2.5.5.5h.5c.3 0 .5-.2.5-.5v-5.7h2ZM146.8 34c2.2 0 3.3-1.4 3.6-2.6L149 31c-.2.7-.9 1.5-2.2 1.5-1.2 0-2.4-.9-2.4-2.5 0-1.7 1.2-2.6 2.4-2.6 1.3 0 2 .8 2.1 1.6l1.5-.5c-.4-1.2-1.5-2.5-3.6-2.5-2 0-4 1.6-4 4s1.9 4 4 4ZM154.4 30c0-1.7 1.3-2.6 2.4-2.6 1.2 0 2.5.9 2.5 2.6 0 1.7-1.3 2.5-2.5 2.5-1.1 0-2.4-.8-2.4-2.5Zm-1.5 0c0 2.5 1.8 4 4 4 2 0 4-1.5 4-4s-2-4-4-4c-2.2 0-4 1.5-4 4ZM172 33.8c.4 0 .6-.2.6-.5v-6.6c0-.3-.2-.5-.5-.5h-1.2c-.2 0-.4 0-.5.3l-2.2 5.2-2.1-5.2c-.1-.2-.3-.3-.5-.3h-1.2c-.2 0-.5.2-.5.5v6.6c0 .3.3.5.5.5h.5c.3 0 .5-.2.5-.5v-4.8l2 5c.2.2.3.3.5.3h.6c.2 0 .4 0 .5-.3l2-5v4.8c0 .3.3.5.6.5h.5ZM177.7 29.7v-2.2h1.2c.7 0 1.2.4 1.2 1 0 .7-.5 1.2-1.2 1.2h-1.2Zm1.3 1.2c1.6 0 2.6-1 2.6-2.3 0-1.4-1-2.4-2.6-2.4h-2.4c-.2 0-.5.2-.5.5v6.6c0 .3.3.5.5.5h.5c.3 0 .5-.2.5-.5V31h1.4ZM187.4 34c1.7 0 3-1 3-2.9v-4.4c0-.3-.2-.5-.5-.5h-.5c-.3 0-.5.2-.5.5V31c0 1-.6 1.5-1.5 1.5S186 32 186 31v-4.3c0-.3-.2-.5-.5-.5h-.5c-.2 0-.5.2-.5.5V31c0 1.9 1.4 2.9 3 2.9ZM199 27.6c.4 0 .6-.2.6-.5v-.4c0-.3-.2-.5-.5-.5h-5.4c-.3 0-.5.2-.5.5v.4c0 .3.2.5.5.5h2v5.7c0 .3.1.5.4.5h.5c.3 0 .5-.2.5-.5v-5.7h2ZM206.8 33.8c.2 0 .5-.2.5-.5V33c0-.3-.3-.5-.5-.5h-2.9v-1.8h2.5c.3 0 .5-.2.5-.5v-.3c0-.3-.2-.5-.5-.5H204v-1.7h2.9c.2 0 .5-.3.5-.5v-.4c0-.3-.3-.5-.5-.5h-3.9c-.3 0-.5.2-.5.5v6.6c0 .3.2.5.5.5h3.9ZM214.2 33.6l.4.2h.6c.3 0 .6-.4.4-.7l-1.3-2.4c1-.3 1.7-1.1 1.7-2.2 0-1.3-1-2.3-2.5-2.3h-3v7.1c0 .3.2.5.5.5h.5c.3 0 .5-.2.5-.5V31h.8l1.4 2.7Zm-2.2-4v-2.1h1.2c.8 0 1.2.4 1.2 1 0 .7-.4 1-1.2 1H212ZM73 17v-3h1.7c1 0 1.6.6 1.6 1.5s-.6 1.4-1.6 1.4H73Zm1.8.8c1.4 0 2.4-1 2.4-2.3 0-1.3-1-2.3-2.4-2.3H72V21h.9v-3.2h1.8Zm5.5-2.2c-1.5 0-2.6 1.1-2.6 2.8 0 1.6 1 2.8 2.6 2.8 1.5 0 2.6-1.2 2.6-2.8 0-1.7-1-2.8-2.6-2.8Zm0 .8c1 0 1.8.7 1.8 2 0 1.2-.9 2-1.8 2-1 0-1.8-.8-1.8-2 0-1.3.9-2 1.8-2Zm7-.7L86 20l-1.3-4.2h-1l1.8 5.3h1l1.4-4.2 1.4 4.2h1l1.7-5.3h-1L89.7 20l-1.5-4.2h-.9Zm6.2 2.2c0-.8.7-1.5 1.6-1.5 1 0 1.5.6 1.5 1.5h-3.1Zm3.3 1.3c-.2.7-.7 1.2-1.6 1.2-1 0-1.7-.8-1.7-1.8h4v-.3c0-1.6-.8-2.7-2.4-2.7-1.4 0-2.5 1-2.5 2.7 0 1.8 1.2 2.9 2.6 2.9 1.2 0 2-.8 2.3-1.7l-.7-.3Zm5-3.5h-.5c-.5 0-1.2.2-1.6 1v-1H99V21h.9v-2.7c0-1.2.6-1.7 1.5-1.7h.4v-.9Zm1.4 2.2c0-.8.7-1.5 1.6-1.5 1 0 1.5.6 1.5 1.5h-3.1Zm3.3 1.3c-.2.7-.7 1.2-1.6 1.2-1 0-1.7-.8-1.7-1.8h4v-.3c0-1.6-.8-2.7-2.4-2.7-1.4 0-2.5 1-2.5 2.7 0 1.8 1.2 2.9 2.6 2.9 1.2 0 2-.8 2.3-1.7l-.7-.3Zm5.9 1v.8h1l-.1-1v-7h-1v3.6c0-.5-.6-1-1.6-1-1.5 0-2.5 1.2-2.5 2.8 0 1.5 1 2.8 2.5 2.8.9 0 1.5-.6 1.7-1.1v.1Zm-1.6.2c-1 0-1.7-.9-1.7-2 0-1.2.7-2 1.7-2s1.6.8 1.6 2c0 1.1-.7 2-1.6 2Zm7.8.6v-.9c.3.6 1 1 1.8 1 1.6 0 2.5-1.2 2.5-2.7 0-1.6-.9-2.8-2.4-2.8a2 2 0 0 0-1.8 1V13h-1v8h1Zm1.7-.6c-1 0-1.7-.9-1.7-2 0-1.2.7-2 1.7-2s1.7.8 1.7 2c0 1.1-.7 2-1.7 2Zm5 2.7 3.4-7.4h-1l-1.6 3.8-1.7-3.8h-1l2.3 4.7-1.3 2.7h1Z"
                ></path>
            </svg>
        </div>
    }
}

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
fn ProfileLoaded(user_details: ProfileDetails) -> impl IntoView {
    let (is_connected, _) = account_connected_reader();
    view! {
        <div class="basis-4/12 aspect-square overflow-clip rounded-full">
            <img class="h-full w-full object-cover" src=user_details.profile_pic_or_random()/>
        </div>
        <div
            class="flex flex-col basis-8/12"
            class=("w-12/12", move || !is_connected())
            class=("sm:w-5/12", move || !is_connected())
        >
            <span class="text-white text-ellipsis line-clamp-1 text-xl">
                {user_details.display_name_or_fallback()}
            </span>
            <a
                class="text-primary-600 text-md"
                href=format!("/your-profile/{}", user_details.username_or_principal())
            >
                View Profile
            </a>
        </div>
    }
}

#[component]
fn ProfileInfo() -> impl IntoView {
    view! {
        <AuthCansProvider fallback=ProfileLoading let:canisters>
            <ProfileLoaded user_details=canisters.profile_details()/>
        </AuthCansProvider>
    }
}

#[component]
fn NsfwToggle() -> impl IntoView {
    let (nsfw_enabled, set_nsfw_enabled, _) =
        use_local_storage::<bool, FromToStringCodec>(NSFW_TOGGLE_STORE);
    let toggle_ref = create_node_ref::<Input>();

    _ = use_event_listener(toggle_ref, ev::change, move |_| {
        set_nsfw_enabled(
            toggle_ref
                .get_untracked()
                .map(|t| t.checked())
                .unwrap_or_default(),
        )
    });

    view! {
        <div class="grid grid-cols-2 items-center w-full">
            <div class="flex flex-row gap-4 items-center">
                <Icon class="text-2xl" icon=icondata::BiShowAltRegular/>
                <span>Show NSFW Videos</span>
            </div>
            <div class="justify-self-end">
                <Toggle checked=nsfw_enabled node_ref=toggle_ref/>
            </div>
        </div>
    }
}

#[component]
fn EnableNotifications(user_details: ProfileDetails) -> impl IntoView {
    let (_, _) = account_connected_reader();

    let on_token_click = create_action(move |()| async move {
        get_token_for_principal(user_details.principal.to_string()).await;
    });

    view! {
        <div class="grid grid-cols-2 items-center w-full">
            <div class="flex flex-row gap-4 items-center">
                <Icon class="text-2xl" icon=icondata::BiCommentDotsRegular/>
                <span>Enable Notifications</span>
            </div>
            <div class="justify-self-end">
                <button
                    class="p-2 bg-black rounded-md text-white"
                    on:click=move |_| on_token_click.dispatch(())
                >
                    Enable
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn Menu() -> impl IntoView {
    let (is_connected, _) = account_connected_reader();
    let query_map = use_query_map();
    let show_content_modal = create_rw_signal(false);
    let is_authorized_to_seed_content: AuthorizedUserToSeedContent = expect_context();

    create_effect(move |_| {
        //check whether query param is right if right set the show_modal_content as true.
        let query_params = query_map.get();
        let url = query_params.0.get("text")?;
        if !url.is_empty() && is_connected.get() {
            show_content_modal.set(true);
        }
        Some(())
    });

    let authorized_fetch = move |cans: Canisters<true>| async move {
        let user_principal = cans.user_principal();
        match is_authorized_to_seed_content.0.get_untracked() {
            Some((auth, principal)) if principal == user_principal => return auth,
            _ => (),
        }

        let content_seed_client: ContentSeedClient = expect_context();

        content_seed_client
            .check_if_authorized(user_principal)
            .await
            .unwrap_or_default()
    };

    view! {
        <WithAuthCans with=authorized_fetch let:authorized>
            {is_authorized_to_seed_content.0.set(Some((authorized.1, authorized.0.user_principal())))}
        </WithAuthCans>
        <Modal show=show_content_modal>
            <AuthCansProvider fallback=Spinner let:canisters>
                <YoutubeUpload
                    canisters
                    url=query_map.get().0.get("text").cloned().unwrap_or_default()
                />
            </AuthCansProvider>
        </Modal>
        <div class="min-h-screen w-full flex flex-col text-white pt-2 pb-12 bg-black items-center divide-y divide-white/10">
            <div class="flex flex-col items-center w-full gap-20 pb-16">
                <Title justify_center=false>
                    <div class="flex flex-row justify-between">
                        <BackButton fallback="/".to_string()/>
                        <span class="font-bold text-2xl">Menu</span>
                        <div></div>
                    </div>
                </Title>
                <div class="flex flex-col items-center w-full gap-4">
                    <div class="flex flex-row w-full max-w-lg justify-center gap-4 items-center px-4">
                        <ProfileInfo/>
                    </div>
                    <Show when=move || !is_connected()>
                        <div class="w-full px-8 md:w-4/12 xl:w-2/12">
                            <ConnectLogin/>
                        </div>
                        <div class="w-full px-8 text-center text-sm font-sans">
                            {r#"Your Yral account has been setup. Login with Google to not lose progress."#}
                        </div>
                    </Show>
                    <Show when=move || is_authorized_to_seed_content.0.get().map(|(a, _)| a).unwrap_or_default() && is_connected()>
                        <div class="w-full px-8 md:w-4/12 xl:w-2/12">
                            <button
                                class="font-bold rounded-full bg-primary-600 py-2 md:py-3 w-full text-center text-lg md:text-xl text-white"
                                on:click=move |_| show_content_modal.set(true)
                            >
                                Upload Content
                            </button>
                        </div>
                    </Show>

                </div>
            </div>
            <div class="flex flex-col py-12 px-8 gap-8 w-full text-lg">
                <NsfwToggle/>
                <MenuItem
                    href="/account-transfer"
                    text="HotorNot Account Transfer"
                    icon=icondata::FaMoneyBillTransferSolid
                />
                <MenuItem href="/refer-earn" text="Refer & Earn" icon=icondata::AiGiftFilled/>
                    <MenuItem href="/leaderboard" text="Leaderboard" icon=icondata::ChTrophy/>

                <MenuItem
                    href=social::TELEGRAM
                    text="Talk to the team"
                    icon=icondata::BiWhatsapp
                    target="_blank"
                />
                <MenuItem href="/terms-of-service" text="Terms of Service" icon=icondata::TbBook2/>
                <MenuItem href="/privacy-policy" text="Privacy Policy" icon=icondata::TbLock/>
                <MenuItem
                    href="/settings"
                    text="Settings"
                    icon=icondata::BiCogRegular
                />
                <Show when=is_connected>
                    <MenuItem href="/logout" text="Logout" icon=icondata::FiLogOut/>
                </Show>
            // <MenuItem href="/install-app" text="Install App" icon=icondata::TbDownload/>
            </div>
            <MenuFooter/>
        </div>
    }
}
