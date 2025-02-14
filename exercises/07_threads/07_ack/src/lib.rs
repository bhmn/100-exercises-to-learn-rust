use data::{Ticket, TicketDraft};
use std::sync::mpsc::{Receiver, Sender};
use store::TicketId;

use crate::store::TicketStore;

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_sender: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_sender: Sender<Option<Ticket>>,
    },
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_sender,
            }) => {
                // todo!()
                let ticket_id = store.add_ticket(draft);
                let _ = response_sender.send(ticket_id);
            }
            Ok(Command::Get {
                id,
                response_sender,
            }) => {
                // todo!()
                let ticket = store.get(id);
                /*
                Comparison cloned() with clone()
                clone() works directly on Option<T> (not Option<&T>) and clones the entire Option:
                //--------------------------------------------------------------------------------------------
                let a = Some(String::from("hello"));
                let b = a.clone(); // Clones the entire Option<String>
                //--------------------------------------------------------------------------------------------
                cloned() works on Option<&T> and clones the value inside the reference:
                let a = Some(String::from("hello"));
                let b = a.as_ref().cloned(); // Clones the String inside the reference

                When to Use cloned()
                Use cloned() when you have an Option<&T> and want to convert it to an Option<T> by cloning the inner value.

                Use clone() when you want to clone the entire Option<T> itself.
                */
                //کلون کردن یه رفرنس یه کپی کامل از داده رو بهت میده که مالکش هستی نه رفرنس چون کلون کردن دیپ کپیه
                //اینم کار میکنه
                //let _ = response_sender.send(Some(ticket.unwrap().clone()));
                //ولی این یکی کوتاه تره
                let _ = response_sender.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
