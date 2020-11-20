mod filters;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use templ_runtime::{Runtime, RuntimeBuilder};
use templ_vm::{compile_path, Error, Template as TemplateProgram};

#[derive(Clone)]
pub struct Template(Arc<TemplateProgram>);

impl std::ops::Deref for Template {
    type Target = TemplateProgram;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct TemplBuilder {
    runtime: RuntimeBuilder,
}

impl TemplBuilder {
    pub fn new() -> TemplBuilder {
        TemplBuilder {
            runtime: Runtime::new(),
        }
    }

    pub fn load_stdlib(self) -> Self {
        TemplBuilder {
            runtime: filters::std_filters(self.runtime),
        }
    }

    pub fn build(self) -> Templ {
        Templ {
            templates: Vec::default(),
            runtime: self.runtime.build(),
        }
    }
}

struct Entry {
    template: Template,
    path: PathBuf,
}

pub struct Templ {
    templates: Vec<Entry>,
    runtime: Runtime,
}

impl Templ {
    #[cfg(feature = "glob")]
    pub fn load_all_from_path(&mut self, pattern: impl AsRef<str>) -> Result<(), Error> {
        for path in glob::glob(pattern.as_ref()).unwrap() {
            self.load_path(path.unwrap())?;
        }
        Ok(())
    }

    pub fn load_path(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path = std::fs::canonicalize(path).expect("");

        if self
            .templates
            .iter()
            .find(|entry| entry.path == path)
            .is_some()
        {
            return Ok(());
        }

        let templates = compile_path(&self.runtime, &path)?;
        for templ in templates.into_iter() {
            if self
                .templates
                .iter()
                .find(|entry| entry.path == path)
                .is_some()
            {
                panic!("template already found");
            }

            self.templates.push(Entry {
                template: Template(Arc::new(templ)),
                path: path.clone(),
            });
        }
        Ok(())
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<&Template> {
        self.templates
            .iter()
            .find(|m| m.template.name() == name.as_ref())
            .map(|m| &m.template)
    }
}
