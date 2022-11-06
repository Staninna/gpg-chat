// Rules
#![allow(dead_code)]

// Imports
use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

// Video data structure
#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

// Video list component
#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

// Implementing the component
#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    // Creating the list of videos
    let on_click = on_click.clone();

    // Creating the list of videos
    videos
        .iter()
        .map(|video| {
            let on_video_select = {
                let on_click = on_click.clone();
                let video = video.clone();
                Callback::from(move |_| on_click.emit(video.clone()))
            };

            html! {
                <p onclick={ on_video_select }>{ format!("{}: {}", video.title, video.speaker) }</p>
            }
        })
        .collect()
}

// Video details component
#[derive(Clone, Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

// Implementing the video details component
#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder"
                 alt="video thumbnail" />
        </div>
    }
}

// Main component
#[function_component(App)]
fn app() -> Html {
    // get videos from backend
    let videos = use_state(|| Vec::new());
    {
        // Clone the reference to the state
        let videos = videos.clone();

        // Fetch the videos from the backend
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos = Request::get("data.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    // Set selected video to none
    let selected_video = use_state(|| None);

    // Set new selected video
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    // Get details of selected video
    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    // Return the html for the component
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
                <VideosList videos={ (*videos).clone() } on_click={ on_video_select } />
            </div>
            { for details }
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
