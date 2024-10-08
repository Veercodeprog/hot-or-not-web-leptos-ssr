use candid::Principal;
use serde::{Deserialize, Serialize};
use web_time::Duration;

use crate::{
    canister::individual_user_template::PostDetailsForFrontend, state::canisters::Canisters,
};

use super::{profile::propic_from_principal, types::PostStatus};

use ic_agent::AgentError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PostViewError {
    #[error("IC agent error {0}")]
    Agent(#[from] AgentError),
    #[error("Canister error {0}")]
    Canister(String),
    #[error("http fetch error {0}")]
    HttpFetch(#[from] reqwest::Error),
    #[error("ml feed error {0}")]
    MLFeedError(String),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FetchCursor {
    pub start: u64,
    pub limit: u64,
}

impl Default for FetchCursor {
    fn default() -> Self {
        Self {
            start: 0,
            limit: 10,
        }
    }
}

impl FetchCursor {
    pub fn advance(&mut self) {
        self.start += self.limit;
        self.limit = 25;
    }

    pub fn set_limit(&mut self, limit: u64) {
        self.limit = limit;
    }

    pub fn advance_and_set_limit(&mut self, limit: u64) {
        self.start += self.limit;
        self.limit = limit;
    }
}

#[derive(Clone, PartialEq, Debug, Hash, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PostDetails {
    pub canister_id: Principal, // canister id of the publishing canister.
    pub post_id: u64,
    pub uid: String,
    pub description: String,
    pub views: u64,
    pub likes: u64,
    pub display_name: String,
    pub propic_url: String,
    /// Whether post is liked by the authenticated
    /// user or not, None if unknown
    pub liked_by_user: Option<bool>,
    pub poster_principal: Principal,
    pub hastags: Vec<String>,
    pub is_nsfw: bool,
    pub hot_or_not_feed_ranking_score: Option<u64>,
    pub created_at: Duration,
}

impl PostDetails {
    pub fn from_canister_post(
        authenticated: bool,
        canister_id: Principal,
        details: PostDetailsForFrontend,
    ) -> Self {
        Self {
            canister_id,
            post_id: details.id,
            uid: details.video_uid,
            description: details.description,
            views: details.total_view_count,
            likes: details.like_count,
            display_name: details
                .created_by_display_name
                .or(details.created_by_unique_user_name)
                .unwrap_or_else(|| details.created_by_user_principal_id.to_text()),
            propic_url: details
                .created_by_profile_photo_url
                .unwrap_or_else(|| propic_from_principal(details.created_by_user_principal_id)),
            liked_by_user: authenticated.then_some(details.liked_by_me),
            poster_principal: details.created_by_user_principal_id,
            hastags: details.hashtags,
            is_nsfw: details.is_nsfw,
            hot_or_not_feed_ranking_score: details.hot_or_not_feed_ranking_score,
            created_at: Duration::new(
                details.created_at.secs_since_epoch,
                details.created_at.nanos_since_epoch,
            ),
        }
    }

    pub fn is_hot_or_not(&self) -> bool {
        self.hot_or_not_feed_ranking_score.is_some()
    }
}

pub async fn get_post_uid<const AUTH: bool>(
    canisters: &Canisters<AUTH>,
    user_canister: Principal,
    post_id: u64,
) -> Result<Option<PostDetails>, PostViewError> {
    let post_creator_can = canisters.individual_user(user_canister).await?;
    let post_details = match post_creator_can
        .get_individual_post_details_by_id(post_id)
        .await
    {
        Ok(p) => p,
        Err(e) => {
            log::warn!(
                "failed to get post details for {} {}: {}, skipping",
                user_canister.to_string(),
                post_id,
                e
            );
            return Ok(None);
        }
    };

    // TODO: temporary patch in frontend to not show banned videos, to be removed later after NSFW tagging
    if PostStatus::from(&post_details.status) == PostStatus::BannedDueToUserReporting {
        return Ok(None);
    }

    let post_uuid = &post_details.video_uid;
    let req_url = format!(
        "https://customer-2p3jflss4r4hmpnz.cloudflarestream.com/{}/manifest/video.m3u8",
        post_uuid,
    );
    let res = reqwest::Client::default().head(req_url).send().await;
    if res.is_err() || (res.is_ok() && res.unwrap().status() != 200) {
        return Ok(None);
    }

    Ok(Some(PostDetails::from_canister_post(
        AUTH,
        user_canister,
        post_details,
    )))
}

pub fn get_feed_component_identifier() -> impl Fn() -> Option<&'static str> {
    move || {
        let loc = get_host();

        if loc == "localhost:3000"
            || loc == "hotornot.wtf"
            || loc.contains("go-bazzinga-hot-or-not-web-leptos-ssr.fly.dev")
        // || loc == "hot-or-not-web-leptos-ssr-staging.fly.dev"
        {
            Some("PostViewWithUpdatesMLFeed")
        } else {
            Some("PostViewWithUpdates")
        }
    }
}

pub fn get_host() -> String {
    #[cfg(feature = "hydrate")]
    {
        use leptos::window;
        window().location().host().unwrap().to_string()
    }

    #[cfg(not(feature = "hydrate"))]
    {
        use axum::http::request::Parts;
        use http::header::HeaderMap;
        use leptos::expect_context;

        let parts: Parts = expect_context();
        let headers = parts.headers;
        headers.get("Host").unwrap().to_str().unwrap().to_string()
    }
}
