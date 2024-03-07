
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

    let mut current_position = FloatVector3 { x: 0.0, y: 0.0, z: 0.0 };

    for i in 0..10 {
        let new_position = move_player(current_position.clone());
        
        let request = tonic::Request::new(
            MoveRequest {
                player_id: 0,
                last_position: Some(current_position.clone()),
                desired_position: Some(new_position.clone()),
            }
        );

        let response = client.move_player(request).await?;
        let res = response.into_inner();
        if(res.success) {
            current_position = new_position;
        }
        println!("RESPONSE: {:?}", res);
    }

    Ok(())
}

fn move_player(current_position: FloatVector3) -> FloatVector3 {
    let new_direction = random_movement();
    let new_position = FloatVector3 {
        x: current_position.x+new_direction.x, 
        y: current_position.y+new_direction.y, 
        z: current_position.z+new_direction.z
    };
    new_position
}

fn random_movement() -> FloatVector3 {
    FloatVector3 { x: 1.0, y: 0.0, z: 0.0 }
}