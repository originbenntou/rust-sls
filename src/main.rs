use lambda_http::{
    IntoResponse, Request, RequestExt, handler,
    lambda_runtime::{self, Context, Error}
};
use serde_json::json;

use rusoto_core::{Region};
use rusoto_s3::{S3, S3Client};

#[macro_use]
extern crate maplit;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(get_query)).await?;
    Ok(())
}

async fn get_query(
    request: Request,
    _ctx: Context
) -> Result<impl IntoResponse, Error> {
    let res = format!(
        "{}",
        request
            .query_string_parameters()
            .get("name")
            .unwrap_or_else(|| "stranger")
    );

    Ok(
        json!({
            "response": res
        })
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_query() {
        let mocked = hashmap! {
            "name".into() => vec!["Test".into()]
        };

        let request: Request<> = Request::default().with_query_string_parameters(mocked);
        let ctx = Context::default();

        let response = get_query(request, ctx).await
            .expect("expected Ok(_) value")
            .into_response();

        let expected = json!({
            "response": "Test"
        }).into_response();

        assert_eq!(response.body(), expected.body())
    }
}

fn default_profile_client(region: Region) -> S3Client {
    S3Client::new(region)
}
