use infinispan::request::ToHttpRequest;
use infinispan::Infinispan;
use reqwest::Response;

pub fn infinispan_client() -> Infinispan {
    Infinispan::new("http://localhost:11222", "username", "password")
}

pub async fn run<R: ToHttpRequest>(request: &R) -> Response {
    infinispan_client().run(request).await.unwrap()
}

pub async fn read_body(response: Response) -> String {
    response.text_with_charset("utf-8").await.unwrap()
}
