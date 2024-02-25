use actix::prelude::*;

struct MyActor {
    pub count: usize,
}

impl Actor for crate::MyActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
struct Ping(usize);

impl Handler<Ping> for crate::MyActor {
    type Result = usize;

    fn handle(&mut self,
              msg: Ping,
              _ctx: &mut Context<Self>
    ) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}