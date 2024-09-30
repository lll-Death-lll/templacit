use std::{marker::PhantomData, path::PathBuf};

use crate::MinecraftItem;

#[derive(Debug)]
pub struct Item<'n, 's, 'i> {
    custom_name: &'n str,
    source: ViewSource<'s>,
    items: Vec<MinecraftItem<'i>>,
}

impl<'n, 's, 'i> Item<'n, 's, 'i> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ItemBuilder<'n, 's, 'i, NoName, NoSource, NoItems> {
        ItemBuilder {
            custom_name: None,
            source: None,
            items: None,
            marker_name: PhantomData,
            marker_source: PhantomData,
            marker_items: PhantomData,
        }
    }
    pub fn name(&self) -> &'n str {
        self.custom_name
    }
    pub fn source(&self) -> &'s ViewSource {
        &self.source
    }
    pub fn properties(&self) -> String {
        let mut contents = "type=item\n".to_string();

        let items = &self
            .items
            .iter()
            .map(|i| format!("minecraft:{}", i.id()))
            .collect::<Vec<String>>()
            .join(" ");
        contents += format!("items={}\n", items).as_str();

        let view = match &self.source {
            ViewSource::Texture(file) => {
                format!("texture={}\n", file.file_name().unwrap().to_str().unwrap())
            }
            ViewSource::Model(file) => {
                format!("model={}\n", file.file_name().unwrap().to_str().unwrap())
            }
        };
        contents += view.as_str();

        contents += format!("components.minecraft\\:custom_name={}", self.custom_name).as_str();

        contents
    }
}

#[derive(Default, Clone)]
pub struct Name;
#[derive(Default, Clone)]
pub struct NoName;
#[derive(Default, Clone)]
pub struct Source;
#[derive(Default, Clone)]
pub struct NoSource;
#[derive(Default, Clone)]
pub struct Items;
#[derive(Default, Clone)]
pub struct NoItems;

pub struct ItemBuilder<'n, 's, 'i, N, S, I> {
    custom_name: Option<&'n str>,
    source: Option<ViewSource<'s>>,
    items: Option<Vec<MinecraftItem<'i>>>,
    marker_name: PhantomData<N>,
    marker_source: PhantomData<S>,
    marker_items: PhantomData<I>,
}

impl<'n, 's, 'i> ItemBuilder<'n, 's, 'i, Name, Source, Items> {
    pub fn build(self) -> Item<'n, 's, 'i> {
        Item {
            custom_name: self.custom_name.unwrap(),
            source: self.source.unwrap(),
            items: self.items.unwrap(),
        }
    }
}

impl<'n, 's, 'i, I> ItemBuilder<'n, 's, 'i, NoName, Source, I> {
    pub fn name_from_source(self) -> ItemBuilder<'n, 's, 'i, Name, Source, I>
    where
        's: 'n,
    {
        let source = self.source.unwrap();
        let name = match source {
            ViewSource::Texture(file) => file.file_stem().unwrap().to_str().unwrap(),
            ViewSource::Model(file) => file.file_stem().unwrap().to_str().unwrap(),
        };

        ItemBuilder {
            custom_name: Some(name),
            source: Some(source),
            items: self.items,
            marker_name: PhantomData,
            marker_source: PhantomData,
            marker_items: PhantomData,
        }
    }
}

impl<'n, 's, 'i, N, S, I> ItemBuilder<'n, 's, 'i, N, S, I> {
    pub fn name(self, name: &'n str) -> ItemBuilder<'n, 's, 'i, Name, S, I> {
        ItemBuilder {
            custom_name: Some(name),
            source: self.source,
            items: self.items,
            marker_name: PhantomData,
            marker_source: PhantomData,
            marker_items: PhantomData,
        }
    }
    pub fn source(self, source: ViewSource<'s>) -> ItemBuilder<'n, 's, 'i, N, Source, I> {
        ItemBuilder {
            custom_name: self.custom_name,
            source: Some(source),
            items: self.items,
            marker_name: PhantomData,
            marker_source: PhantomData,
            marker_items: PhantomData,
        }
    }
    pub fn items(
        self,
        items: Vec<MinecraftItem<'i>>,
    ) -> Result<ItemBuilder<'n, 's, 'i, N, S, Items>, String> {
        if items.is_empty() {
            return Err("No items were provided".to_owned());
        }

        Ok(ItemBuilder {
            custom_name: self.custom_name,
            source: self.source,
            items: Some(items),
            marker_name: PhantomData,
            marker_source: PhantomData,
            marker_items: PhantomData,
        })
    }
}

#[derive(Debug)]
pub enum ViewSource<'a> {
    Texture(&'a PathBuf),
    Model(&'a PathBuf),
}

impl<'a> ViewSource<'a> {
    pub fn new(file: &'a PathBuf) -> Result<ViewSource<'a>, String> {
        if let Some(extension) = file.extension() {
            return match extension.to_str() {
                Some("png") => Ok(ViewSource::Texture(file)),
                Some("bbmodel") => Ok(ViewSource::Model(file)),
                Some(e) => Err(format!("{} extension is not texture or model", e)),
                None => Err("File extension not found".to_owned()),
            };
        } else {
            Err("File extension is incorrect".to_owned())
        }
    }
    pub fn path(&self) -> &PathBuf {
        match self {
            ViewSource::Texture(t) => t,
            ViewSource::Model(m) => m,
        }
    }
}
