use log::*;
use model::article::Article;
use actix_web::{HttpResponse, web};
use common::msg::*;
#[macro_use]
use common::cli;
use common::cli::redis::cmd;



type SimpleResp = Result<HttpResponse, BusinessError>;

pub async fn save_article(args: web::Json<Article>) -> SimpleResp {
    let article: Article = args.into_inner();
    info!("save article, {:?}", article);

    let mut conn = cli::get_conn();
    cmd("SET").arg("key1").arg("我要分裂了").query(conn)?;
    Resp::ok(article.title).to_json_result()
}