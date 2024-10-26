use std::{fs, io::Write, path::PathBuf};
mod files;
mod item;
mod minecraft_item;

use clap::arg;
use clap::Parser;
use clap::Subcommand;
use files::*;
use item::*;
use minecraft_item::*;

#[derive(Parser, Debug)]
#[command(
    version = "0.0.1",
    about = "Minecraft CIT resourcepack creation",
    long_about = "A tool for easier creation of minecraft resource packs with CIT"
)]
struct Args {
    /// Name of the new item
    #[arg(short, long)]
    name: String,

    #[command(subcommand)]
    source: ItemSource,

    /// Minecraft items id's you want to replace, separated by comma ','
    #[arg(short, long, value_delimiter = ',')]
    items: Vec<String>,
}

#[derive(Subcommand, Debug)]
enum ItemSource {
    Texture {
        /// Texture of the new item
        #[arg(short, long, value_name = "FILE")]
        texture: PathBuf,
    },
    Model {
        /// Model of the new item
        #[arg(short, long, value_name = "FILE")]
        model: PathBuf,
    },
}

impl<'a> ViewSource<'a> {
    fn from_item_source(item_source: &'a ItemSource) -> Result<ViewSource<'a>, String> {
        match item_source {
            ItemSource::Texture { texture } => ViewSource::new(texture), // Assumes ViewSource::new takes a reference
            ItemSource::Model { model } => ViewSource::new(model), // Assumes ViewSource::new takes a reference
        }
    }
}

fn main() {
    let args = Args::parse();

    let item = Item::new();

    let view_source = match ViewSource::from_item_source(&args.source) {
        Ok(view_source) => view_source,
        Err(e) => {
            println!("View source not found: {e}");
            return;
        }
    };

    let Ok(item) = item.source(view_source).name(&args.name).items(
        args.items
            .iter()
            .filter_map(|item_name| match MinecraftItem::new(item_name) {
                Ok(item) => return Some(item),
                Err(e) => {
                    println!("Error looking for items: {}", e);
                    return None;
                }
            })
            .collect(),
    ) else {
        println!("Error while building item");
        return;
    };

    let item = item.build();

    let cit_dir = get_cit_dir().expect("Cit dir couldn't be found/created");
    let item_dir = create_subdir(item.name(), &cit_dir).expect("Couldn't create file directory");

    let properties_file = item_dir.join(format!("{}.properties", &item.name()));
    let mut file = fs::File::create(&properties_file).expect("Failed to create file");
    let _ = file
        .write(item.properties().as_bytes())
        .expect("Failed to write to file");

    move_file(item.source().path(), &item_dir).expect("Failed to create file");
}
