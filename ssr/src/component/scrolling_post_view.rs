use leptos::*;
use leptos_icons::*;
use leptos_use::{use_intersection_observer_with_options, UseIntersectionObserverOptions};

use crate::page::post_view::video_loader::{BgView, VideoView};

use crate::utils::posts::PostDetails;
#[cfg(feature = "wasm-bindgen")]
use js_sys::Date;
#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[component]
pub fn ScrollingPostView<F: Fn() -> V + Clone + 'static, V, O: Fn() -> IV, IV: IntoView>(
    video_queue: RwSignal<Vec<PostDetails>>,
    current_idx: RwSignal<usize>,
    #[prop(optional)] fetch_next_videos: Option<F>,
    recovering_state: RwSignal<bool>,
    queue_end: RwSignal<bool>,
    #[prop(optional)] overlay: Option<O>,
) -> impl IntoView {
    let muted = create_rw_signal(true);
    let scroll_root: NodeRef<html::Div> = create_node_ref();
    let last_muted_change = create_rw_signal(js_sys::Date::now()); // Initialize with current time (f64)

    let var_name = view! {
            <div class="h-full w-full overflow-hidden overflow-y-auto">
                <div
                    _ref=scroll_root
                    class="snap-mandatory snap-y overflow-y-scroll h-dvh w-dvw bg-black"
                    style:scroll-snap-points-y="repeat(100vh)"
                >

                    {overlay.map(|o| o())}

                    <For
                        each=move || video_queue().into_iter().enumerate()
                        key=move |(_, details)| (details.canister_id, details.post_id)
                        children=move |(queue_idx, _details)| {
                            let container_ref = create_node_ref::<html::Div>();
                            let next_videos = fetch_next_videos.clone();
                            use_intersection_observer_with_options(
                                container_ref,
                                move |entry, _| {
                                    let Some(visible) = entry.first().filter(|e| e.is_intersecting())
                                    else {
                                        return;
                                    };
                                    let rect = visible.bounding_client_rect();
                                    if rect.y() == rect.height()
                                        || queue_idx == current_idx.get_untracked()
                                    {
                                        return;
                                    }
                                    if video_queue.with_untracked(|q| q.len()).saturating_sub(queue_idx)
                                        <= 10
                                    {
                                        next_videos.as_ref().map(|nv| { nv() });
                                    }
                                    current_idx.set(queue_idx);
                                },
                                UseIntersectionObserverOptions::default()
                                    .thresholds(vec![0.83])
                                    .root(Some(scroll_root)),
                            );
                            create_effect(move |_| {
                                let Some(container) = container_ref() else {
                                    return;
                                };
                                if current_idx() == queue_idx && recovering_state.get_untracked() {
                                    container.scroll_into_view();
                                    recovering_state.set(false);
                                }
                            });
                            let show_video = create_memo(move |_| {
                                queue_idx.abs_diff(current_idx()) <= 20
                            });
                            view! {
                                <div _ref=container_ref class="snap-always snap-end w-full h-full">
                                    <Show when=show_video>
                                        <BgView video_queue current_idx idx=queue_idx>
                                            <VideoView video_queue current_idx idx=queue_idx muted/>
                                        </BgView>
                                    </Show>
                                </div>
                            }
                        }
                    />

                    <Show when=queue_end>
                        <div class="h-full w-full relative top-0 left-0 bg-inherit z-[21] flex snap-always snap-end justify-center items-center text-xl text-white/80">
                            <span>You have reached the end!</span>
                        </div>
                    </Show>
    <Show when=muted>
        <button
            class="fixed top-1/2 left-1/2 z-20 cursor-pointer"
            on:click=move |_| {
                muted.set(false);
                last_muted_change.set(js_sys::Date::now());
            }
        >
            <Icon
                class=format!("text-white/80 text-4xl {}",
                if js_sys::Date::now() - last_muted_change.get() < 3000.0 { "flash-animation" } else {""  })
                icon=icondata::BiVolumeMuteSolid
            />
        </button>
    </Show>

                </div>
            </div>
        };
    var_name
}
