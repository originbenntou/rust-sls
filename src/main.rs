use lambda_http::{
    IntoResponse, Request, RequestExt, handler,
    lambda_runtime::{self, Context, Error}
};
use serde_json::json;

#[macro_use]
extern crate maplit;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(test_func)).await?;
    Ok(())
}

async fn test_func(
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

    #[test]
    fn handler_handles() {
        let mocked = hashmap! {
            "name".into() => vec!["Test".into()]
        };

        let request: Request<> = Request::default().with_query_string_parameters(mocked);
        let ctx = Context::default();

        let response = test_func(request, ctx)
            .expect("expected Ok(_) value")
            .into_response();

        let expected = json!({
            "response": "Test"
        }).into_response();

        assert_eq!(response.body(), expected.body())
    }
}
