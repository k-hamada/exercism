use failure::Error;
use std::fs;

pub struct Flags {
    n: bool,
    i: bool,
    l: bool,
    x: bool,
    v: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Flags {
            n: flags.contains(&"-n"),
            i: flags.contains(&"-i"),
            l: flags.contains(&"-l"),
            x: flags.contains(&"-x"),
            v: flags.contains(&"-v"),
        }
    }
}

struct GrepPattern {
    context: String,
    case_insensitive: bool,
    match_entire_lines: bool,
    invert: bool,
}

impl GrepPattern {
    fn new(context: &str, flag: &Flags) -> Self {
        let context = if flag.i {
            context.to_lowercase()
        } else {
            context.to_string()
        };
        let case_insensitive = flag.i;
        let match_entire_lines = flag.x;
        let invert = flag.v;

        GrepPattern {
            context,
            case_insensitive,
            match_entire_lines,
            invert,
        }
    }

    fn include(&self, line: &str) -> bool {
        let included = match (self.case_insensitive, self.match_entire_lines) {
            (true, true) => line.to_lowercase() == self.context,
            (true, false) => line.to_lowercase().contains(&self.context),
            (false, true) => line == self.context,
            (false, false) => line.contains(&self.context),
        };
        included ^ self.invert
    }
}

struct GrepLine {
    context: String,
    file: String,
    number: usize,
}

impl GrepLine {
    pub fn new(context: &str, file: &str, index: usize) -> Self {
        GrepLine {
            context: context.to_string(),
            file: file.to_string(),
            number: index + 1,
        }
    }

    fn include(&self, pattern: &GrepPattern) -> bool {
        pattern.include(&self.context)
    }

    fn to_string(&self, flags: &Flags, many_files: bool) -> String {
        if flags.l {
            return self.file.to_string();
        }

        match (flags.n, flags.l || many_files) {
            (true, true) => format!("{}:{}:{}", self.file, self.number, self.context),
            (true, false) => format!("{}:{}", self.number, self.context),
            (false, true) => format!("{}:{}", self.file, self.context),
            (false, false) => self.context.to_string(),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let pattern = GrepPattern::new(pattern, flags);
    let many_files = files.len() > 1;

    files
        .iter()
        .map(|file| fs::read_to_string(file).map(|contents| (file, contents)))
        .collect::<Result<Vec<_>, _>>()
        .map(|files| {
            files
                .iter()
                .flat_map(|(name, contents)| file_grep(name, contents, &pattern, flags, many_files))
                .collect()
        })
        .map_err(std::convert::Into::into)
}

fn file_grep(
    name: &str,
    contents: &str,
    pattern: &GrepPattern,
    flags: &Flags,
    many_files: bool,
) -> Vec<String> {
    let mut candidate = contents
        .lines()
        .enumerate()
        .map(|(i, line)| GrepLine::new(line, name, i))
        .filter(|line| line.include(pattern))
        .map(|line| line.to_string(flags, many_files))
        .collect::<Vec<_>>();
    if flags.l {
        candidate.dedup();
    };
    candidate
}
