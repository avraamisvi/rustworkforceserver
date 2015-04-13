use nickel::{ Nickel, Request, Response };
use nickel::{ Response, Request };

pub struct ReceiveRouter {
    test: String
}

impl ReceiveRouter {

    pub fn new(message: String)->ReceiveRouter {
        ReceiveRouter{test:message}
    }
}

pub trait ReceiveRouterTrait {

    fn handle(&self)->String;

    fn handle1(req: Request, res: Response);
}

impl ReceiveRouterTrait for ReceiveRouter {

    fn handle(&self)->String {
        let st = self.test.as_str();
        String::from_str(st)
    }

    fn handle1(req: Request, res: Response) {
        "ola"
    }
}
