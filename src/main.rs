use riff_weaver::rustipy::token::get_access_token; 
use tokio::runtime::Builder; 
use riff_weaver::types::AccessToken;


// multithread this to make it refresh every 3600 seconds. 
async fn access_token_rt()-> Result<AccessToken, String > {
    get_access_token().await
}

fn main() ->Result<(), String> {
    let rt = Builder::new_current_thread()
        .enable_all()
        .build();

    let access_token = rt.map(|ree| {
        ree.block_on(async {
            println!("Came here"); 
            match access_token_rt().await {
                Ok(at) => {
                    println!("This is the token");
                    at
                },
                Err(e) => panic!("Jesus christ what happened {}", e),
            }
        })
    });

    let at = match access_token {
        Ok(access_token) => access_token,
        Err(e) => panic!("We failed {}", e.to_string()),
    };


    Ok(())
}
