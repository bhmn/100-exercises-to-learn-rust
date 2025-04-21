// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

// Replace `Mutex` with `RwLock` in the `TicketStore` struct and
//  all other relevant places to allow multiple readers to access the ticket store concurrently.
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};
use std::sync::{Arc, RwLock};

use crate::data::{Ticket, TicketDraft, TicketPatch};
use crate::store::{TicketId, TicketStore};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, OverloadedError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::Insert {
                draft,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        Ok(response_receiver.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Arc<RwLock<Ticket>>>, OverloadedError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::Get {
                id,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        Ok(response_receiver.recv().unwrap())
    }

    pub fn update(&self, ticket_patch: TicketPatch) -> Result<(), OverloadedError> {
        let (sender_response, reveive_response) = sync_channel(1);
        self.sender
            .try_send(Command::Update {
                patch: ticket_patch,
                response_channel: sender_response,
            })
            .map_err(|_| OverloadedError)?;

        Ok(reveive_response.recv().unwrap())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("The store is overloaded")]
pub struct OverloadedError;

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Arc<RwLock<Ticket>>>>,
    },
    Update {
        patch: TicketPatch,
        response_channel: SyncSender<()>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket);
            }
            Ok(Command::Update {
                patch,
                response_channel,
            }) => {
                //todo!()

                //از طریق if let version
                if let Some(ticket) = store.get_mut(patch.id) {
                    if let Some(title) = patch.title {
                        if let Ok(mut ticket) = ticket.write() {
                            ticket.title = title;
                        }
                    }
                    if let Some(description) = patch.description {
                        if let Ok(mut ticket) = ticket.write() {
                            ticket.description = description;
                        }
                    }

                    if let Some(status) = patch.status {
                        if let Ok(mut ticket) = ticket.write() {
                            ticket.status = status;
                        }
                    }
                };

                //match version
                // match store.get_mut(patch.id) {
                //     Some(ticket) => {
                //         match patch.title {
                //             Some(title) => {
                //                 // ticket.write(). = title;
                //                 match ticket.write() {
                //                     Ok(mut temp_ticket) => {
                //                         temp_ticket.title = title;
                //                     }
                //                     Err(e) => {}
                //                 }
                //             }
                //             None => {}
                //         }
                //         match patch.description {
                //             Some(description) => {
                //                 // ticket.get_mut().unwrap().description = description;
                //                 match ticket.write() {
                //                     Ok(mut temp_ticket) => {
                //                         temp_ticket.description = description;
                //                     }
                //                     Err(e) => {}
                //                 }
                //             }
                //             None => {}
                //         }
                //         match patch.status {
                //             Some(status) => {
                //                 //ticket.get_mut().unwrap().status = status;
                //                 match ticket.write() {
                //                     Ok(mut temp_ticket) => {
                //                         temp_ticket.status = status;
                //                     }
                //                     Err(e) => {}
                //                 }
                //             }
                //             None => {}
                //         }
                //     }
                //     None => {}
                // }

                let _ = response_channel.send(());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
