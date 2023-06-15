use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::BufReader;
use std::{fmt::Debug, fs::File};

use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Library {
    pub tracks: HashMap<u64, Track>,
    pub playlists: HashMap<u64, Playlist>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Track {
    pub id: u64,
    pub name: Option<String>,              // `bson:"Name,omitempty"`
    pub artist: Option<String>,            // `bson:"Artist,omitempty"`
    pub album_artist: Option<String>,       // `bson:"AlbumArtist,omitempty"`
    pub composer: Option<String>,          // `bson:"Composer,omitempty"`
    pub genre: Option<String>,             // `bson:"Genre,omitempty"`
    pub album: Option<String>,             // `bson:"Album,omitempty"`
    pub kind: Option<String>,              // `bson:"Kind,omitempty"`
    pub loved: Option<bool>,               // `bson:"Loved,omitempty"`
    pub disliked: Option<bool>,            // `bson:"Disliked,omitempty"`
    pub matched: Option<bool>,             // `bson:"Matched,omitempty"`
    pub explicit: Option<bool>,            // `bson:"Explicit,omitempty"`
    pub compilation: Option<bool>,         // `bson:"Compilation,omitempty"`
    pub part_of_gapless_album: Option<bool>,  // `bson:"PartOfGaplessAlbum,omitempty"`
    pub movie: Option<bool>,               // `bson:"Movie,omitempty"`
    pub podcast: Option<bool>,             // `bson:"Podcast,omitempty"`
    pub unplayed: Option<bool>,            // `bson:"Unplayed,omitempty"`
    pub comments: Option<String>,          // `bson:"Comments,omitempty"`
    pub content_rating: Option<String>,     // `bson:"ContentRating,omitempty"`
    pub size: Option<i64>,                 // `bson:"Size,omitempty"`
    pub total_time: Option<i64>,            // `bson:"TotalTime,omitempty"`
    pub disc_number: Option<i64>,           // `bson:"DiscNumber,omitempty"`
    pub disc_count: Option<i64>,            // `bson:"DiscCount,omitempty"`
    pub track_number: Option<i64>,          // `bson:"TrackNumber,omitempty"`
    pub track_count: Option<i64>,           // `bson:"TrackCount,omitempty"`
    pub year: Option<i64>,                 // `bson:"Year,omitempty"`
    pub bpm: Option<i64>,                  // `bson:"BPM,omitempty"`
    pub date_modified: Option<String>,      // `bson:"DateModified,omitempty"`
    pub date_added: Option<String>,         // `bson:"DateAdded,omitempty"`
    pub bit_rate: Option<i64>,              // `bson:"BitRate,omitempty"`
    pub sample_rate: Option<i64>,           // `bson:"SampleRate,omitempty"`
    pub equalizer: Option<String>,           // `bson:"Equalizer,omitempty"`
    pub play_count: Option<i64>,            // `bson:"PlayCount,omitempty"`
    pub play_date: Option<i64>,             // `bson:"PlayDate,omitempty"`
    pub play_date_utc: Option<String>,       // `bson:"PlayDateUTC,omitempty"`
    pub skip_count: Option<i64>,            // `bson:"SkipCount,omitempty"`
    pub skip_date: Option<String>,          // `bson:"SkipDate,omitempty"`
    pub release_date: Option<String>,       // `bson:"ReleaseDate,omitempty"`
    pub normalization: Option<i64>,       // `bson:"Normalization,omitempty"`
    pub rating: Option<i64>,               // `bson:"Rating,omitempty"`
    pub rating_computed: Option<bool>,      // `bson:"RatingComputed,omitempty"`
    pub album_rating: Option<i64>,          // `bson:"AlbumRating,omitempty"`
    pub album_rating_computed: Option<bool>, // `bson:"AlbumRatingComputed,omitempty"`
    pub artwork_count: Option<i64>,         // `bson:"ArtworkCount,omitempty"`
    pub sort_name: Option<String>,          // `bson:"SortName,omitempty"`
    pub sort_album: Option<String>,         // `bson:"SortAlbum,omitempty"`
    pub sort_album_artist: Option<String>,   // `bson:"SortAlbumArtist,omitempty"`
    pub sort_composer: Option<String>,      // `bson:"SortComposer,omitempty"`
    pub sort_artist: Option<String>,        // `bson:"SortArtist,omitempty"`
    pub persistent_id: Option<String>,      // `bson:"PersistentID,omitempty"`
    pub track_type: Option<String>,         // `bson:"TrackType,omitempty"`
    pub purchased: Option<bool>,           // `bson:"Purchased,omitempty"`
    pub music_video: Option<bool>,          // `bson:"MusicVideo,omitempty"`
    pub has_video: Option<bool>,            // `bson:"HasVideo,omitempty"`
    pub location: Option<String>,          // `bson:"Location,omitempty"`
    pub file_folder_count: Option<i64>,      // `bson:"FileFolderCount,omitempty"`
    pub library_folder_count: Option<i64>,   // `bson:"LibraryFolderCount,omitempty"`
    pub volume_adjustment: Option<i64>,     // `bson:"VolumeAdjustment,omitempty"`
}

impl Track {
    fn default(id: u64) -> Track {
        Track {
            id,
            name: None,
            artist: None,
            album_artist: None,
            composer: None,
            genre: None,
            album: None,
            kind: None,
            loved: None,
            disliked: None,
            matched: None,
            explicit: None,
            compilation: None,
            part_of_gapless_album: None,
            movie: None,
            podcast: None,
            unplayed: None,
            comments: None,
            content_rating: None,
            size: None,
            total_time: None,
            disc_number: None,
            disc_count: None,
            track_number: None,
            track_count: None,
            year: None,
            bpm: None,
            date_modified: None,
            date_added: None,
            bit_rate: None,
            sample_rate: None,
            equalizer: None,
            play_count: None,
            play_date: None,
            play_date_utc: None,
            skip_count: None,
            skip_date: None,
            release_date: None,
            normalization: None,
            rating: None,
            rating_computed: None,
            album_rating: None,
            album_rating_computed: None,
            artwork_count: None,
            sort_name: None,
            sort_album: None,
            sort_album_artist: None,
            sort_composer: None,
            sort_artist: None,
            persistent_id: None,
            track_type: None,
            purchased: None,
            music_video: None,
            has_video: None,
            location: None,
            file_folder_count: None,
            library_folder_count: None,
            volume_adjustment: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
            // key => (),
            key => println!("Key: {:?}, value: {:?}", key, current_value),
        }
    }

    Ok(Library { tracks, playlists })
}

struct ElementsIterator {
    parser: EventReader<BufReader<File>>,
}

impl ElementsIterator {
    fn next_track(&mut self) -> Option<Track> {
        let track_id_str = match self.next() {
            Some(Element::Key(k)) => k,
            None => return None,
            element => panic!("Unexpected element {:?}", element),
        };
        match self.next() {
            Some(Element::Dict) => (),
            // Some(Element::Dict) => println!("Start track: {:?}", track_id),
            element => panic!("Unexpected element {:?}", element),
        };

        let track_id = track_id_str.parse::<u64>().unwrap_or_else(|_| panic!("Failed to parse track id: {:?}", track_id_str));
        let mut track = Track::default(track_id);
        loop {
            let field_key = match self.next() {
                Some(Element::Key(k)) => k,
                None => break,
                element => panic!("Unexpected element {:?}", element),
            };
            // println!("Field: {:?}", field_key);

            match field_key.as_ref() {
                "Track ID" => {self.next_int();},
                "Name"=> track.name = self.next_str(),
                "Artist"=> track.artist = self.next_str(),
                "Album Artist"=> track.album_artist = self.next_str(),
                "Composer"=> track.composer = self.next_str(),
                "Genre"=> track.genre = self.next_str(),
                "Album"=> track.album = self.next_str(),
                "Kind"=> track.kind = self.next_str(),
                "Loved"=> track.loved = self.next_bool(),
                "Disliked"=> track.disliked = self.next_bool(),
                "Matched"=> track.matched = self.next_bool(),
                "Explicit"=> track.explicit = self.next_bool(),
                "Compilation"=> track.compilation = self.next_bool(),
                "Part Of Gapless Album"=> track.part_of_gapless_album = self.next_bool(),
                "Movie"=> track.movie = self.next_bool(),
                "Podcast"=> track.podcast = self.next_bool(),
                "Unplayed"=> track.unplayed = self.next_bool(),
                "Comments"=> track.comments = self.next_str(),
                "Content Rating"=> track.content_rating = self.next_str(),
                "Size"=> track.size = self.next_int(),
                "Total Time"=> track.total_time = self.next_int(),
                "Disc Number"=> track.disc_number = self.next_int(),
                "Disc Count"=> track.disc_count = self.next_int(),
                "Track Number"=> track.track_number = self.next_int(),
                "Track Count"=> track.track_count = self.next_int(),
                "Year"=> track.year = self.next_int(),
                "BPM"=> track.bpm = self.next_int(),
                "Date Modified"=> track.date_modified = self.next_date(),
                "Date Added"=> track.date_added = self.next_date(),
                "Bit Rate"=> track.bit_rate = self.next_int(),
                "Sample Rate"=> track.sample_rate = self.next_int(),
                "Equalizer"=> track.equalizer = self.next_str(),
                "Play Count"=> track.play_count = self.next_int(),
                "Play Date"=> track.play_date = self.next_int(),
                "Play Date UTC"=> track.play_date_utc = self.next_date(),
                "Skip Count"=> track.skip_count = self.next_int(),
                "Skip Date"=> track.skip_date = self.next_date(),
                "Release Date"=> track.release_date = self.next_date(),
                "Normalization"=> track.normalization = self.next_int(),
                "Rating"=> track.rating = self.next_int(),
                "Rating Computed"=> track.rating_computed = self.next_bool(),
                "Album Rating"=> track.album_rating = self.next_int(),
                "Album Rating Computed"=> track.album_rating_computed = self.next_bool(),
                "Artwork Count"=> track.artwork_count = self.next_int(),
                "Sort Name"=> track.sort_name = self.next_str(),
                "Sort Album"=> track.sort_album = self.next_str(),
                "Sort Album Artist"=> track.sort_album_artist = self.next_str(),
                "Sort Composer"=> track.sort_composer = self.next_str(),
                "Sort Artist"=> track.sort_artist = self.next_str(),
                "Persistent ID"=> track.persistent_id = self.next_str(),
                "Track Type"=> track.track_type = self.next_str(),
                "Purchased"=> track.purchased = self.next_bool(),
                "Music Video"=> track.music_video = self.next_bool(),
                "Has Video"=> track.has_video = self.next_bool(),
                "Location"=> track.location = self.next_str(),
                "File Folder Count"=> track.file_folder_count = self.next_int(),
                "Library Folder Count"=> track.library_folder_count = self.next_int(),
                "Volume Adjustment"=> track.volume_adjustment = self.next_int(),
                field => panic!("Unknown field: {:?}", field),
            }
        }

        Some(track)
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

    fn next_bool(&mut self) -> Option<bool> {
        match self.next() {
            Some(Element::Boolean(b)) => Some(b),
            element => panic!("Unexpected element {:?}", element),
        }
    }

    fn next_int(&mut self) -> Option<i64> {
        match self.next() {
            Some(Element::Integer(i)) => Some(i),
            element => panic!("Unexpected element {:?}", element),
        }
    }

    fn next_str(&mut self) -> Option<String> {
        match self.next() {
            Some(Element::String(s)) => s,
            element => panic!("Unexpected element {:?}", element),
        }
    }

    fn next_date(&mut self) -> Option<String> {
        match self.next() {
            Some(Element::Date(d)) => Some(d),
            element => panic!("Unexpected element {:?}", element),
        }
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
        let result = parse_itunes_xml("/Users/Vladimir_Okonechnikov/workspace/rust/tauri-leptos-app/itunes-xml/tests/fixtures/single-track.xml");
        println!("{:?}", result)
        // assert_eq!(Ok(()), result);
    }
}
