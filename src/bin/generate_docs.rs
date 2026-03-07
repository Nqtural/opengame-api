use opengame_api::ApiDoc;
use utoipa::OpenApi;

fn main() {
    let openapi = ApiDoc::openapi();
    std::fs::write("docs/openapi.json", openapi.to_json().unwrap()).unwrap();
}
