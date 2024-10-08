use std::cmp::Ordering;

use leptos::{html::Video, *};
use leptos_use::use_event_listener;

use crate::utils::event_streaming::events::VideoWatched;
use crate::{
    canister::{
        individual_user_template::PostViewDetailsFromFrontend,
        utils::{bg_url, mp4_url},
    },
    component::{feed_popup::FeedPopUp, video_player::VideoPlayer},
    state::{
        auth::account_connected_reader, canisters::unauth_canisters,
        local_storage::use_referrer_store,
    },
};

use super::{overlay::VideoDetailsOverlay, PostDetails};

#[component]
pub fn BgView(
    video_queue: RwSignal<Vec<PostDetails>>,
    current_idx: RwSignal<usize>,
    idx: usize,
    children: Children,
) -> impl IntoView {
    let post = create_memo(move |_| video_queue.with(|q| q.get(idx).cloned()));
    let uid = move || post().as_ref().map(|q| q.uid.clone()).unwrap_or_default();

    let (is_connected, _) = account_connected_reader();
    let (show_login_popup, set_show_login_popup) = create_signal(true);

    let (show_refer_login_popup, set_show_refer_login_popup) = create_signal(true);
    let (referrer_store, _, _) = use_referrer_store();

    create_effect(move |_| {
        if current_idx.get() % 5 != 0 {
            set_show_login_popup.update(|n| *n = false);
        } else {
            set_show_login_popup.update(|n| *n = true);
        }
        Some(())
    });

    view! {
        <div class="bg-transparent w-full h-full relative overflow-hidden">
            <div
                class="absolute top-0 left-0 bg-cover bg-center w-full h-full z-[1] blur-lg"
                style:background-color="rgb(0, 0, 0)"
                style:background-image=move || format!("url({})", bg_url(uid()))
            ></div>
            <Show when=move || {
                current_idx.get() != 0 && current_idx.get() % 5 == 0 && !is_connected.get()
                    && show_login_popup.get()
            }>
                <FeedPopUp
                    on_click=move |_| set_show_login_popup.set(false)
                    header_text="Your 1000 COYNs
                    Await You!"
                    body_text="SignUp/Login to save your progress and claim your rewards."
                    login_text="Login"
                />
            </Show>
            <Show when=move || {
                referrer_store.get().is_some() && idx == 0 && !is_connected.get()
                    && show_refer_login_popup.get()
            }>
                <FeedPopUp
                    on_click=move |_| set_show_refer_login_popup.set(false)
                    header_text="Claim Your Referral
                    Rewards Now!"
                    body_text="SignUp from this link to get 500 COYNs as referral rewards."
                    login_text="Sign Up"
                />
            </Show>
            {move || post().map(|post| view! { <VideoDetailsOverlay post /> })}
            {children()}
        </div>
    }
}

#[component]
pub fn VideoView(
    #[prop(into)] post: MaybeSignal<Option<PostDetails>>,
    #[prop(optional)] _ref: NodeRef<Video>,
    #[prop(optional)] autoplay_at_render: bool,
    muted: RwSignal<bool>,
) -> impl IntoView {
    let post_for_uid = post.clone();
    let uid = create_memo(move |_| post_for_uid.with(|p| p.as_ref().map(|p| p.uid.clone())));
    let view_bg_url = move || uid().map(bg_url);
    let view_video_url = move || uid().map(mp4_url);

    // Handles mute/unmute
    create_effect(move |_| {
        let vid = _ref()?;
        vid.set_muted(muted());
        Some(())
    });

    create_effect(move |_| {
        let vid = _ref()?;
        // the attributes in DOM don't seem to be working
        vid.set_muted(muted.get_untracked());
        vid.set_loop(true);
        if autoplay_at_render {
            vid.set_autoplay(true);
            _ = vid.play();
        }
        Some(())
    });

    // Video views send to canister
    // 1. When video is paused -> partial video view
    // 2. When video is 95% done -> full view
    let post_for_view = post.clone();
    let send_view_detail_action =
        create_action(move |(percentage_watched, watch_count): &(u8, u8)| {
            let percentage_watched = *percentage_watched;
            let watch_count = *watch_count;
            let post_for_view = post_for_view.clone();

            async move {
                let canisters = unauth_canisters();

                let payload = match percentage_watched.cmp(&95) {
                    Ordering::Less => {
                        PostViewDetailsFromFrontend::WatchedPartially { percentage_watched }
                    }
                    _ => PostViewDetailsFromFrontend::WatchedMultipleTimes {
                        percentage_watched,
                        watch_count,
                    },
                };

                let post = post_for_view.get_untracked();
                let post_id = post.as_ref().map(|p| p.post_id).unwrap();
                let canister_id = post.as_ref().map(|p| p.canister_id).unwrap();
                let send_view_res = canisters
                    .individual_user(canister_id)
                    .await
                    .ok()?
                    .update_post_add_view_details(post_id, payload)
                    .await;

                if let Err(err) = send_view_res {
                    log::warn!("failed to send view details: {:?}", err);
                }
                Some(())
            }
        });

    let video_views_watch_multiple = create_rw_signal(false);

    let _ = use_event_listener(_ref, ev::pause, move |_evt| {
        let Some(video) = _ref() else {
            return;
        };

        let duration = video.duration();
        let current_time = video.current_time();
        if current_time < 0.5 {
            return;
        }

        let percentage_watched = ((current_time / duration) * 100.0) as u8;
        send_view_detail_action.dispatch((percentage_watched, 0_u8));
    });

    let _ = use_event_listener(_ref, ev::timeupdate, move |_evt| {
        let Some(video) = _ref() else {
            return;
        };

        let duration = video.duration();
        let current_time = video.current_time();
        let percentage_watched = ((current_time / duration) * 100.0) as u8;

        if current_time < 0.95 * duration {
            video_views_watch_multiple.set(false);
        }

        if percentage_watched >= 95 && !video_views_watch_multiple.get() {
            send_view_detail_action.dispatch((percentage_watched, 0_u8));

            video_views_watch_multiple.set(true);
        }
    });

    VideoWatched.send_event(post, _ref);

    view! {
        <VideoPlayer
            node_ref=_ref
            view_bg_url=Signal::derive(view_bg_url)
            view_video_url=Signal::derive(view_video_url)
        />
    }
}

#[component]
pub fn VideoViewForQueue(
    video_queue: RwSignal<Vec<PostDetails>>,
    current_idx: RwSignal<usize>,
    idx: usize,
    muted: RwSignal<bool>,
) -> impl IntoView {
    let container_ref = create_node_ref::<Video>();

    // Handles autoplay
    create_effect(move |_| {
        let Some(vid) = container_ref() else {
            return;
        };
        if idx != current_idx() {
            _ = vid.pause();
            return;
        }
        vid.set_autoplay(true);
        _ = vid.play();
    });

    let post = Signal::derive(move || video_queue.with(|q| q.get(idx).cloned()));

    view! {
        <VideoView
            post
            _ref=container_ref
            muted
        />
    }
}
