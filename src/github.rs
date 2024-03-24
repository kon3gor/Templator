use tokio::runtime::{Builder, Runtime};

use crate::api::{TemplateSource, TemplatorResult};

const DIR: &'static str = "dir";

pub struct GithubSource {
    user: String,
    repo: String,
    rt: Runtime,
}

impl GithubSource {
    pub fn new(uri: String) -> Self {
        let (user, repo) = uri.split_once('/').unzip();
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        Self {
            rt: rt,
            user: user.unwrap().to_string(),
            repo: repo.unwrap().to_string(),
        }
    }

    async fn load_contents(&self) -> TemplatorResult<Vec<String>> {
        let content = octocrab::instance()
            .repos(&self.user, &self.repo)
            .get_content()
            .send()
            .await?;

        let dirs: Vec<String> = content
            .items
            .iter()
            .filter(|x| x.r#type == DIR)
            .map(|x| x.name.clone())
            .collect();
        Ok(dirs)
    }

    // maybe create a snapshot in /tmp directory with all the contents of the repo?
    // then here I can just copy specified directory from /tmp/{repo} and live a long happy life
    async fn load_directory_from_git(&self, directory: String) -> TemplatorResult<()> {
        todo!("Not yet implemented")
    }
}

impl TemplateSource for GithubSource {
    fn get_choices(&self) -> crate::api::TemplatorResult<Vec<String>> {
        let result = self.rt.block_on(self.load_contents())?;
        return Ok(result);
    }

    fn load_choice(&self, choice: String, name: Option<String>) -> crate::api::TemplatorResult<()> {
        Ok(self.rt.block_on(self.load_directory_from_git(choice))?)
    }
}
