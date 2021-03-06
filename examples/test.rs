use hyper::{Body, Response, Server};
use routerify::prelude::*;
use routerify::{Middleware, Router, RouterService};
use std::net::SocketAddr;

fn router_explore() -> Router<Body, routerify::Error> {
    Router::builder()
        .middleware(Middleware::pre(|req| async move {
            // println!("Explore: {}", req.remote_addr());
            Ok(req)
        }))
        .get("/users/:userName/:data/abc", |req| async move {
            // dbg!(req.param("apiType"));
            dbg!(req.params());
            let user_name = req.param("userName").unwrap();
            Ok(Response::new(user_name.to_string().into()))
        })
        .post("/data/:dataName", |req| async move {
            let data_name = req.param("dataName").unwrap();
            Ok(Response::new(data_name.to_string().into()))
        })
        .build()
        .unwrap()
}

fn router_gallery() -> Router<Body, routerify::Error> {
    Router::builder()
        .middleware(Middleware::pre(|req| async move {
            // println!("Gallery: {}", req.remote_addr());
            Ok(req)
        }))
        .get("/ip/:ip", |req| async move {
            let ip = req.param("ip").unwrap();
            Ok(Response::new(ip.to_string().into()))
        })
        .post("/charts/:chart", |req| async move {
            let chart = req.param("chart").unwrap();
            Ok(Response::new(chart.to_string().into()))
        })
        .build()
        .unwrap()
}

fn router_v1() -> Router<Body, routerify::Error> {
    Router::builder()
        .get("/ping", |req| async move {
            // dbg!(req.param("apiType"));
            Ok(Response::new("ping".into()))
        })
        .scope("/explore", router_explore())
        .scope("/gallery", router_gallery())
        .middleware(Middleware::post(|res| async move {
            // println!("transformed resp v1");
            Ok(res)
        }))
        .middleware(
            Middleware::post_with_path("/abc", |res| async move {
                // println!("abc transformed resp v1");
                Ok(res)
            })
            .unwrap(),
        )
        .build()
        .unwrap()
}

fn router_api() -> Router<Body, routerify::Error> {
    Router::builder().scope("/v1", router_v1()).build().unwrap()
}

fn _router() -> Router<Body, routerify::Error> {
    Router::builder()
        .middleware(
            Middleware::pre_with_path("/abc", |req| async move {
                // println!("pre /abc");
                Ok(req)
            })
            .unwrap(),
        )
        .middleware(Middleware::pre(|req| async move {
            // println!("root: {}", req.remote_addr());
            Ok(req)
        }))
        .middleware(Middleware::post(|res| async move {
            // println!("transformed resp");
            Ok(res)
        }))
        .middleware(
            Middleware::post_with_path("/*", |res| async move {
                // println!("abc transformed resp");
                // Err(routerify::Error::new("remote addr error"))
                Ok(res)
            })
            .unwrap(),
        )
        // .get("/", |req| async move { Ok(Response::new("Home".into())) })
        .any_method("/", |_req| async move { Ok(Response::new(Body::from("hey"))) })
        .scope("/api", router_api())
        // .any(|req| async move { Ok(Response::new("io: Not Found".into())) })
        // .err_handler(|err| async move { Response::new(format!("Something went wrong!: {}", err).into()) })
        .build()
        .unwrap()
}

fn router() -> Router<Body, routerify::Error> {
    Router::builder()
        .middleware(Middleware::post(|res| async move {
            // println!("Post without info");
            Ok(res)
        }))
        // .middleware(Middleware::post_with_info(|res, req_info| async move {
        //     // println!("Post with info: {:?}", req_info.headers());
        //     Ok(res)
        // }))
        // .middleware(Middleware::post_with_info(|res, req_info| async move {
        //     // println!("Post with info: {:?}", req_info.headers());
        //     Ok(res)
        // }))
        // .middleware(Middleware::post_with_info(|res, req_info| async move {
        //     // println!("Post with info: {:?}", req_info.headers());
        //     Ok(res)
        // }))
        .get("/", |req| async move { Err(routerify::Error::new("Hey error")) })
        // .get("/", |req| async move { Ok(Response::new(Body::from("Hello world!"))) })
        .err_handler(|err| async move {
            println!("err without info");
            Response::new(Body::from("Err response"))
        })
        // .err_handler_with_info(|err, req_info| async move {
        //     println!("err2 without info: {:?}", req_info);
        //     Response::new(Body::from("Err response2"))
        // })
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router = router();
    let service = RouterService::new(router).unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = Server::bind(&addr).serve(dbg!(service));

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
