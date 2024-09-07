pub struct Colors;

impl Colors {
    const OKBLUE: &'static str = "\x1b[94m";
    const OKGREEN: &'static str = "\x1b[92m";
    const WARNING: &'static str = "\x1b[93m";
    const ERROR: &'static str = "\x1b[91m";
    const NORMAL: &'static str = "\x1b[0m";

    pub fn ok_blue() -> &'static str { Colors::OKBLUE }
    pub fn ok_green() -> &'static str { Colors::OKGREEN }
    pub fn warning() -> &'static str { Colors::WARNING }
    pub fn error() -> &'static str { Colors::ERROR }
    pub fn normal() -> &'static str { Colors::NORMAL }
}
