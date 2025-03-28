use std::collections::BTreeMap;

struct Ticket {
    title: Title,
    description: Description,
    status: Status,
}

pub struct Title(String);

pub struct Description(String);

pub enum Status {
    Pending,
    DoIno,
    Progressing,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TicketId(i32);

struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    count: i32,
}

impl TicketStore {
    fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            count: 0,
        }
    }

    fn add_tickets(&mut self, ticket: Ticket) {
        self.count += 1;
        self.tickets.insert(TicketId(self.count), ticket);
    }

    fn get(&self, ticket_id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&ticket_id)
    }
}
