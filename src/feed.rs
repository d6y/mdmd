use rss::{extension::Extension, Channel, Guid, Item};

pub trait ChannelSurf {
    /// Search the channel for the first GUID lexically greater than the given GUID,
    /// which we refer to as "the next GUID" or "next post".
    fn find_next_guid(&self, guid: &Guid) -> Option<&Guid>;

    /// Search the channel for all GUIDs lexically greater than the given GUID,
    /// which we refer to as "the next GUIDs" or "next posts".
    fn find_next_guids(&self, guid: &Guid) -> Vec<&Guid>;

    fn find_by_guid(&self, guid: &Guid) -> Option<&Item>;
}

pub trait ItemSurf {
    fn medias(&self) -> Vec<&Extension>;
}

impl ItemSurf for Item {
    fn medias(&self) -> Vec<&Extension> {
        let mut medias = Vec::new();
        for (ext_type, ext_map) in self.extensions.iter() {
            if ext_type == "media" {
                let xs: &Vec<Extension> = ext_map.get("content").unwrap();
                for media in xs.iter() {
                    medias.push(media);
                }
            }
        }
        medias
    }
}

impl ChannelSurf for Channel {
    fn find_next_guid(&self, from: &Guid) -> Option<&Guid> {
        self.find_next_guids(from).into_iter().next()
    }

    fn find_next_guids(&self, from: &Guid) -> Vec<&Guid> {
        let mut candidates: Vec<&Guid> = self
            .items()
            .iter()
            .flat_map(|item| item.guid())
            .filter(|&g| g.value > from.value)
            .collect();

        candidates.sort_by_key(|g| &g.value);
        candidates
    }

    fn find_by_guid(&self, guid: &Guid) -> Option<&Item> {
        self.items()
            .iter()
            .find(|&item| item.guid().map(|g| g.value()) == Some(guid.value()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    const RSS_STR: &str = include_str!("../rss/example01.rss");

    #[test]
    fn test_find_next_from_older() {
        let channel = Channel::from_str(RSS_STR).unwrap();

        // We know there are two GUIDs after this one in the example01 RSS feed
        let from: Guid = Guid {
            value: "https://mastodon.green/@d6y/109818375938647316".to_string(),
            permalink: true,
        };

        let expected = Guid {
            value: "https://mastodon.green/@d6y/109818388501614164".to_string(),
            permalink: true,
        };

        assert_eq!(Some(&expected), channel.find_next_guid(&from));
    }

    #[test]
    fn test_find_next_from_latest() {
        let channel = Channel::from_str(RSS_STR).unwrap();
        // This is the latest GUID: there is no next GUID
        let from: Guid = Guid {
            value: "https://mastodon.green/@d6y/109848274586190243".to_string(),
            permalink: true,
        };

        assert_eq!(None, channel.find_next_guid(&from));
    }
}
