extern crate futures;
extern crate actix;
use actix::prelude::*;
use futures::future::Future;

// message format
struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

// Actor
struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Context<Self>) {
       println!("Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
       println!("Actor is stopped");
    }
}

// Actor handles message
impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>)
        -> Self::Result {
            self.count += msg.0;
            self.count
    }
}

// run the app
fn main() {
    let system = System::new("test");
    let addr = MyActor{count: 1}.start();

    let res = addr.send(Ping(10));
    Arbiter::spawn(
        res
        .map(|res| {println!("RESULT: {}", res)})
        .map_err(|err| println!("ERR: {}", err))
        .map(|_| System::current().stop()));

    system.run();
}
