# Memorandum

-20201217
	the ui will to be use druid
	[druid](https://github.com/linebender/druid)

-20201108 
    Today, I dislike to write code.

- add lines for len


---


## [Is this the right way to read lines from file and split them into words in Rust?](https://stackoverflow.com/questions/25581463/is-this-the-right-way-to-read-lines-from-file-and-split-them-into-words-in-rust)

[Ask Question](https://stackoverflow.com/questions/ask)

Asked  6 years, 2 months ago

Active  [3 years, 2 months ago](https://stackoverflow.com/questions/25581463/is-this-the-right-way-to-read-lines-from-file-and-split-them-into-words-in-rust?lastactivity "2017-08-25 13:46:43Z")

Viewed  6k times


[](https://stackoverflow.com/posts/25581463/timeline "Show activity on this post.")

> Editor's note: This code example is from a version of Rust prior to 1.0 and is not syntactically valid Rust 1.0 code. Updated versions of this code produce different errors, but the answers still contain valuable information.

I've implemented the following method to return me the words from a file in a 2 dimensional data structure:

```
fn read_terms() -> Vec<Vec<String>> {
    let path = Path::new("terms.txt");
    let mut file = BufferedReader::new(File::open(&path));
    return file.lines().map(|x| x.unwrap().as_slice().words().map(|x| x.to_string()).collect()).collect();
}

```

Is this the right, idiomatic and efficient way in Rust? I'm wondering if  `collect()`  needs to be called so often and whether it's necessary to call  `to_string()`  here to allocate memory. Maybe the return type should be defined differently to be more idiomatic and efficient?

[rust](https://stackoverflow.com/questions/tagged/rust "show questions tagged 'rust'")

[share](https://stackoverflow.com/q/25581463/8146671 "short permalink to this question")  [edit](https://stackoverflow.com/posts/25581463/edit "revise and improve this post")  follow

[edited  Aug 25 '17 at 13:42](https://stackoverflow.com/posts/25581463/revisions "show all edits to this post")

[](https://stackoverflow.com/users/155423/shepmaster)

![](https://www.gravatar.com/avatar/419218774d04a581476ea1887a0921e0?s=32&d=identicon&r=PG)

[Shepmaster](https://stackoverflow.com/users/155423/shepmaster)

242k3737 gold badges626626 silver badges836836 bronze badges

asked  Aug 30 '14 at 10:40

[](https://stackoverflow.com/users/304522/micha%c5%82-fronczyk)

![](https://www.gravatar.com/avatar/8d73cb7a4363c6b8ebfa98ea06f10a78?s=32&d=identicon&r=PG)

[Michał Fronczyk](https://stackoverflow.com/users/304522/micha%c5%82-fronczyk)

1,60122 gold badges1818 silver badges2727 bronze badges

[add a comment](https://stackoverflow.com/questions/25581463/is-this-the-right-way-to-read-lines-from-file-and-split-them-into-words-in-rust?r=SearchResults# "Use comments to ask for more information or suggest improvements. Avoid answering questions in comments.")

### 2 Answers

[Active](https://stackoverflow.com/questions/25581463/is-this-the-right-way-to-read-lines-from-file-and-split-them-into-words-in-rust?answertab=active#tab-top "Answers with the latest activity first")[Oldest](https://stackoverflow.com/questions/25581463/is-this-the-right-way-to-read-lines-from-file-and-split-them-into-words-in-rust?answertab=oldest#tab-top "Answers in the order they were provided")[Votes](https://stackoverflow.com/questions/25581463/is-this-the-right-way-to-read-lines-from-file-and-split-them-into-words-in-rust?answertab=votes#tab-top "Answers with the highest score first")

7

[](https://stackoverflow.com/posts/25585564/timeline "Show activity on this post.")

You could instead read the entire file as a single  `String`  and then build a structure of references that points to the words inside:

```
use std::io::{self, Read};
use std::fs::File;

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn words_by_line<'a>(s: &'a str) -> Vec<Vec<&'a str>> {
    s.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}

fn example_use() {
    let whole_file = filename_to_string("terms.txt").unwrap();
    let wbyl = words_by_line(&whole_file);
    println!("{:?}", wbyl)
}

```

This will read the file with less overhead because it can slurp it into a single buffer, whereas reading lines with  `BufReader`  implies a lot of copying and allocating, first into the buffer inside  `BufReader`, and then into a newly allocated  `String`  for each line, and then into a newly allocated the  `String`  for each word. It will also use less memory, because the single large  `String`  and vectors of references are more compact than many individual  `String`s.

A drawback is that you can't directly return the structure of references, because it can't live past the stack frame the holds the single large  `String`. In  `example_use`  above, we have to put the large  `String`  into a  `let`  in order to call  `words_by_line`. It is possible to get around this with unsafe code and wrapping the  `String`  and references in a private struct, but that is much more complicated.

[share](https://stackoverflow.com/a/25585564/8146671 "short permalink to this answer")  [edit](https://stackoverflow.com/posts/25585564/edit "revise and improve this post")  follow

[edited  Aug 25 '17 at 13:45](https://stackoverflow.com/posts/25585564/revisions "show all edits to this post")

[](https://stackoverflow.com/users/155423/shepmaster)

![](https://www.gravatar.com/avatar/419218774d04a581476ea1887a0921e0?s=32&d=identicon&r=PG)

[Shepmaster](https://stackoverflow.com/users/155423/shepmaster)

242k3737 gold badges626626 silver badges836836 bronze badges

answered  Aug 30 '14 at 18:31

[](https://stackoverflow.com/users/1224627/wingedsubmariner)

![](https://www.gravatar.com/avatar/011b3ee0350155b8827cdc9918d2907c?s=32&d=identicon&r=PG)

[wingedsubmariner](https://stackoverflow.com/users/1224627/wingedsubmariner)

12.1k11 gold badge1919 silver badges4242 bronze badges

[add a comment](https://stackoverflow.com/questions/25581463/is-this-the-right-way-to-read-lines-from-file-and-split-them-into-words-in-rust?r=SearchResults# "Use comments to ask for more information or suggest improvements. Avoid comments like “+1” or “thanks”.")

10

[](https://stackoverflow.com/posts/39434382/timeline "Show activity on this post.")

There is a shorter and more readable way of getting words from a text file.

```
use std::io::{BufRead, BufReader};
use std::fs::File;

let reader = BufReader::new(File::open("file.txt").expect("Cannot open file.txt"));

for line in reader.lines() {
    for word in line.unwrap().split_whitespace() {
        println!("word '{}'", word);
    }
}

```

[share](https://stackoverflow.com/a/39434382/8146671 "short permalink to this answer")  [edit](https://stackoverflow.com/posts/39434382/edit "revise and improve this post")  follow

[edited  Aug 25 '17 at 13:46](https://stackoverflow.com/posts/39434382/revisions "show all edits to this post")

[](https://stackoverflow.com/users/155423/shepmaster)

![](https://www.gravatar.com/avatar/419218774d04a581476ea1887a0921e0?s=32&d=identicon&r=PG)

[Shepmaster](https://stackoverflow.com/users/155423/shepmaster)

242k3737 gold badges626626 silver badges836836 bronze badges

answered  Sep 11 '16 at 8:50

[](https://stackoverflow.com/users/691825/katomaso)

![](https://i.stack.imgur.com/tQ0zP.jpg?s=32&g=1)

[katomaso](https://stackoverflow.com/users/691825/katomaso)

17222 silver badges88 bronze badges

[add a comment](https://stackoverflow.com/questions/25581463/is-this-the-right-way-to-read-lines-from-file-and-split-them-into-words-in-rust?r=SearchResults# "Use comments to ask for more information or suggest improvements. Avoid comments like “+1” or “thanks”.")

### Your Answer