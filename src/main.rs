use std::{env, fs, io::Write, path::PathBuf};
mod files;
mod item;
mod minecraft_item;

use clap::{arg, command, value_parser, ArgAction};
use files::*;
use item::*;
use minecraft_item::*;

fn main() {
    let matches = command!()
        .arg(
            arg!(-n --name <item_name> "Name of the new item")
                .required(false)
                .value_parser(value_parser!(String))
        )
        .arg(
            arg!(-t --texture <texture_file> "Texture of the new item")
                .required(true)
                .value_parser(value_parser!(PathBuf))
                .conflicts_with("model")
        )
        .arg(
            arg!(-m --model <model_file> "Model of the new item")
                .required(true)
                .value_parser(value_parser!(PathBuf))
                .conflicts_with("texture")
        )
        .arg(
            arg!(-i --items <items_list> "Minecraft items id's you want to replace, separated by comma ','")
                .required(true)
                .value_parser(value_parser!(String))
                .action(ArgAction::Append)
                .value_delimiter(',')
        )
        .get_matches();

    let item = Item::new();

    let view_source = if let Some(texture) = matches.get_one::<PathBuf>("texture") {
        ViewSource::new(texture)
    } else if let Some(model) = matches.get_one::<PathBuf>("model") {
        ViewSource::new(model)
    } else {
        Err("How did we get here? No texture or model".to_owned())
    };
    if let Err(e) = view_source {
        println!("Error while looking for a file: {e}");
        return;
    }
    let item = item.source(view_source.expect("View source not found"));

    let item = match matches.get_one::<String>("name") {
        Some(name) => item.name(name),
        None => item.name_from_source(),
    };

    let mut minecraft_items = vec![];
    if let Some(items) = matches.get_many::<String>("items") {
        for item in items {
            match MinecraftItem::new(item.as_str()) {
                Ok(i) => minecraft_items.push(i),
                Err(e) => println!("Error while adding item: {}", e),
            }
        }
    }
    let item = item.items(minecraft_items);
    if let Err(e) = item {
        println!("Error looking for items: {}", e);
        return;
    }
    let item = item.unwrap();

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
