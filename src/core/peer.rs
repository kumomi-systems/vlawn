use derivative::Derivative;
use uuid::Uuid;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Peer {
    id: Uuid,

    #[derivative(PartialEq = "ignore")]
    name: String,
}

impl Peer {
    pub fn new(name: &str) -> Self {
        Peer {
            id: Uuid::new_v4(),
            name: name.to_string(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
