// WARNING:
// - the lifetimes are not set correctly, you have to set them to make it compile
// - you have also to implemment missing functions and fix the code
// - *** see test test functions in the code for usage examples

use std::fs::File;
use std::io;
use std::io::BufRead;

// (1) LineEditor: implement functionality
pub struct LineEditor {
    lines: Vec<String>,
}

impl LineEditor {
    pub fn new(s: String) -> Self {
        let lines: Vec<String> = s.split("\n").map(|s| s.to_string()).collect();
        Self { lines }
    }

    // create a new LineEditor from a file
    pub fn from_file(file_name: &str) -> Result<Self, io::Error> {
        let file = File::open(file_name)?;
        let reader = io::BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
        Ok(Self { lines })
    }

    pub fn all_lines(&self) -> Vec<&str> {
        self.lines.iter().map(|l| l.as_str()).collect()
    }

    pub fn replace(&mut self, line: usize, start: usize, end: usize, subst: &str) {
        if line > self.lines.len(){
            return;
        }
        if start < 0 || end > self.lines.len() {
            return;
        }
        let mut new_line = String::new();
        new_line.push_str(&self.lines[line][0..start]);
        new_line.push_str(subst);
        new_line.push_str(&self.lines[line][end..]);
        self.lines[line] = new_line;
    }
}

// (2) Match contains the information about the match. Fix the lifetimes
// repl will contain the replacement.
// It is an Option because it may be not set yet or it may be skipped
#[derive(Clone)]
struct Match<'a> {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub text: &'a str,
    pub repl: Option<String>,
}

// use the crate "regex" to find the pattern and its method find_iter for iterating over the matches
// modify if necessary, this is just an example for using a regex to find a pattern
fn find_example<'a, 'b>(lines: &'b Vec<&'a str>, pattern: &'a str) -> Vec<Match<'a>> {
    let mut matches = Vec::new();
    let re = regex::Regex::new(pattern).unwrap();
    for (line_idx, line) in lines.iter().enumerate() {
        for mat in re.find_iter(line) {
            matches.push(Match {
                line: line_idx,
                start: mat.start(),
                end: mat.end(),
                text: &line[mat.start()..mat.end()],
                repl: None,
            });
        }
    }
    matches
}

// (3) Fix the lifetimes of the FindReplace struct
// (4) implement the Finder struct
struct FindReplace<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    matches: Vec<Match<'a>>,
}

impl<'a> FindReplace<'a>{
    pub fn new(lines: Vec<&'a str>, pattern: &'a str) -> FindReplace<'a> {
        let mut fr = FindReplace{
            lines: lines.clone(),
            pattern: pattern.to_string(),
            matches: Vec::new(),
        };
        fr.matches = find_example(&(fr.lines.clone()), pattern);
        fr
    }
    // return all the matches
    pub fn matches(&self) -> &Vec<Match> {
        &self.matches
    }

    // apply a function to all matches and allow to accept them and set the repl
    // useful for promptig the user for a replacement
    pub fn apply(&mut self, fun: impl Fn(&mut Match) -> bool) {
        self.matches.iter_mut().for_each(|m| {fun(m);});
    }
}


//(5) how FindReplace should work together with the LineEditor in order
// to replace the matches in the text
#[test]
fn test_find_replace() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = FindReplace::new(lines, "ll");

    // find all the matches and accept them
    finder.apply(|m| {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
        m.repl = Some("some repl".to_string());
        true
    });

    // now let's replace the matches
    // why this loop won't work?
    //for m: Match in finder.matches() {
    //    editor.replace(/* add match */);
    //}

    // alternate method: why this one works?

    let mut subs = Vec::new();
    for m in finder.matches() {
        //subs.push( /** add match if repl is set */);
    }

    for (line, start, end, subst) in subs {
        editor.replace(line, start, end, subst);
    }

}


// (6) sometimes it's very expensive to find all the matches at once before applying
// the changes
// we can implement a lazy finder that finds just the next match and returns it
// each call to next() will return the next match
// this is a naive implementation of an Iterarator

#[derive(Debug, Clone, Copy)]
struct FinderPos {
    pub line: usize,
    pub offset: usize,
}

struct LazyFinder<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    pos: Option<FinderPos>,
}

impl<'a> LazyFinder<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &'a str) -> Self {
        let mut lf = LazyFinder{
            lines: lines.clone(),
            pattern: pattern.to_string(),
            pos: None,
        };
        let m = find_example(&lines, &pattern);
        lf.pos.unwrap().offset = 0;
        lf.pos.unwrap().line = m[0].line;
        lf
    }

    pub fn next(&'a mut self) -> Option<Match<'a>> {
        // remember:
        // return None if there are no more matches
        // return Some(Match) if there is a match
        // each time save the position of the match for the next call
        let mv = find_example(&self.lines, &self.pattern);
        if mv.is_empty() {
            return None
        }
        self.pos.unwrap().offset += 1;
        let index = self.pos.unwrap().offset;
        self.pos.unwrap().line = mv[index].line;
        Some(mv[index].clone())
    }
}

// (7) example of how to use the LazyFinder
#[test]
fn test_lazy_finder() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = LazyFinder::new(lines, "ll");

    // find all the matches and accept them
    while let Some(m) = finder.next() {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
    }
}


// (8) now you have everything you need to implement the real Iterator
/*
struct FindIter {
    lines: Vec<&str>,
    pattern: String,
    // ... other?
}

impl FindIter {
    pub fn new(lines: Vec<&str>, pattern: &str) -> Self {
        unimplemented!()
    }
}

impl Iterator for FindIter {
    type Item = Match; // <== we inform the Iterator that we return a Match

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

// (9) test the find iterator
#[test]
fn test_find_iter() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = FindIter::new(lines, "ll");

    // find all the matches and accept them
    for m in finder {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);

    }
}*/

