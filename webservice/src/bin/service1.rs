/*
Copyright 2022 The Workpieces LLC.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

pub fn gengeral_routers(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Actix Web service running ...");
}

// 实例化HTTP server并运行
#[actix_rt::main]
async fn main() -> io::Result<()> {
    let app = move || App::new().configure(gengeral_routers);

    HttpServer::new(app)
        .bind("127.0.0.1:8080")?
        .start()
        .await
        .map_err(Into::into)
}
