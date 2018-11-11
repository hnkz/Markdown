use super::mdobject::MDObject;

#[derive(Debug)]
pub struct CodeBlock {
    val: String,
}

impl CodeBlock {
    pub fn new(val: String) -> CodeBlock {
        CodeBlock {
            val: val,
        }
    }
}

impl MDObject for CodeBlock {
    fn output(&self) {
        println!("{:?}", self);
    }
}