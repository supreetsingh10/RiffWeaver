use riff_weaver::rustipy::token::get_access_token; 
use std::error::Error; 
use tokio::runtime::Builder; 
use riff_weaver::types::AccessToken;


async fn access_token_rt() {
    if let Ok(fu) = get_access_token().await {
        println!("{}", fu); 
    }
}

fn main() ->Result<(), Box<dyn Error>> {
    let lmao = Builder::new_current_thread()
        .enable_all()
        .build()
        .map(|rt| {
                rt.block_on(async { access_token_rt().await});
        });

    Ok(())
}
