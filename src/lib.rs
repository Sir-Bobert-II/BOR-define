use log::info;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};
use structstruck::strike;

type Words = Vec<Word>;

strike! {
    #[strikethrough[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]]
    #[strikethrough[serde(rename_all = "camelCase")]]
    struct Word {
        word: String,
        phonetics: Vec<pub struct Phonetic {
             audio: String,
            source_url: Option<String>,
             license: Option<pub struct License {
                name: String,
                url: String,
            }>,
            text: Option<String>,
        }>,
        meanings: Vec<pub struct Meaning {
            part_of_speech: String,
            definitions: Vec<pub struct Definition {
                definition: String,
                synonyms: Vec<Value>,
                antonyms: Vec<Value>,
                example: Option<String>,
            }
            >,
            synonyms: Vec<String>,
            antonyms: Vec<String>,
        }>,
        license: License,
        source_urls: Vec<String>,
    }
}


pub async fn run(word: &str) -> String
{
    // Replace spaces with %20 for the url
    let word_ = word.to_lowercase().trim().replace(' ', "%20").to_string();

    let request_url =
    format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        word_
    );

    // Make the API call, parse the json to a `Page`.
    if let Ok(words) = {
        match reqwest::get(&request_url).await {
            Ok(x) => {
                info!("Requested '{}'", request_url);
                x
            }
            Err(e) => return format!("RequestError: Internal request error: {e}"),
        }
        .json::<Words>()
        .await
 
    }
    {
        let mut buf = String::new();
        for meaning in &words[0].meanings
        {
            buf.push_str(&format!("({}) {}\n",meaning.part_of_speech, meaning.definitions[0].definition));
            if let Some(example) = &meaning.definitions[0].example
            {
                buf.push_str(&format!("    Example: '{}'\n", example))
            }
        }
        format!("Definitions for {word_}:\n{}", buf)
    }
    else {
        format!("Couldn't define '{word}'")
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("define")
        .description("Define an english word")
        .dm_permission(true)
        .create_option(|option| {
            option
                .name("word")
                .description("The word to define")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
