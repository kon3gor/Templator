pub trait TemplateSource {
    fn get_choices(&self) -> Vec<String>;
    fn load_choice(&self, choice: String) -> bool;
}
