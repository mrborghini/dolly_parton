pub struct Colors;

impl Colors {
    const HEADER: &'static str = "\x1b[95m";
    const OKBLUE: &'static str = "\x1b[94m";
    const OKCYAN: &'static str = "\x1b[96m";
    const OKGREEN: &'static str = "\x1b[92m";
    const WARNING: &'static str = "\x1b[93m";
    const ERROR: &'static str = "\x1b[91m";
    const NORMAL: &'static str = "\x1b[0m";
    const BOLD: &'static str = "\x1b[1m";
    const UNDERLINE: &'static str = "\x1b[4m";

    pub fn header() -> &'static str { Colors::HEADER }
    pub fn ok_blue() -> &'static str { Colors::OKBLUE }
    pub fn ok_cyan() -> &'static str { Colors::OKCYAN }
    pub fn ok_green() -> &'static str { Colors::OKGREEN }
    pub fn warning() -> &'static str { Colors::WARNING }
    pub fn error() -> &'static str { Colors::ERROR }
    pub fn normal() -> &'static str { Colors::NORMAL }
    pub fn bold() -> &'static str { Colors::BOLD }
    pub fn underline() -> &'static str { Colors::UNDERLINE }
}
