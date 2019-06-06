impl Display for Vec
====================

The problem:
------------

### Preamble

We have a struct, lets say an album, we’d like to be able to format when used in macro’s such as 
`println!`.

We’ll keep the struct simple:

```rust
struct Album {
    pub title: String,
    pub artist: String,
}
```

We want to be able to print this out in the format “title (artist)”, for example:

```rust
fn main() {
    let album = Album {
        title: "Sgt. Pepper's Lonely Hearts Club Band".into(),
        artist: "The Beatles".into(),
    };

    println!("{}", album);
}
```

Should print out:

```
Sgt. Pepper's Lonely Hearts Club Band (The Beatles)
```

To do this, all we need to do is implement the Display trait for Album:

```rust
use std::fmt;

impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.artist)
    }
}
```

With those three blocks in place, the program will produce the expected result.

### Display and Vectors

I assume you’re already comfortable with the above, that’s not why you’re here. You’re here because
you want to do that, but with something in a Rust Vec.

What we’d like to do now is show a vector of `Album`s, as before but with each one on a new line.

```rust
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
```

We know this won’t work on its own, but let’s have Rusts compiler help us out here and try to build
this... 
 
... so we get the following error:

```
error[E0277]: `std::vec::Vec<Album>` doesn't implement `std::fmt::Display`
  --> src/bin/display-and-vectors.rs:26:20
   |
26 |     println!("{}", albums);
   |                    ^^^^^^ `std::vec::Vec<Album>` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `std::vec::Vec<Album>`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: required by `std::fmt::Display::fmt`
```

And right there under “help” rust tells us what we need to do.

```
the trait std::fmt::Display is not implemented for std::vec::Vec<Album>
```

So let’s do that:

```rust
impl fmt::Display for Vec<Album> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.iter().fold(Ok(()), |result, album| {
            result.and_then(|_| writeln!(f, "{}", album))
        })
    }
}
```


> ℹ️ A word on what this function is doing. For each item in the Vec, we want to write a new line to
the formatter, however `write!` and `writeln!` return a `fmt::Result`, and we also need to return a
`fmt::Result`. Using a fold allows us to do that multiple times and return a single result for the
whole set. The and_then, means we won’t try to write any more after the first error, and will return
that same error from our function.

Now this is probably where you’ve gotten to yourself, so you’ll know that this won’t compile, and
indeed, here’s the error:

```
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> src/bin/display-and-vectors.rs:14:1
   |
14 | impl fmt::Display for Vec<Album> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl doesn't use types inside crate
   |
   = note: the impl does not reference only types defined in this crate
   = note: define and implement a trait or new type instead
```

The important part here is that the impl does not reference only types defined in this crate. In
Rust you may apply  your traits onto other people’s types, or you may apply other people’s traits
onto your types. 

> ⚠️ You may not apply other people’s traits onto other people’s types.

Since Display and Vec are not ours (they belong to the standard library) we may not implement one
for the other.

The solution:
-------------

### The newtype pattern

The solution to our problem is actually mentioned in the next line:

```
define and implement a trait or new type instead
```

We can’t define a new trait since we need to use Display, but we can define a new type. Obviously
we don’t want to create our own Vec, but we can instead use the newtype pattern (newtype is one
word).

This pattern simply wraps one type in another. In our case:

```rust
struct Albums(pub Vec<Album>);
```

> ℹ️ This pattern is normally used to improve type safety. You can imagine for example if Emails are
> stored as Strings, you may want a function that only handles Emails and not any old String. You
> could use newtype to enforce this:
>
> ```rust
> struct Email(pub String);
> ```

There’s one little quirk with the newtype pattern though which is that where you used to be able to
use self or, directly use the value we care about, if you want to do that now you have to use
`self.0` (or `albums.0` if its stored in a variable called "albums").

If we change our implementation of `fmt::Display` to use our newtype, it looks like this.

```rust
impl fmt::Display for Albums {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.iter().fold(Ok(()), |result, album| {
            result.and_then(|_| writeln!(f, "{}", album))
        })
    }
}
```

Before it’ll work though, we also need to change how we use it in our main function by wrapping the
vec in our newtype.

```rust
fn main() {
    let albums = Albums(vec![
        Album {
            title: "Sgt. Pepper's Lonely Hearts Club Band".into(),
            artist: "The Beatles".into(),
        },
        Album {
            title: "Dark Side of the Moon".into(),
            artist: "Pink Floyd".into(),
        },
    ]);

    println!("{}", albums);
}
```

Now our program outputs exactly what we wanted.

```
Sgt. Pepper's Lonely Hearts Club Band (The Beatles)
Dark Side of the Moon (Pink Floyd)
```

There’s still a problem though!

### The Ownership Problem

This solution is all very well for our use case here, but our Albums type takes ownership of the
data its given. This may not always be the appropriate thing to do. For example, say we have a User
type, and the User owns a collection of albums.

```rust
struct User {
    name: String,
    albums: ???,
}
```

What type do we make `albums`? The obvious choice, and usually the right one, is to use `Albums`,
but that might not work in every use case. If we’re only using the newtype for `Display` it will
add overhead for things like serde serialization/deserialization. So, for the sake of argument,
let’s say we need the User albums to be `Vec<Album>`, how do we display it now?

Well, we could take albums from user, then wrap it up, but that means the caller needs to know how
to do that. We could implement a function on User that returns an Albums type, lets look at that.

```rust
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
```

Uh oh! User owns the `Vec<Album>` data we need so using our previous newtype we only have two
options
1. We consume User and return just its albums
2. We make a copy of the data in albums (which also requires Album implements or derives Clone)

Neither of these are particularly desirable, is there a better way?

### Referencing:

What if, rather than taking ownership of the data, the newtype just needed a reference to where the
data already exists?

```rust
struct Albums<'a>(pub &'a Vec<Album>);
```

“Argh! Lifetimes!” I hear you cry. 

Don’t worry! All this says is that `Albums` has a lifetime `'a`, which is tied to the reference it
holds. I.e. `Albums` can not out live the `&Vec<Album>` inside of it.

This does change our implementation a little bit because we now need to acknowledge the lifetime,
but it’s not involved in the display itself so only the first line has to change.

```rust
impl<'a> fmt::Display for Albums<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, album| {
            result.and_then(|_| writeln!(f, "{}", album))
        })
    }
}
```

Notice that, other than the first line, everything else is identical to before. We can now wrap the
album data without having to take ownership of it:

```rust
impl User {
    fn borrow_albums(&self) -> Albums {
        Albums(&self.albums)
    }
}
```

Where did the lifetimes go? Years ago you would have had to specify the lifetimes, but today Rust
is smart enough to know if one reference is going in (`&self`), and one is coming out
(`&self.albums`) they must have the same lifetime. 

Our code now works as you’d expect without making any unnecessary memory allocations, or consuming
data we may want to use later.

```rust
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
    println!("{}", daniel.borrow_albums());
}
```

### Dereferencing

There is one more little trick you can use to make your code even cleaner. Let’s go back to when we
said the `User.albums` probably should be an `Albums` type, is there anything we can do to make
using it easier? 

For example, we don’t want to type `daniel.albums.0` every time we want access to the underlying
vector.

Well, there’s a trait for that, `std::ops::Deref`. Going back to our original type that owned its
data:

```rust
struct Albums(pub Vec<Album>);
```

We can implement the `Deref` trait to allow the outer type to be treated as though it is a reference
to the inner type. We implement `Deref` for Album like this:

```rust
use std::ops;

impl ops::Deref for Albums {
    type Target = Vec<Album>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

We can take immediate advantage of this in our Display implementation:

```rust
impl fmt::Display for Albums {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.iter().fold(Ok(()), |result, album| {
            result.and_then(|_| writeln!(f, "{}", album))
        })
    }
}
```

Notice, we no longer need the `.0` when getting an iterator. The iter function only needs a
reference to the vector, it does not need ownership so this is perfect.

Creating our User object now needs the newtype wrapper to go back in, but we can now treat albums
as both a Albums type and a `&Vec<Album>` type.

```rust
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
```

And now you know far more about impl Display for Vec than you ever needed to know!

Hope it was fun,

Daniel
