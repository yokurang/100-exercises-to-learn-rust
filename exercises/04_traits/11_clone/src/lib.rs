// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.
pub trait Clone {
    fn clone(&self) -> Self;
}

pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Clone for Ticket {
    fn clone(&self) -> Self {
        Ticket {
            title: self.title.clone(),
            description: self.description.clone(),
            status: self.status.clone(),
        }
    }
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}

impl Clone for Summary {
    fn clone(&self) -> Self {
        Summary {
            title: self.title.clone(),
            status: self.status.clone(),
        }
    }
}

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    let summary = ticket.clone().summary();
    (ticket, summary)
}
