use crate::error::TemplatorError;

pub type TemplatorResult<T> = Result<T, TemplatorError>;

pub trait TemplateSource {
    fn get_choices(&self) -> TemplatorResult<Vec<String>>;
    fn load_choice(&self, choice: String) -> TemplatorResult<()>;
}
