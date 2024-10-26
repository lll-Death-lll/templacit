use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref MINECRAFT_ITEMS: HashSet<&'static str> = {
        let items_str = include_str!("./minecraft_items.txt");
        let mut s = HashSet::new();
        items_str.lines().for_each(|item| {
            s.insert(item);
        });
        s
    };
}

#[derive(Debug)]
pub struct MinecraftItem<'i> {
    id: &'i str,
}

impl<'i> MinecraftItem<'i> {
    pub fn new(id: &'i str) -> Result<MinecraftItem<'i>, String> {
        if !MINECRAFT_ITEMS.contains(id) {
            return Err("No item with such id".to_owned());
        }

        Ok(MinecraftItem { id })
    }
    pub fn id(&self) -> &'i str {
        self.id
    }
}
