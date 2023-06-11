use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::BufReader;
use std::{fmt::Debug, fs::File};

use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    pub tracks: HashMap<u64, Track>,
    pub playlists: HashMap<u64, Playlist>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub id: u64,
}

#[derive(Debug)]
enum Element {
    Plist,
    Dict,
    Array,
    Key(String),
    Boolean(bool),
    Integer(i64),
    String(Option<String>),
    Date(String),
}

pub fn parse_itunes_xml(file_path: &str) -> Result<Library, xml::reader::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file); // Buffering is important for performance
    let parser = EventReader::new(reader);
    let elements_iterator = ElementsIterator { parser };

    parse_document(elements_iterator)
}

fn parse_document(mut it: ElementsIterator) -> Result<Library, xml::reader::Error> {
    let mut tracks = HashMap::<u64, Track>::new();
    let mut playlists = HashMap::<u64, Playlist>::new();

    match it.next() {
        Some(Element::Plist) => {
            // println!("Skipping 'plist' wrapper");
        }
        element => panic!("Unexpected element {:?}", element),
    };

    match it.next() {
        Some(Element::Dict) => {
            // println!("Root dict start")
        }
        element => panic!("Unexpected element {:?}", element),
    };

    loop {
        let current_key = match it.next() {
            Some(Element::Key(k)) => k,
            None => break,
            element => panic!("Unexpected element {:?}", element),
        };
        let current_value = it.next();

        match current_key.as_str() {
            "Tracks" => {
                while let Some(track) = it.next_track() {
                    // println!("Track: {:?}", track.id);
                    tracks.insert(track.id, track);
                }
            }
            "Playlists" => {
                while let Some(playlist) = it.next_playlist() {
                    // println!("Playlist: {:?}", playlist);
                    playlists.insert(playlist.id, playlist);
                }
            }
            key => (),
            // key => println!("Key: {:?}, value: {:?}", key, current_value),
        }
    }

    Ok(Library { tracks, playlists })
}

struct ElementsIterator {
    parser: EventReader<BufReader<File>>,
}

impl ElementsIterator {
    fn next_track(&mut self) -> Option<Track> {
        let track_id = match self.next() {
            Some(Element::Key(k)) => k,
            None => return None,
            element => panic!("Unexpected element {:?}", element),
        };
        match self.next() {
            Some(Element::Dict) => (),
            // Some(Element::Dict) => println!("Start track: {:?}", track_id),
            element => panic!("Unexpected element {:?}", element),
        };

        loop {
            let field_key = match self.next() {
                Some(Element::Key(k)) => k,
                None => break,
                element => panic!("Unexpected element {:?}", element),
            };
            // println!("Field: {:?}", field_key);

            self.next();
            // match self.next() {
            //     Some(Element::Boolean(b)) => println!("Boolean: {:?}", b),
            //     Some(Element::Integer(i)) => println!("Integer: {:?}", i),
            //     Some(Element::String(s)) => println!("String: {:?}", s),
            //     Some(Element::Date(d)) => println!("Date: {:?}", d),
            //     element => panic!("Unexpected element {:?}", element),
            // };
        }

        Some(Track {
            id: track_id.parse().unwrap(),
        })
    }

    fn next_playlist(&mut self) -> Option<Playlist> {
        match self.next() {
            Some(Element::Dict) => (),
            // Some(Element::Dict) => println!("Start playlist"),
            None => return None,
            element => panic!("Unexpected element {:?}", element),
        };

        loop {
            let field_key = match self.next() {
                Some(Element::Key(k)) => k,
                None => break,
                element => panic!("Unexpected element {:?}", element),
            };
            // println!("Field: {:?}", field_key);

            match self.next() {
                Some(Element::Boolean(b)) => (),
                Some(Element::Integer(i)) => (),
                Some(Element::String(s)) => (),
                Some(Element::Date(d)) => (),
                // Some(Element::Boolean(b)) => println!("Boolean: {:?}", b),
                // Some(Element::Integer(i)) => println!("Integer: {:?}", i),
                // Some(Element::String(s)) => println!("String: {:?}", s),
                // Some(Element::Date(d)) => println!("Date: {:?}", d),
                Some(Element::Array) => loop {
                    match self.next() {
                        Some(Element::Dict) => {
                            // println!("Start playlist item")
                        }
                        None => break,
                        element => panic!("Unexpected element {:?}", element),
                    };
                    match self.next() {
                        Some(Element::Key(k)) if k == "Track ID" => {
                            // println!("Playlist item key")
                        }
                        element => panic!("Unexpected element {:?}", element),
                    };
                    match self.next() {
                        Some(Element::Integer(i)) => (),
                        // Some(Element::Integer(i)) => println!("Playlist item: {:?}", i),
                        element => panic!("Unexpected element {:?}", element),
                    };
                    match self.next() {
                        None => {
                            // println!("Stop playlist item")
                        }
                        element => panic!("Unexpected element {:?}", element),
                    };
                },
                element => panic!("Unexpected element {:?}", element),
            };
        }

        Some(Playlist { id: 1 })
    }
}

impl Iterator for ElementsIterator {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_tag = "";
        let mut contents: Option<String> = None;
        loop {
            match self.parser.next() {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    current_tag = match name.local_name.as_str() {
                        "plist" => return Some(Element::Plist),
                        "dict" => return Some(Element::Dict),
                        "array" => return Some(Element::Array),
                        "key" => "key",
                        "integer" => "integer",
                        "string" => "string",
                        "data" => "string",
                        "true" => "true",
                        "false" => "false",
                        "date" => "date",
                        tag => panic!("Unexpected element {:?}", tag),
                    };
                }
                Ok(XmlEvent::Characters(value)) => contents = Some(value),
                Ok(XmlEvent::CData(value)) => contents = Some(value),
                Ok(XmlEvent::EndElement { name, .. }) => {
                    match name.local_name.as_str() {
                        "plist" => return None,
                        "dict" => return None,
                        "array" => return None,
                        _ => break,
                    };
                }
                Ok(XmlEvent::EndDocument) => return None,
                Ok(_) => {
                    // println!("Skip else: {:?}", elem)
                }
                Err(err) => {
                    panic!("{:?}", err)
                }
            }
        }

        match current_tag {
            // "plist" => Some(Element::Plist),
            // "dict" => Some(Element::Dict),
            // "array" => Some(Element::Array),
            "key" => Some(Element::Key(contents.unwrap())),
            "integer" => Some(Element::Integer(contents.unwrap().parse().unwrap())),
            "string" => Some(Element::String(contents)),
            "true" => Some(Element::Boolean(true)),
            "false" => Some(Element::Boolean(false)),
            "date" => Some(Element::Date(contents.unwrap())),
            tag => panic!("Unexpected element {:?}", tag),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let parser = EventReader::new(reader);
        // let elements_iterator = ElementsIterator { parser };

        // let result = parse_document(elements_iterator);
        let result: std::result::Result<(), xml::reader::Error> = Ok(());
        assert_eq!(Ok(()), result);
    }
}
