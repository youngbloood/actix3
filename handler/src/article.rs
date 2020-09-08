use log::*;
use model::article::Article;
use actix_web::{HttpResponse, web};
use common::msg::*;


type SimpleResp = Result<HttpResponse, BusinessError>;

pub async fn save_article(args: web::Json<Article>) -> SimpleResp {
    let article: Article = args.into_inner();
    info!("save article, {:?}", article);
    Resp::ok(article.title).to_json_result()
}