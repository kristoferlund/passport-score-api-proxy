use worker::*;

const PASSPORT_API_URL: &str = "https://api.scorer.gitcoin.co/registry/submit-passport";

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> worker::Result<Response> {
    let router = Router::new();
    router
        .get_async("/:address", |_req, ctx| async move {
            if let Some(address) = ctx.param("address") {
                let api_key = ctx.secret("PASSPORT_API_KEY")?.to_string();
                let scorer_id = ctx.secret("PASSPORT_SCORER_ID")?.to_string();

                let mut headers: Headers = Headers::new();
                headers.append("X-API-KEY", &api_key)?;
                headers.append("Content-Type", "application/json")?;

                let body = serde_json::json!({
                    "address": address,
                    "scorer_id": scorer_id,
                })
                .to_string();

                let mut init = RequestInit::new();
                init.with_method(Method::Post);
                init.with_body(Some(body.into()));
                init.with_headers(headers);

                let request = Request::new_with_init(PASSPORT_API_URL, &init)?;
                return Fetch::Request(request).send().await;
            }

            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}
