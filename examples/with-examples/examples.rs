
pub struct Example {
    pub title: &'static str,
    pub cmd: &'static str,
    pub comments: &'static str,
}

impl Example {
    pub const fn new(
        title: &'static str,
        cmd: &'static str,
        comments: &'static str,
    ) -> Self {
        Self { title, cmd, comments }
    }
}

pub static EXAMPLES: &[Example] = &[
    Example::new(
        "Default computation on your prefered path",
        "withex ~",
        ""
    ),
    Example::new(
        "Compute for a height of 37",
        "withex -h 37",
        "This uses the default value (`3`) for the width
        "
    ),
    Example::new(
        "Maximum precision",
        "withex -h 37 -w 28 --strategy precise",
        "This may take a while but it's *super* precise"
    ),
];
