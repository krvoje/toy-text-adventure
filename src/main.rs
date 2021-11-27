use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

enum Action {
    Help,
    Quit,
    Describe(String),
}

impl Action {
    fn parse(input: &str) -> Vec<Action> {
        input.split_whitespace().map(|command| {
            match command.to_lowercase().as_str() {
                "quit" => Action::Quit,
                "help" => Action::Help,
                _      => Action::Describe(command.to_string()),
            }
        }).collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Scene {
    name: String,
    description: String,
    items: HashMap<String, Item>,
    is_first_scene: bool,
}

impl Scene {
    pub fn describe(&self, item_name: String) -> &str {
        let term = item_name.to_lowercase();
        if term == self.name.to_lowercase() {
            self.description.as_str()
        } else {
            self.items.iter()
                .find(|(name, item)| term == name.to_lowercase())
                .map(|(name, item)|item.description.as_str())
                .unwrap_or("")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Story {
    scenes: HashMap<String, Scene>,
}

fn prompt(prompt_text: &String) -> String {
    print!("{}>", prompt_text);
    io::stdout().flush();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn parse_story() -> Story {
    let file = File::open("res/Story.yaml").unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let story: Story = serde_yaml::from_str(&contents).unwrap();
    story
}

fn main() {
    let story = parse_story();
    let mut is_finished = false;
    let mut current_scene = story.scenes.iter().find(|(_, scene)| {scene.is_first_scene}).unwrap().1;
    while !is_finished {
        let input: String = prompt(&current_scene.name);
        let output = Action::parse(input.as_str()).into_iter().map(|action| {
            match action {
                Action::Help                  => "Try typing any of the highlighted words",
                Action::Quit                  => {is_finished = true; "Goodbye!"},
                Action::Describe(item) => current_scene.describe(item),
            }
        }).into_iter().collect::<Vec<&str>>()
        .join("\n");

        println!("{}", output);
    }
}
