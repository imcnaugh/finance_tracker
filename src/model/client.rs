use crate::model::new_client::NewClient;
use crate::utils::generate_new_id;

pub(crate) struct Client {
    id: String,
    name: String,
}

impl From<NewClient> for Client {
    fn from(value: NewClient) -> Self {
        Self {
            id: generate_new_id(),
            name: value.get_name().into(),
        }
    }
}

impl Client {
    pub(crate) fn new(id: String, name: String) -> Self {
        Self { id, name }
    }

    pub(crate) fn get_id(&self) -> &str {
        self.id.as_str()
    }

    pub(crate) fn get_name(&self) -> &str {
        self.name.as_str()
    }
}
