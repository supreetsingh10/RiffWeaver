// Added for testing purposes. 
// use this to test the spotify api interaction thing. 
use libra::rustipy::token::{ get_access_token, AccessToken}; 
use tokio::runtime::Builder; 
use libra::rustipy::constants::DEBUG; 

async fn access_token_rt()-> Result<AccessToken, String > {
    get_access_token().await
}

fn main() ->Result<(), String> {
    let rt = Builder::new_current_thread()
        .enable_all()
        .build();

    let access_token = rt.map(|ree| {
        ree.block_on(async {
            if DEBUG {
                println!("Came here"); 
            }
            match access_token_rt().await {
                Ok(at) => {
                    DEBUG.then(|| println!("This is the token {}", at));
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

    println!("{}", at); 

    Ok(())
}
