use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
struct Segments {
    segment: Vec<Segment>,
}

#[derive(Deserialize, PartialEq, Debug)]
struct Segment {
    #[serde(rename = "@bytes")]
    bytes: u64,
    #[serde(rename = "@number")]
    number: u64,
    #[serde(rename = "$value")]
    message_id: String,
}

#[derive(Deserialize, PartialEq, Debug)]
struct Group(#[serde(rename = "$value")] String);

#[derive(Deserialize, PartialEq, Debug)]
struct Groups {
    group: Vec<Group>,
}

#[derive(Deserialize, PartialEq, Debug)]
struct File {
    #[serde(rename = "@poster")]
    poster: String,
    #[serde(rename = "@date")]
    date: u64,
    #[serde(rename = "@subject")]
    subject: String,
    groups: Groups,
    segments: Segments,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
enum MetaType {
    Title,
    Tag,
    Category,
    Password,
}

#[derive(Deserialize, PartialEq, Debug)]
struct Meta {
    #[serde(rename = "@type")]
    mtype: MetaType,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Deserialize, PartialEq, Debug)]
struct Head {
    meta: Vec<Meta>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct NZBData {
    head: Option<Head>,
    file: File,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn basic_deserialization() {
        let data =
            fs::read_to_string("fixtures/base.nzb").expect("could not read base fixture file");
        let parsed = quick_xml::de::from_str::<NZBData>(&data).unwrap();
        let expected = NZBData {
            head: Some(Head {
                meta: vec![
                    Meta {
                        mtype: MetaType::Title,
                        value: "Your File!".to_string(),
                    },
                    Meta {
                        mtype: MetaType::Password,
                        value: "secret".to_string(),
                    },
                    Meta {
                        mtype: MetaType::Tag,
                        value: "HD".to_string(),
                    },
                    Meta {
                        mtype: MetaType::Category,
                        value: "TV".to_string(),
                    },
                ],
            }),
            file: File {
                poster: "Joe Bloggs <bloggs@nowhere.example>".to_string(),
                date: 1071674882,
                subject: "Here's your file!  abc-mr2a.r01 (1/2)".to_string(),
                groups: Groups {
                    group: vec![
                        Group("alt.binaries.newzbin".to_string()),
                        Group("alt.binaries.mojo".to_string()),
                    ],
                },
                segments: Segments {
                    segment: vec![
                        Segment {
                            bytes: 102394,
                            number: 1,
                            message_id: "123456789abcdef@news.newzbin.com".to_string(),
                        },
                        Segment {
                            bytes: 4501,
                            number: 2,
                            message_id: "987654321fedbca@news.newzbin.com".to_string(),
                        },
                    ],
                },
            },
        };

        assert_eq!(parsed, expected);
    }
}
