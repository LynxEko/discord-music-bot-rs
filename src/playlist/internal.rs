use std::collections::VecDeque;

use songbird::tracks::TrackHandle;

#[derive(Default)]
pub enum PlayMode {
    Playing,
    #[default]
    Shuffle,
}

#[derive(Default)]
pub struct Playlist {
    play_state: PlayMode,
    _paused: bool,
    loop_song_list: Vec<String>,
    immideate_song_list: VecDeque<String>,
    pub current_track: Option<TrackHandle>,
}

impl Playlist {
    pub async fn get_current_track(&mut self) -> Option<TrackHandle> {
        match &self.current_track {
            Some(ct) => {
                if ct
                    .get_info()
                    .await
                    .is_ok_and(|info| !info.playing.is_done())
                {
                } else {
                    self.current_track = None;
                }

                self.current_track.clone()
            }
            None => None,
        }
    }

    pub fn set_current_track(&mut self, current_track: TrackHandle) {
        self.current_track = Some(current_track)
    }

    pub fn next_song(&mut self) -> Option<String> {
        if let Some(song) = self.immideate_song_list.pop_front() {
            Some(song)
        } else {
            match &mut self.play_state {
                PlayMode::Playing => {
                    let song = self
                        .loop_song_list
                        .first()
                        .and_then(|song| Some(song.clone()));
                    if song.is_some() {
                        self.loop_song_list.remove(0);
                    }
                    song
                }
                PlayMode::Shuffle => {
                    let r = rand::random::<u64>() % self.loop_song_list.len() as u64;
                    Some(self.loop_song_list.remove(r as usize))
                }
            }
        }
    }

    pub fn add_to_playlist(&mut self, song: String) {
        self.loop_song_list.push(song);
    }

    pub fn add_now(&mut self, song: String) {
        self.immideate_song_list.push_back(song);
    }

    pub fn switch_playstate(&mut self) -> PlayMode {
        match self.play_state {
            PlayMode::Playing => PlayMode::Shuffle,
            PlayMode::Shuffle => PlayMode::Playing,
        }
    }
}
