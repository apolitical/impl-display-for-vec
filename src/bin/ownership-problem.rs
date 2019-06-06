use std::fmt;

#[derive(Clone)]
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

impl fmt::Display for Albums {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, album| {
            result.and_then(|_| writeln!(f, "{}", album))
        })
    }
}

struct User {
    name: String,
    albums: Vec<Album>,
}

impl User {
    fn into_album(self) -> Albums {
        Albums(self.albums)
    }

    fn get_albums(&self) -> Albums {
        Albums(self.albums.clone())
    }
}

fn main() {
    let daniel = User {
        name: "Daniel".into(),
        albums: vec![
            Album {
                title: "Sgt. Pepper's Lonely Hearts Club Band".into(),
                artist: "The Beatles".into(),
            },
            Album {
                title: "Dark Side of the Moon".into(),
                artist: "Pink Floyd".into(),
            },
        ],
    };

    println!("{}'s albums:", daniel.name);
    println!("{}", daniel.get_albums());

    println!("{}'s albums:", daniel.name);
    println!("{}", daniel.into_album());
}
