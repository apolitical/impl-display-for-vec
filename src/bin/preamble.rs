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

fn main() {
    let album = Album {
        title: "Sgt. Pepper's Lonely Hearts Club Band".into(),
        artist: "The Beatles".into(),
    };

    println!("{}", album);
}
