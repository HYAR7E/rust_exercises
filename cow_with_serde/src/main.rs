use std::borrow::Cow;

use serde::{Deserialize, Serialize};

fn fetch_data() -> String {
    String::from(
        r#"
        {
            "id": 1,
            "title": "Hello, Rust",
            "borrowed": "Borrowed bc it does not contain quotes",
            "owned": "Owned bc this contains \"quotes\""
        }
        "#,
    )
}

#[derive(Debug, Serialize, Deserialize)]
struct BlogPost<'a> {
    id: u32,

    title: &'a str,

    #[serde(borrow)]
    borrowed: Cow<'a, str>,

    owned: Cow<'a, str>,
}

fn main() -> Result<(), serde_json::Error> {
    let data = fetch_data();
    let post: BlogPost = match serde_json::from_str(&data) {
        Ok(val) => val,
        Err(e) => panic!("Error when deserializing json: {e}"),
    };
    println!("deserialized = {:?}", post);
    // drop(data);
    assert_eq!(matches!(post.borrowed, Cow::Borrowed(_)), true);
    assert_eq!(matches!(post.owned, Cow::Borrowed(_)), false);

    let post_json: String = serde_json::to_string(&post)?;
    println!("serialized = {:?}", post_json);

    Ok(())
}
