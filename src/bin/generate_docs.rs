use utoipa::OpenApi;
use opengame_api::ApiDoc;

fn main() {
    let openapi = ApiDoc::openapi();
    std::fs::write("docs/openapi.json", openapi.to_json().unwrap()).unwrap();
}
