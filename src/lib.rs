use worker::*;

const SUBMIT_URL: &str = "https://api.scorer.gitcoin.co/registry/submit-passport";
const GET_URL: &str = "https://api.scorer.gitcoin.co/registry/score";

fn get_secrets(ctx: &RouteContext<()>) -> worker::Result<(String, String)> {
    let api_key = ctx.secret("PASSPORT_API_KEY")?.to_string();
    let scorer_id = ctx.secret("PASSPORT_SCORER_ID")?.to_string();
    Ok((api_key, scorer_id))
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> worker::Result<Response> {
    let router = Router::new();
    router
        .get_async("/submit/:address", |_, ctx| async move {
            if let Some(address) = ctx.param("address") {
                let (api_key, scorer_id) = get_secrets(&ctx)?;

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

                let request = Request::new_with_init(SUBMIT_URL, &init)?;
                return Fetch::Request(request).send().await;
            }
            Response::error("Bad Request", 400)
        })
        .get_async("/get/:address", |_req, ctx| async move {
            if let Some(address) = ctx.param("address") {
                let (api_key, scorer_id) = get_secrets(&ctx)?;

                let mut headers: Headers = Headers::new();
                headers.append("X-API-KEY", &api_key)?;

                let url = format!("{}/{}/{}", GET_URL, scorer_id, address);
                let mut init = RequestInit::new();
                init.with_method(Method::Get);
                init.with_headers(headers);

                let request = Request::new_with_init(&url, &init)?;
                return Fetch::Request(request).send().await;
            }
            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}
