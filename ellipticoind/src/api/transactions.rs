use super::views::Transaction;
use super::State;
use crate::diesel::OptionalExtension;
use crate::models;
use crate::network::Message;
use crate::schema::transactions::dsl::transactions;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use http_service::Body;
use serde_cbor::from_slice;
use tide::Response;
use vm::redis::Commands;

pub async fn show(req: tide::Request<State>) -> Response {
    let con = req.state().db.get().unwrap();
    let transaction_hash: String = req.param("transaction_hash").unwrap();
    let transaction = transactions
        .find(base64::decode_config(&transaction_hash, base64::URL_SAFE).unwrap())
        .first::<models::Transaction>(&con)
        .optional()
        .unwrap();

    if let Some(transaction) = transaction {
        Response::new(200).body(Body::from(
            serde_cbor::to_vec(&Transaction::from(&transaction)).unwrap(),
        ))
    } else {
        Response::new(404)
    }
}
pub async fn create(mut req: tide::Request<State>) -> Response {
    let transaction_bytes = req.body_bytes().await.unwrap();
    let transaction: vm::Transaction = from_slice(&transaction_bytes).unwrap();
    let mut network_sender = req.state().network_sender.clone();
    network_sender
        .send(&Message::Transaction(transaction))
        .await;
    let mut redis = req.state().redis.get_connection().unwrap();
    redis
        .rpush::<&str, Vec<u8>, ()>("transactions::pending", transaction_bytes)
        .unwrap();

    Response::new(201)
}
