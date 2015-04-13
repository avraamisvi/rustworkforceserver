#![feature(collections)]
#![feature(convert)]

#[macro_use] extern crate nickel_macros;
extern crate nickel;

mod alpar;

use nickel::{
    Nickel, NickelError, Continue, Halt, Response, Request,
    QueryString, JsonBody, HttpRouter
};
use alpar::routers::receive::ReceiveRouterTrait;
use alpar::routers::receive::ReceiveRouter;

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    fn test()->String {
        let myrouter = ReceiveRouter::new("ola mundo".to_string());
        myrouter.handle()
    }

    fn test2(req: Request, res: Response)->String {
        let myrouter = ReceiveRouter::new("ola mundo".to_string());
        myrouter.handle1()
    }

    router.post("/a/post/request", middleware! { |request, response|
        /*let person = request.json_as::<Person>().unwrap();
        format!("Hello {} {}", person.firstname, person.lastname)*/


    });


    router.get("/user", middleware! { |_req, _res|
        test()
    });

    server.utilize(router);
    server.listen("127.0.0.1:6767");
}
