
use player_input::player_action_client::PlayerActionClient;
use common::FloatVector3;
use player_input::{MoveRequest, MoveResponse};

pub mod common {
    tonic::include_proto!("common");
}

pub mod player_input {
    tonic::include_proto!("player_input");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Client startup");

    let mut client = PlayerActionClient::connect(
        "http://[::1]:5004"
    ).await?;

    let request = tonic::Request::new(
        MoveRequest {
            player_id: 0,
            last_position: Some(FloatVector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            desired_position: Some(FloatVector3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            }),
        }
    );

    let response = client.move_player(request).await?;

    let res = response.into_inner();
    println!("RESPONSE: {:?}", res);

    Ok(())
}