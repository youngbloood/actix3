use log::*;
use model::article::Article;
use actix_web::{HttpResponse, web};
use common::msg::*;
use common::cli::myredis;
use common::cli::redis::cmd;



type SimpleResp = Result<HttpResponse, BusinessError>;

pub async fn save_article(args: web::Json<Article>) -> SimpleResp {
    let article: Article = args.into_inner();
    info!("save article, {:?}", article);

    let mut conn = myredis::get_conn();
    let result_t= cmd("SET").arg("key1").arg("我要分裂了").query::<()>(&mut conn);
    let result =  result_t.unwrap();
    println!("写入redis的结果:{:?}",result);
    Resp::ok(article.title).to_json_result()
}