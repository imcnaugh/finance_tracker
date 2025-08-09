pub(crate) struct Client {
    id: String,
    name: String,
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
