How do you impl Display for Vec
===============================

This is a common question, and applies not only to Display and Vec, but how do you implement any
trait from outside your crate for any type outside your crate?

Lets create a micro app that helps us explore the problem. We'll create a simple struct implement
Display for that, then try to implement Display for a Vec of that struct.

Contents:
---------

1. [The problem](#the-problem)
    1. [Preamble](#preamble)
    2. [Display and vectors](#display-and-vectors)
2. [The solution](#the-solution)
    1. [The newtype pattern](#the-newtype-pattern)
    2. [The ownership problem](#the-ownership-problem)
    3. [Referencing](#referencing)
    4. [Dereferencing](#dereferencing)
3. [Conclusion](#conclusion)

The problem:
------------

### Preamble

To begin, we need a simple struct to play with. Lets create a simple representation of a music
album.

```rust
struct Album {
    pub title: String,
    pub artist: String,
}
```

Our app is going to print the albums in the format “title (artist)”. So, if write: 

```rust
fn main() {
    let album = Album {
        title: "Sgt. Pepper's Lonely Hearts Club Band".into(),
        artist: "The Beatles".into(),
    };

    println!("{}", album);
}
```

We want it to print:

```
Sgt. Pepper's Lonely Hearts Club Band (The Beatles)
```

To do this, we implement the `std::fmt::Display` trait for Album to format the output

```rust
use std::fmt;

impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.artist)
    }
}
```

And now when we run the program we get the expected result

```
$ cargo run --bin preamble     
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/preamble`
Sgt. Pepper's Lonely Hearts Club Band (The Beatles)
```

|Code example   | [preamble.rs]             |
|---------------|:--------------------------|
|Run the example| `cargo run --bin preamble`|

### Display and Vectors

Now to tackle the problem at hand. Lets try to implement Display for a Vec of Albums. Lets start by
creating the Vec and handing it to `println!`  

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

Obviously this won't work because we haven't describe how the Vec should be display, but lets do
some Compiler Driven Development and get an idea of whats going wrong 

```
$ cargo run --bin display-and-vectors
   Compiling impl-vec v0.1.0 (/Users/danielmason/projects/Apolitical/impl-vec)
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

As usual the compiler does it's best to tell us whats wrong, and as we expected, it won't compile
until we implement `Display` for `Vec`

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
> the formatter, however `write!` and `writeln!` return a `fmt::Result`, and we also need to return
> a `fmt::Result`. Using a fold allows us to do that multiple times and return a single result for
> the whole set. The and_then, means we won’t try to write any more after the first error, and will
> return that same error from our function.

Lets try again:

```
$ cargo run --bin display-and-vectors
   Compiling impl-vec v0.1.0 (/Users/danielmason/projects/Apolitical/impl-vec)
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> src/bin/display-and-vectors.rs:14:1
   |
14 | impl fmt::Display for Vec<Album> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl doesn't use types inside crate
   |
   = note: the impl does not reference only types defined in this crate
   = note: define and implement a trait or new type instead
```

Now we have a new error, but if we look closely the compiler has told us why this doesn't work.

```
the impl does not reference only types defined in this crate
```

In Rust you may implement traits from your crate onto types from other crates, or you may
implent traits from other crates onto your types.

> ⚠️ You may not apply other people’s traits onto other people’s types.

Since Display and Vec are both in the standard library, neither is in our crate, we may not
implement one for the other.

But, we can get around that.

|Code example   | [display-and-vectors.rs]             |
|---------------|:-------------------------------------|
|Run the example| `cargo run --bin display-and-vectors`|

The solution:
-------------

### The newtype pattern

The solution to our problem is actually mentioned in the next line of the error:

```
define and implement a trait or new type instead
```

We can’t define a new trait since we need to use `Display`, but we can define a new type. Obviously
we don’t want to create our own Vec, but we can instead use the newtype pattern.

This pattern simply wraps one type in another. In our case:

```rust
struct Albums(pub Vec<Album>);
```

> ℹ️ This pattern is normally used to improve type safety. For example if your program needs to deal
> with emails which are stored in Strings, you may want a function that only handles Emails and not
> any old String. You could use newtype idiom to enforce this:
>
> ```rust
> struct Email(pub String);
> ```

There’s one little quirk with the newtype pattern though which is that where you used to be able to
use `self` or, whatever variable contains it, you now have to use `self.0` (or `albums.0` if its
stored in a variable called "albums") to access the underlying data.

If we change our implementation of `fmt::Display` to use our newtype, it looks like this.

```rust
impl fmt::Display for Albums {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, album| {
            result.and_then(|_| writeln!(f, "{}", album))
        })
    }
}
```

We also need to wrap our Vec in our Albums newtype before we can print it.

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

There’s still a problem though, lets dig a little further.

|Code example   | [newtype.rs]                    |
|---------------|:--------------------------------|
|Run the example| `cargo run --bin newtype`       |
|Further Reading|[Rust by example: New Type Idiom]|

### The Ownership Problem

Lets contrive a more complex example. What if the albums belong to another struct. What type should
we use here?

```rust
struct User {
    name: String,
    albums: ???,
}
```

The obvious choice, and usually the right one, is to use `Albums`, but that might not work in every
use case. If we’re only using the newtype for `Display` it will add some mental overhead where we
want to use the Vec underneath.

So lets give our User a Vec of Album and look at how we can get our Albums type for Displaying it.

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

Uh oh! The problem here is that `User` owns the `Vec<Album>` data that we need for our newtype, to
get at it we only have two options:
1. We consume User and return just its albums
2. We make a copy of the data in albums (note: we can derive Clone for Album to do this)

Neither of these are particularly desirable, is there a better way?

|Code example   | [ownership-problem.rs]             |
|---------------|:-----------------------------------|
|Run the example| `cargo run --bin ownership-problem`|

### Referencing:

What if, rather than taking ownership of the data, the newtype just took a reference to the data? We
can do that, it's going to get a little rocky but it'll be worth it:

```rust
struct Albums<'a>(pub &'a Vec<Album>);
```

“Argh, lifetimes!” I hear you cry. (Or is it just me?)

Don’t worry though, all this says is that `Albums` has a lifetime `'a`, which is tied to the owned
value that `&Vec<Album>` references. I.e. The compiler will check that `Albums` isn't used after 
the owned `Vec<Album>` has been discarded.

This does change our implementation a little because we now need to acknowledge the lifetime, but
it’s not involved in the display itself so only the first line has to change.

```rust
impl<'a> fmt::Display for Albums<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, album| {
            result.and_then(|_| writeln!(f, "{}", album))
        })
    }
}
```

We can now wrap the album data without having to take ownership of it:

```rust
impl User {
    fn borrow_albums(&self) -> Albums {
        Albums(&self.albums)
    }
}
```

Where did the lifetimes go? Not that long ago, you would have had to specify the lifetimes on this
function too, but today Rust is smart enough to know if one reference is going in (`&self`), and one
is coming out (`&self.albums`) they must have the same lifetime. 

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

|Code example   | [referencing.rs]             |
|---------------|:-----------------------------|
|Run the example| `cargo run --bin referencing`|

### Dereferencing

There is one more little trick you can use to make your code even cleaner. Let’s go back to when we
said the `User.albums` probably should be the `Albums` newtype, is there anything we can do to make
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

Notice, we no longer need the `.0` when getting an iterator. The `iter` function only needs a
reference to the vector, it does not need ownership of it, so this works well.

Our User object now needs the newtype wrapper to go back in, but we can now treat albums
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

You can remove the `impl User` code entirely.

|Code example   | [dereferencing.rs]             |
|---------------|:-------------------------------|
|Run the example| `cargo run --bin dereferencing`|

Concolusion
-----------

Here's what we've learned:

1. You can not apply external traits onto external types.
2. You can use the newtype idiom to wrap types, making it yours, allowing you to apply the external 
   traits
3. You can use use references inside of newtypes
4. You can use the Deref trait to expose the contents of the newtype

<!-- Further Reading -->
[Rust by example: New Type Idiom]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
<!-- File references -->
[preamble.rs]: https://github.com/apolitical/impl-display-for-vec/blob/master/src/bin/preamble.rs
[display-and-vectors.rs]: https://github.com/apolitical/impl-display-for-vec/blob/master/src/bin/display-and-vectors.rs
[newtype.rs]: https://github.com/apolitical/impl-display-for-vec/blob/master/src/bin/newtype.rs
[ownership-problem.rs]: https://github.com/apolitical/impl-display-for-vec/blob/master/src/bin/ownership-problem.rs
[referencing.rs]: https://github.com/apolitical/impl-display-for-vec/blob/master/src/bin/referencing.rs
[dereferencing.rs]: https://github.com/apolitical/impl-display-for-vec/blob/master/src/bin/dereferencing.rs
