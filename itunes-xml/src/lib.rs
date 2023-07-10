use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::BufReader;
use std::{fmt::Debug, fs::File};

use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Library {
    pub metadata: HashMap<String, Element>,
    pub tracks: HashMap<u64, Track>,
    pub playlists: HashMap<u64, Playlist>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Element {
    Plist,
    Dict,
    Array,
    Key(String),
    Boolean(bool),
    Integer(i64),
    String(Option<String>),
    Data(Option<String>),
    Date(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Track {
    pub id: u64,
    pub name: Option<String>,                // `bson:"Name,omitempty"`
    pub artist: Option<String>,              // `bson:"Artist,omitempty"`
    pub album_artist: Option<String>,        // `bson:"AlbumArtist,omitempty"`
    pub composer: Option<String>,            // `bson:"Composer,omitempty"`
    pub genre: Option<String>,               // `bson:"Genre,omitempty"`
    pub album: Option<String>,               // `bson:"Album,omitempty"`
    pub kind: Option<String>,                // `bson:"Kind,omitempty"`
    pub loved: Option<bool>,                 // `bson:"Loved,omitempty"`
    pub disliked: Option<bool>,              // `bson:"Disliked,omitempty"`
    pub matched: Option<bool>,               // `bson:"Matched,omitempty"`
    pub explicit: Option<bool>,              // `bson:"Explicit,omitempty"`
    pub compilation: Option<bool>,           // `bson:"Compilation,omitempty"`
    pub part_of_gapless_album: Option<bool>, // `bson:"PartOfGaplessAlbum,omitempty"`
    pub movie: Option<bool>,                 // `bson:"Movie,omitempty"`
    pub podcast: Option<bool>,               // `bson:"Podcast,omitempty"`
    pub unplayed: Option<bool>,              // `bson:"Unplayed,omitempty"`
    pub comments: Option<String>,            // `bson:"Comments,omitempty"`
    pub content_rating: Option<String>,      // `bson:"ContentRating,omitempty"`
    pub size: Option<i64>,                   // `bson:"Size,omitempty"`
    pub total_time: Option<i64>,             // `bson:"TotalTime,omitempty"`
    pub disc_number: Option<i64>,            // `bson:"DiscNumber,omitempty"`
    pub disc_count: Option<i64>,             // `bson:"DiscCount,omitempty"`
    pub track_number: Option<i64>,           // `bson:"TrackNumber,omitempty"`
    pub track_count: Option<i64>,            // `bson:"TrackCount,omitempty"`
    pub year: Option<i64>,                   // `bson:"Year,omitempty"`
    pub bpm: Option<i64>,                    // `bson:"BPM,omitempty"`
    pub date_modified: Option<String>,       // `bson:"DateModified,omitempty"`
    pub date_added: Option<String>,          // `bson:"DateAdded,omitempty"`
    pub bit_rate: Option<i64>,               // `bson:"BitRate,omitempty"`
    pub sample_rate: Option<i64>,            // `bson:"SampleRate,omitempty"`
    pub equalizer: Option<String>,           // `bson:"Equalizer,omitempty"`
    pub play_count: Option<i64>,             // `bson:"PlayCount,omitempty"`
    pub play_date: Option<i64>,              // `bson:"PlayDate,omitempty"`
    pub play_date_utc: Option<String>,       // `bson:"PlayDateUTC,omitempty"`
    pub skip_count: Option<i64>,             // `bson:"SkipCount,omitempty"`
    pub skip_date: Option<String>,           // `bson:"SkipDate,omitempty"`
    pub release_date: Option<String>,        // `bson:"ReleaseDate,omitempty"`
    pub normalization: Option<i64>,          // `bson:"Normalization,omitempty"`
    pub rating: Option<i64>,                 // `bson:"Rating,omitempty"`
    pub rating_computed: Option<bool>,       // `bson:"RatingComputed,omitempty"`
    pub album_rating: Option<i64>,           // `bson:"AlbumRating,omitempty"`
    pub album_rating_computed: Option<bool>, // `bson:"AlbumRatingComputed,omitempty"`
    pub artwork_count: Option<i64>,          // `bson:"ArtworkCount,omitempty"`
    pub sort_name: Option<String>,           // `bson:"SortName,omitempty"`
    pub sort_album: Option<String>,          // `bson:"SortAlbum,omitempty"`
    pub sort_album_artist: Option<String>,   // `bson:"SortAlbumArtist,omitempty"`
    pub sort_composer: Option<String>,       // `bson:"SortComposer,omitempty"`
    pub sort_artist: Option<String>,         // `bson:"SortArtist,omitempty"`
    pub persistent_id: Option<String>,       // `bson:"PersistentID,omitempty"`
    pub track_type: Option<String>,          // `bson:"TrackType,omitempty"`
    pub purchased: Option<bool>,             // `bson:"Purchased,omitempty"`
    pub music_video: Option<bool>,           // `bson:"MusicVideo,omitempty"`
    pub has_video: Option<bool>,             // `bson:"HasVideo,omitempty"`
    pub hd: Option<bool>,                    // `bson:"HD,omitempty"`
    pub favorited: Option<bool>,             // `bson:"Favorited,omitempty"`
    pub location: Option<String>,            // `bson:"Location,omitempty"`
    pub file_folder_count: Option<i64>,      // `bson:"FileFolderCount,omitempty"`
    pub library_folder_count: Option<i64>,   // `bson:"LibraryFolderCount,omitempty"`
    pub volume_adjustment: Option<i64>,      // `bson:"VolumeAdjustment,omitempty"`
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Playlist {
    pub id: u64,                     // <key>Playlist ID</key><integer>50344</integer>
    pub name: String, // `bson:"Name,omitempty"` // <key>Name</key><string>Music</string>
    pub persistent_id: String, // `bson:"PersistentID,omitempty"` // <key>Playlist Persistent ID</key><string>87864958089CA4B9</string>
    pub description: Option<String>, // `bson:"Description,omitempty"` // <key>Description</key><string></string>
    pub parent_persistent_id: Option<String>, // `bson:"PersistentID,omitempty"` // <key>Parent Persistent ID</key><string>87864958089CA4B9</string>
    pub all_items: bool, // `bson:"AllItems,omitempty"` // <key>All Items</key><true/>
    pub distinguished_kind: Option<i64>, // <key>Distinguished Kind</key><integer>4</integer>
    pub music: Option<bool>, // <key>Music</key><true/>
    pub master: Option<bool>, // <key>Master</key><true/>
    pub visible: Option<bool>, // <key>Visible</key><true/>
    pub folder: Option<bool>, // <key>Folder</key><true/>
    pub movies: Option<bool>, // <key>Movies</key><true/>
    pub tv_shows: Option<bool>, // TV Shows
    pub audiobooks: Option<bool>, // Audiobooks
    pub podcasts: Option<bool>, // Podcasts
    pub items: HashSet<u64>, // `bson:"Items,omitempty"` // <key>Playlist Items</key>
    pub smart_info: Option<String>, /*
                         <key>Smart Info</key>
                         <data>
                         AQEAAwAAAAIAAAAZAAAAAAAAAAcAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
                         AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
                         AAAAAA==
                         </data>
                         */
    pub smart_criteria: Option<String>, /*
                                           <key>Smart Criteria</key>
                                           <data>
                                           U0xzdAABAAEAAAADAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
                                           AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
                                           AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADwAAAQAAAAAAAAAAAAAAAAAAAAAAAAA
                                           AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABEAAAAAAAQIbEAAAAAAAAAAAAAAAAAAAAB
                                           AAAAAAAQIbEAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA8AgAEAAAA
                                           AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAARAAAAAAAIIAE
                                           AAAAAAAAAAAAAAAAAAAAAQAAAAAAIIAEAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAA
                                           AAAAAAAAAAAAhQAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
                                           AAAAAAAAAEQAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAQAAAAAAAAAAAAAAAAAA
                                           AAEAAAAAAAAAAAAAAAAAAAAAAAAAAA==
                                           </data>
                                        */
}

pub fn parse_itunes_xml(file_path: &str) -> Result<Library, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let parser = EventReader::new(reader);
    let elements_iterator = ElementsIterator { parser };

    let library = parse_document(elements_iterator)?;
    Ok(library)
}

fn parse_document(mut it: ElementsIterator) -> Result<Library, String> {
    let mut tracks = HashMap::<u64, Track>::new();
    let mut playlists = HashMap::<u64, Playlist>::new();
    let mut metadata = HashMap::<String, Element>::new();

    match it.next() {
        Some(Element::Plist) => {
            // println!("Skip plist wrapper start");
        }
        element => return Err(format!("Unexpected element {:?}", element)),
    };

    match it.next() {
        Some(Element::Dict) => {
            // println!("Skip root dict start")
        }
        element => return Err(format!("Unexpected element {:?}", element)),
    };

    loop {
        let current_key = match it.next() {
            Some(Element::Key(k)) => k,
            Some(element) => return Err(format!("Unexpected element {:?}", element)),
            None => break,
        };
        let current_value = match it.next() {
            Some(element) => element,
            None => return Err(format!("Received key without the value: {}", current_key)),
        };

        match current_key.as_str() {
            "Tracks" => {
                while let Some(track) = it.next_track() {
                    tracks.insert(track.id, track);
                }
            }
            "Playlists" => {
                while let Some(playlist) = it.next_playlist() {
                    playlists.insert(playlist.id, playlist);
                }
            }
            _ => {
                metadata.insert(current_key, current_value);
            }
        }
    }

    Ok(Library {
        metadata,
        tracks,
        playlists,
    })
}

struct ElementsIterator {
    parser: EventReader<BufReader<File>>,
}

impl ElementsIterator {
    fn next_track(&mut self) -> Option<Track> {
        let mut track = Track::default();

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

        track.id = track_id_str
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("Failed to parse track id: {:?}", track_id_str));
        loop {
            let field_key = match self.next() {
                Some(Element::Key(k)) => k,
                None => break,
                element => panic!("Unexpected element {:?}", element),
            };
            // println!("Field: {:?}", field_key);

            match field_key.as_ref() {
                "Track ID" => {
                    self.next_int();
                }
                "Name" => track.name = self.next_str(),
                "Artist" => track.artist = self.next_str(),
                "Album Artist" => track.album_artist = self.next_str(),
                "Composer" => track.composer = self.next_str(),
                "Genre" => track.genre = self.next_str(),
                "Album" => track.album = self.next_str(),
                "Kind" => track.kind = self.next_str(),
                "Loved" => track.loved = self.next_bool(),
                "Disliked" => track.disliked = self.next_bool(),
                "Matched" => track.matched = self.next_bool(),
                "Explicit" => track.explicit = self.next_bool(),
                "Compilation" => track.compilation = self.next_bool(),
                "Part Of Gapless Album" => track.part_of_gapless_album = self.next_bool(),
                "Movie" => track.movie = self.next_bool(),
                "Podcast" => track.podcast = self.next_bool(),
                "Unplayed" => track.unplayed = self.next_bool(),
                "Comments" => track.comments = self.next_str(),
                "Content Rating" => track.content_rating = self.next_str(),
                "Size" => track.size = self.next_int(),
                "Total Time" => track.total_time = self.next_int(),
                "Disc Number" => track.disc_number = self.next_int(),
                "Disc Count" => track.disc_count = self.next_int(),
                "Track Number" => track.track_number = self.next_int(),
                "Track Count" => track.track_count = self.next_int(),
                "Year" => track.year = self.next_int(),
                "BPM" => track.bpm = self.next_int(),
                "Date Modified" => track.date_modified = self.next_date(),
                "Date Added" => track.date_added = self.next_date(),
                "Bit Rate" => track.bit_rate = self.next_int(),
                "Sample Rate" => track.sample_rate = self.next_int(),
                "Equalizer" => track.equalizer = self.next_str(),
                "Play Count" => track.play_count = self.next_int(),
                "Play Date" => track.play_date = self.next_int(),
                "Play Date UTC" => track.play_date_utc = self.next_date(),
                "Skip Count" => track.skip_count = self.next_int(),
                "Skip Date" => track.skip_date = self.next_date(),
                "Release Date" => track.release_date = self.next_date(),
                "Normalization" => track.normalization = self.next_int(),
                "Rating" => track.rating = self.next_int(),
                "Rating Computed" => track.rating_computed = self.next_bool(),
                "Album Rating" => track.album_rating = self.next_int(),
                "Album Rating Computed" => track.album_rating_computed = self.next_bool(),
                "Artwork Count" => track.artwork_count = self.next_int(),
                "Sort Name" => track.sort_name = self.next_str(),
                "Sort Album" => track.sort_album = self.next_str(),
                "Sort Album Artist" => track.sort_album_artist = self.next_str(),
                "Sort Composer" => track.sort_composer = self.next_str(),
                "Sort Artist" => track.sort_artist = self.next_str(),
                "Persistent ID" => track.persistent_id = self.next_str(),
                "Track Type" => track.track_type = self.next_str(),
                "Purchased" => track.purchased = self.next_bool(),
                "Music Video" => track.music_video = self.next_bool(),
                "Has Video" => track.has_video = self.next_bool(),
                "HD" => track.hd = self.next_bool(),
                "Favorited" => track.favorited = self.next_bool(),
                "Location" => track.location = self.next_str(),
                "File Folder Count" => track.file_folder_count = self.next_int(),
                "Library Folder Count" => track.library_folder_count = self.next_int(),
                "Volume Adjustment" => track.volume_adjustment = self.next_int(),
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

        let mut playlist = Playlist::default();

        loop {
            let field_key = match self.next() {
                Some(Element::Key(k)) => k,
                None => break,
                element => panic!("Unexpected element {:?}", element),
            };

            match field_key.as_str() {
                "Playlist ID" => playlist.id = self.next_int().unwrap() as u64,
                "Name" => playlist.name = self.next_str().unwrap(),
                "Playlist Persistent ID" => playlist.persistent_id = self.next_str().unwrap(),
                "Description" => playlist.description = self.next_str(),
                "Parent Persistent ID" => playlist.parent_persistent_id = self.next_str(),
                "All Items" => playlist.all_items = self.next_bool().unwrap(),
                "Distinguished Kind" => playlist.distinguished_kind = self.next_int(),
                "Music" => playlist.music = self.next_bool(),
                "Master" => playlist.master = self.next_bool(),
                "Visible" => playlist.visible = self.next_bool(),
                "Folder" => playlist.folder = self.next_bool(),
                "Movies" => playlist.movies = self.next_bool(),
                "TV Shows" => playlist.tv_shows = self.next_bool(),
                "Audiobooks" => playlist.audiobooks = self.next_bool(),
                "Podcasts" => playlist.podcasts = self.next_bool(),
                "Smart Info" => playlist.smart_info = self.next_str(),
                "Smart Criteria" => playlist.smart_criteria = self.next_str(),
                "Playlist Items" => {
                    match self.next() {
                        Some(Element::Array) => (),
                        element => panic!("Unexpected element {:?}", element),
                    }

                    loop {
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
                        match self.next_int() {
                            Some(id) => playlist.items.insert(id as u64),
                            // Some(Element::Integer(i)) => println!("Playlist item: {:?}", i),
                            element => panic!("Unexpected element {:?}", element),
                        };
                        match self.next() {
                            None => {
                                // println!("Stop playlist item")
                            }
                            element => panic!("Unexpected element {:?}", element),
                        };
                    }
                }
                element => panic!("Unexpected element {:?}", element),
            }
        }

        Some(playlist)
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
                        tag => panic!("Unsupported element {:?}", tag),
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
        let result = parse_itunes_xml("tests/fixtures/single-track.xml");
        println!("{:?}", result)
        // assert_eq!(Ok(()), result);
    }
}
