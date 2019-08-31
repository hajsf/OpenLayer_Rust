use ws::Handler;
use crate::models::client::Client;
use ws::{Message, Request, Response, Result, CloseCode, Handshake};

impl Handler for Client {
    fn on_open(&mut self, hs: Handshake) -> Result<()> {
        self.handle_on_open(hs)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.handle_on_message(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        self.handle_on_close(code, reason)
    }

    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        self.handle_on_request(req)
    }
}
