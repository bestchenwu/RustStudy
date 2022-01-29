pub struct ClassName {
    field: String,
}

impl ClassName {
    pub fn new(newField: String) -> ClassName {
        ClassName {
            field: newField
        }
    }

    pub fn public_method(&self) -> &String {
        println!("from public_method");
        self.private_method()
    }

    fn private_method(&self) -> &String {
        println!("from private_method");
        &self.field
    }
}