use crate::{BufferError, BufferIsBuild, BufferUsages};

#[derive(Default, Debug, Clone)]
pub struct BufferBuilder {
    label: Option<String>,
    usages: Option<BufferUsages>,
    content: Option<Vec<u8>>,
}

impl BufferBuilder {
    pub fn label(&mut self, label: String) -> &mut Self {
        self.label = Some(label);
        self
    }

    pub fn usages(&mut self, usages: BufferUsages) -> &mut Self {
        self.usages = Some(usages);
        self
    }

    pub fn content(&mut self, data: Vec<u8>) -> &mut Self {
        self.content = Some(data);
        self
    }

    pub fn try_build(self) -> Result<BufferIsBuild, BufferError> {
        Ok(BufferIsBuild {
            label: self.label.ok_or(BufferError::LabelBuildError)?,
            usages: self.usages.ok_or(BufferError::UsagesBuildError)?,
            content: self.content.ok_or(BufferError::DataBuildError)?,
        })
    }
}
