 use rspotify::{clients::{BaseClient, OAuthClient}, model::TimeLimits, AuthCodePkceSpotify};

pub async fn get_recently_played(spotify: &AuthCodePkceSpotify, num: Option<u32>, tl: Option<TimeLimits>) 
{
    let resp = spotify.current_user_recently_played(num, tl).await.unwrap();
    // loop through the results. 

    for i in resp.items {
        println!("{:?}", i.track.name);
    }
}

pub async fn get_user(spotify: &AuthCodePkceSpotify) 
{
    let u = spotify.current_user().await.unwrap(); 
    println!("{:?}", u);
}

pub async fn stop_song(spotify: &AuthCodePkceSpotify) 
{
    spotify.pause_playback(None).await;
}
