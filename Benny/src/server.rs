use tonic::{transport::Server, Request, Response, Status};

use player_input::player_action_server::{PlayerAction, PlayerActionServer};
use player_input::{MoveRequest, MoveResponse};

pub mod common {
    tonic::include_proto!("common");
}
pub mod player_input {
    tonic::include_proto!("player_input");
}

#[derive(Debug, Default)]
pub struct PlayerActionService {}

#[tonic::async_trait]
impl PlayerAction for PlayerActionService {
    async fn move_player(&self, request: Request<MoveRequest>) -> Result<Response<MoveResponse>, Status> {
        println!("Received request: {:?}", request);

        let req = request.into_inner();

        let reply = MoveResponse {
            success: true,
            new_position: req.desired_position.clone(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Server startup");

    let addr = "[::1]:5004".parse()?;
    let player_service = PlayerActionService::default();

    Server::builder()
        .add_service(PlayerActionServer::new(player_service))
        .serve(addr)
        .await?;

    Ok(())
}
