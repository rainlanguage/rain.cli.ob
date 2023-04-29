use graphql_client::GraphQLQuery;
use graphql_client::Response;
use reqwest::Url;

use once_cell::sync::Lazy;

static BASE_URL: Lazy<Url> =
    Lazy::new(|| Url::parse("https://api.thegraph.com/subgraphs/name/siddharth2207/rainorderbook").unwrap());

    static PAGE_SIZE: i64 = 500;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/subgraph/orders/orders.schema.json",
    query_path = "src/subgraph/orders/orders.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct Orders;

pub async fn query() -> anyhow::Result<()> {
    let variables = orders_query::Variables {};
    let request_body = OrdersQuery::build_query(variables);
    let client = reqwest::Client::new();
    let res = client.post(BASE_URL).json(&request_body).send().await?;
    let response_body: Response<orders_query::ResponseData> = res.json().await?;
    match response_body {
        Response {
            data: Some(orders_query::ResponseData { orders }),
            ..
        } => {
            dbg!(&orders);
        },
        _ => { tracing::warn!("Failed to get orders"); }
    }
    Ok(())
    // let mut all = vec![];
    // let mut page: Vec<IPFSCID>;
    // let mut skip = 0;

    // loop {
    //     let variables = pin_query::Variables {
    //         ids: Some(authors.clone()),
    //         first: PAGE_SIZE,
    //         skip,
    //     };

    //     let request_body = PinQuery::build_query(variables);
    //     let client = reqwest::Client::new();
    //     let res = client
    //         .post(subgraph.url()?)
    //         .json(&request_body)
    //         .send()
    //         .await?;
    //     let response_body: Response<pin_query::ResponseData> = res.json().await?;
    //     page = match response_body {
    //         Response {
    //             data: Some(pin_query::ResponseData { hashes }),
    //             ..
    //         } => {
    //             hashes
    //                 .into_iter()
    //                 .filter_map(|pin_query_hashes| {
    //                     // Decode and drop any data that doesn't cleanly convert to a
    //                     // multihash.
    //                     bs58::decode(pin_query_hashes.hash)
    //                         .into_vec()
    //                         .map_err(anyhow::Error::from)
    //                         .and_then(|data| {
    //                             IPFSCID::from_bytes(&data).map_err(anyhow::Error::from)
    //                         })
    //                         .ok()
    //                 })
    //                 .collect()
    //         }
    //         _ => {
    //             vec![]
    //         }
    //     };
    //     if page.is_empty() {
    //         break;
    //     } else {
    //         tracing::info!("page length {} {}", subgraph.network, page.len());
    //         skip += (PAGE_SIZE-1);
    //         all.extend(page);
    //     }
    // }
    // Ok(all)
}