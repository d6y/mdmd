use rss::{extension::Extension, Channel};
use std::str::FromStr;

mod feed;
mod markdown;

fn main() {
    println!("Hello, world!");

    let rss_str = include_str!("../rss/example01.rss");
    let channel = Channel::from_str(rss_str).unwrap();
    // println!("{:?}", channel);
    for item in channel.items() {
        println!("{:?}", item.guid);
        println!("{:?}", item.link);
        println!("{:?}", item.description);
        println!("{:?}", item.pub_date);
        //println!("{:?}", item.extensions);
        for (ext_type, ext_map) in item.extensions.iter() {
            println!("{ext_type:?}");
            // vec<Extension>
            let xs: &Vec<Extension> = ext_map.get("content").unwrap();
            for x in xs {
                dbg!(x.attrs.get("type"));
                dbg!(x.attrs.get("url"));
                dbg!(x
                    .children
                    .get("description")
                    .and_then(|d| d[0].value.to_owned()));
            }
            /*{"media": {"content": [Extension { name: "media:content", value: None, attrs: {"fileSize": "305683", "medium": "image", "type": "image/jpeg", "url": "https://files.mastodon.green/media_attachments/files/109/681/669/125/176/877/original/83a92e31941b9c90.jpeg"}, children: {"description": [Extension { name: "media:description", value: Some("A view toward  Brighton West Pier remains. A blue sky, a calm sea, but there’s a red danger sign on the beach still, asking people to keep away from the waves."), attrs: {"type": "plain"}, children: {} }], "rating": [Extension { name: "media:rating", value: Some("nonadult"), attrs: {"scheme": "urn:simple"}, children: {} }]} }]}} */
        }
    }
}
