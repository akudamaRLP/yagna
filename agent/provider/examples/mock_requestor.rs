use serde_json;
use std::{ thread, time::{Duration, SystemTime}, };

use ya_client::{
    market::{ApiClient, RequestorApi},
    web::WebClient,
    Result,
};
use ya_model::market::{Agreement, Demand, Offer, Proposal, ProviderEvent, RequestorEvent};


async fn query_events(client: &RequestorApi, subscription_id: &str) -> Result<Vec<RequestorEvent>> {

    let mut requestor_events = vec![];

    while requestor_events.is_empty() {

        requestor_events = client
            .collect(&subscription_id, Some(1), Some(2)).await?;

        println!("Waiting for events");
        thread::sleep(Duration::from_millis(3000));
    }

    println!("{} events found.", requestor_events.len());
    return Ok(requestor_events);
}


async fn simulate_requestor(client: &RequestorApi) -> Result<()> {
    let demand = Demand::new(serde_json::json!({}), "(&(cpu.architecture=wasm32))".into());
    let subscription_id = client.subscribe(&demand).await?;

    println!("Demand created. Subscription_id {}.", &subscription_id);

    let requestor_events = query_events(client, &subscription_id).await?;

    let RequestorEvent::OfferEvent { offer, .. } = &requestor_events[0];
    let offer = offer.as_ref().unwrap();

    println!("Received offer {}. Sending agreeement {}.", &offer.id, &offer.id);

    let now = format!("{}", humantime::format_rfc3339_seconds(SystemTime::now()));
    let agreement = Agreement::new(offer.id.clone(), now);
    let res = client.create_agreement(&agreement).await?;

//    println!("Received offer {}. Sending new proposal {}.", &offer.id, &offer.id);
//
//    let proposal = client.get_proposal(&subscription_id, &offer.id).await?;
//    client.create_proposal(&proposal.demand, &subscription_id, &offer.id).await?;
//
//    let requestor_events = query_events(client, &subscription_id).await?;
//    let RequestorEvent::OfferEvent { offer, .. } = &requestor_events[0];



    Ok(())
}

async fn async_main(api: ApiClient) {
    if let Err(error) = simulate_requestor(api.requestor()).await {
        println!("Error: {}", error);
    };
}

fn main() {
    let client = ApiClient::new(WebClient::builder()).unwrap();

    actix_rt::System::new("test")
        .block_on(async_main(client));
}

