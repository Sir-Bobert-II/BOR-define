use log::info;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;


type Words = Vec<Word>;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    name: String,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Phonetic {
    audio: String,
    source_url: Option<String>,
    license: Option<License>,
    text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meaning {
    part_of_speech: String,
    definitions: Vec<Definition>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    definition: String,
    synonyms: Vec<Value>,
    antonyms: Vec<Value>,
    example: Option<String>,
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Word {
    word: String,
    phonetics: Vec<Phonetic>,
    meanings: Vec<Meaning>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
    license: License,
    source_urls: Vec<String>,
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
