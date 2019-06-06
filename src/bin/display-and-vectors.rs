use std::fmt;

struct Album {
    pub title: String,
    pub artist: String,
}

impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.artist)
    }
}

impl fmt::Display for Vec<Album> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.iter().fold(Ok(()), |result, album| {
            result.and_then(writeln!(f, "{}", album))
        })
    }
}

fn main() {
    let albums = vec![
        Album {
            title: "Sgt. Pepper's Lonely Hearts Club Band".into(),
            artist: "The Beatles".into(),
        },
        Album {
            title: "Dark Side of the Moon".into(),
            artist: "Pink Floyd".into(),
        },
    ];

    println!("{}", albums);
}
