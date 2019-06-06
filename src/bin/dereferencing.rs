use std::fmt;
use std::ops;

struct Album {
    pub title: String,
    pub artist: String,
}

impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.artist)
    }
}

struct Albums(pub Vec<Album>);

impl ops::Deref for Albums {
    type Target = Vec<Album>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Albums {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.iter().fold(Ok(()), |result, album| {
            result.and_then(|_| writeln!(f, "{}", album))
        })
    }
}

struct User {
    name: String,
    albums: Albums,
}

fn main() {
    let daniel = User {
        name: "Daniel".into(),
        albums: Albums(vec![
            Album {
                title: "Sgt. Pepper's Lonely Hearts Club Band".into(),
                artist: "The Beatles".into(),
            },
            Album {
                title: "Dark Side of the Moon".into(),
                artist: "Pink Floyd".into(),
            },
        ]),
    };

    println!("{}'s albums:", daniel.name);
    println!("{}", daniel.albums);
    daniel
        .albums
        .iter()
        .for_each(|album| println!("{}", album.title.to_uppercase()))
}
