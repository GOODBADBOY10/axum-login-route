use axum::extract::State;
use axum::Json;

use crate::model::{ModelController, TicketForCreate, Ticket};
use crate::Result;


// regions:       ------------------- REST Handlers
async fn create_ticket(State(mc): State<ModelController>, Json(ticket_fc): Json<TicketForCreate>) -> Result<Json<Ticket>> {
    println!("-->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}


// end-regions:       ------------------- REST Handlers