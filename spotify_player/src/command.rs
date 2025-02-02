use crate::state::{Album, Artist, DataReadGuard, Playlist, Track};
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
/// Application's command
pub enum Command {
    None,

    NextTrack,
    PreviousTrack,
    ResumePause,
    PlayRandom,
    Repeat,
    Shuffle,
    VolumeUp,
    VolumeDown,
    Mute,
    SeekForward,
    SeekBackward,

    Quit,
    OpenCommandHelp,
    ClosePopup,

    SelectNextOrScrollDown,
    SelectPreviousOrScrollUp,
    PageSelectNextOrScrollDown,
    PageSelectPreviousOrScrollUp,
    SelectFirstOrScrollToTop,
    SelectLastOrScrollToBottom,

    ChooseSelected,

    RefreshPlayback,

    #[cfg(feature = "streaming")]
    RestartIntegratedClient,

    FocusNextWindow,
    FocusPreviousWindow,

    SwitchTheme,
    SwitchDevice,
    Search,
    Queue,

    ShowActionsOnSelectedItem,
    ShowActionsOnCurrentTrack,
    AddSelectedItemToQueue,

    BrowseUserPlaylists,
    BrowseUserFollowedArtists,
    BrowseUserSavedAlbums,

    CurrentlyPlayingContextPage,
    TopTrackPage,
    RecentlyPlayedTrackPage,
    LikedTrackPage,
    #[cfg(feature = "lyric-finder")]
    LyricPage,
    LibraryPage,
    SearchPage,
    BrowsePage,
    PreviousPage,
    #[cfg(feature = "clipboard")]
    OpenSpotifyLinkFromClipboard,

    SortTrackByTitle,
    SortTrackByArtists,
    SortTrackByAlbum,
    SortTrackByDuration,
    SortTrackByAddedDate,
    ReverseTrackOrder,

    MovePlaylistItemUp,
    MovePlaylistItemDown,
}

#[derive(Debug, Copy, Clone)]
pub enum TrackAction {
    GoToArtist,
    GoToAlbum,
    GoToTrackRadio,
    ShowActionsOnAlbum,
    ShowActionsOnArtist,
    AddToQueue,
    AddToPlaylist,
    DeleteFromCurrentPlaylist,
    AddToLikedTracks,
    DeleteFromLikedTracks,
    CopyTrackLink,
}

#[derive(Debug, Copy, Clone)]
pub enum AlbumAction {
    GoToArtist,
    GoToAlbumRadio,
    ShowActionsOnArtist,
    AddToLibrary,
    DeleteFromLibrary,
    CopyAlbumLink,
}

#[derive(Debug, Copy, Clone)]
pub enum ArtistAction {
    GoToArtistRadio,
    Follow,
    Unfollow,
    CopyArtistLink,
}

#[derive(Debug, Copy, Clone)]
pub enum PlaylistAction {
    GoToPlaylistRadio,
    AddToLibrary,
    DeleteFromLibrary,
    CopyPlaylistLink,
}

/// constructs a list of actions on a track
pub fn construct_track_actions(track: &Track, data: &DataReadGuard) -> Vec<TrackAction> {
    let mut actions = vec![
        TrackAction::GoToArtist,
        TrackAction::GoToAlbum,
        TrackAction::GoToTrackRadio,
        TrackAction::ShowActionsOnAlbum,
        TrackAction::ShowActionsOnArtist,
        TrackAction::CopyTrackLink,
        TrackAction::AddToPlaylist,
        TrackAction::AddToQueue,
    ];

    // check if the track is a liked track
    if data.user_data.is_liked_track(track) {
        actions.push(TrackAction::DeleteFromLikedTracks);
    } else {
        actions.push(TrackAction::AddToLikedTracks);
    }

    actions
}

/// constructs a list of actions on an album
pub fn construct_album_actions(album: &Album, data: &DataReadGuard) -> Vec<AlbumAction> {
    let mut actions = vec![
        AlbumAction::GoToArtist,
        AlbumAction::GoToAlbumRadio,
        AlbumAction::ShowActionsOnArtist,
        AlbumAction::CopyAlbumLink,
    ];
    if data.user_data.saved_albums.iter().any(|a| a.id == album.id) {
        actions.push(AlbumAction::DeleteFromLibrary);
    } else {
        actions.push(AlbumAction::AddToLibrary);
    }
    actions
}

/// constructs a list of actions on an artist
pub fn construct_artist_actions(artist: &Artist, data: &DataReadGuard) -> Vec<ArtistAction> {
    let mut actions = vec![ArtistAction::GoToArtistRadio, ArtistAction::CopyArtistLink];
    if data
        .user_data
        .followed_artists
        .iter()
        .any(|a| a.id == artist.id)
    {
        actions.push(ArtistAction::Unfollow);
    } else {
        actions.push(ArtistAction::Follow);
    }
    actions
}

/// constructs a list of actions on an playlist
pub fn construct_playlist_actions(
    playlist: &Playlist,
    data: &DataReadGuard,
) -> Vec<PlaylistAction> {
    let mut actions = vec![
        PlaylistAction::GoToPlaylistRadio,
        PlaylistAction::CopyPlaylistLink,
    ];
    if data.user_data.playlists.iter().any(|a| a.id == playlist.id) {
        actions.push(PlaylistAction::DeleteFromLibrary);
    } else {
        actions.push(PlaylistAction::AddToLibrary);
    }
    actions
}

impl Command {
    pub fn desc(&self) -> &'static str {
        match self {
            Self::None => "do nothing",
            Self::NextTrack => "next track",
            Self::PreviousTrack => "previous track",
            Self::ResumePause => "resume/pause based on the current playback",
            Self::PlayRandom => "play a random track in the current context",
            Self::Repeat => "cycle the repeat mode",
            Self::Shuffle => "toggle the shuffle mode",
            Self::VolumeUp => "increase playback volume by 5%",
            Self::VolumeDown => "decrease playback volume by 5%",
            Self::Mute => "toggle playback volume between 0% and previous level",
            Self::SeekForward => "seek forward by 5s",
            Self::SeekBackward => "seek backward by 5s",
            Self::Quit => "quit the application",
            Self::OpenCommandHelp => "open a command help popup",
            Self::ClosePopup => "close a popup",
            #[cfg(feature = "streaming")]
            Self::RestartIntegratedClient => "restart the integrated librespot client",
            Self::SelectNextOrScrollDown => "select the next item in a list/table or scroll down",
            Self::SelectPreviousOrScrollUp => {
                "select the previous item in a list/table or scroll up"
            }
            Self::PageSelectNextOrScrollDown => {
                "select the next page item in a list/table or scroll a page down"
            }
            Self::PageSelectPreviousOrScrollUp => {
                "select the previous page item in a list/table or scroll a page up"
            }
            Self::SelectFirstOrScrollToTop => {
                "select the first item in a list/table or scroll to the top"
            }
            Self::SelectLastOrScrollToBottom => {
                "select the last item in a list/table or scroll to the bottom"
            }
            Self::ChooseSelected => "choose the selected item and act on it",
            Self::RefreshPlayback => "manually refresh the current playback",
            Self::ShowActionsOnSelectedItem => "open a popup showing actions on a selected item",
            Self::ShowActionsOnCurrentTrack => "open a popup showing actions on the current track",
            Self::AddSelectedItemToQueue => "add the selected item to queue",
            Self::FocusNextWindow => "focus the next focusable window (if any)",
            Self::FocusPreviousWindow => "focus the previous focusable window (if any)",
            Self::SwitchTheme => "open a popup for switching theme",
            Self::SwitchDevice => "open a popup for switching device",
            Self::Search => "open a popup for searching in the current page",
            Self::Queue => "open a popup for showing the current queue",
            Self::BrowseUserPlaylists => "open a popup for browsing user's playlists",
            Self::BrowseUserFollowedArtists => "open a popup for browsing user's followed artists",
            Self::BrowseUserSavedAlbums => "open a popup for browsing user's saved albums",
            Self::CurrentlyPlayingContextPage => "go to the currently playing context page",
            Self::TopTrackPage => "go to the user top track page",
            Self::RecentlyPlayedTrackPage => "go to the user recently played track page",
            Self::LikedTrackPage => "go to the user liked track page",
            #[cfg(feature = "lyric-finder")]
            Self::LyricPage => "go to the lyric page of the current track",
            Self::LibraryPage => "go to the user library page",
            Self::SearchPage => "go to the search page",
            Self::BrowsePage => "go to the browse page",
            Self::PreviousPage => "go to the previous page",
            #[cfg(feature = "clipboard")]
            Self::OpenSpotifyLinkFromClipboard => "open a Spotify link from clipboard",
            Self::SortTrackByTitle => "sort the track table (if any) by track's title",
            Self::SortTrackByArtists => "sort the track table (if any) by track's artists",
            Self::SortTrackByAlbum => "sort the track table (if any) by track's album",
            Self::SortTrackByDuration => "sort the track table (if any) by track's duration",
            Self::SortTrackByAddedDate => "sort the track table (if any) by track's added date",
            Self::ReverseTrackOrder => "reverse the order of the track table (if any)",
            Self::MovePlaylistItemUp => "move playlist item up one position",
            Self::MovePlaylistItemDown => "move playlist item down one position",
        }
    }
}
