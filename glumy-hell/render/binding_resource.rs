use crate::{Buffer, TextureView};

pub enum BindingResource<'r> {
    Buffer(&'r Buffer),
    TextureView(&'r TextureView),
}
