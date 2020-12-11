use netlify_lambda_http::{
  lambda::{lambda, Context},
  IntoResponse, Request, RequestExt,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda(http)]
#[tokio::main]
async fn main(request: Request, _: Context) -> Result<impl IntoResponse, Error> {
  let response = format!(
    "🦀🦀 Hello, {} 🦀🦀",
    request
      .query_string_parameters()
      .get("name")
      .unwrap_or_else(|| "stranger")
  );
  Ok(response)
}
