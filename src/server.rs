use ws::{CloseCode, Error, Handler, Handshake, Message, Result, Sender};

#[derive(Debug)]
pub struct Server {
    pub out: Option<Sender>,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // New connection
        let clone_out = self.out.clone();
        clone_out.unwrap().send("ok")
    }

    fn on_message(&mut self, _: Message) -> Result<()> {
        // Received a message
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            CloseCode::Abnormal => {
                println!("Closing handshake failed! Unable to obtain closing status from client.")
            }
            _ => println!("The client encountered an error: {}", reason),
        }
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}
