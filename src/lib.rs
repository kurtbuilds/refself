
pub struct Source {
    pub content: String,
    tokens: Vec<&'static str>,
}

//! As long as the struct never gives out static references,
//! and references always lives as long as `self`, then the transmute, and this struct,
//! is perfectly safe.
impl Source {
    pub fn new(content: String) -> Self {
        let tokens = content.split_whitespace()
            .map(|t| unsafe { std::mem::transmute(t) })
            .collect();
        Self { content, tokens }
    }

    pub fn tokens_iter(&self) -> impl Iterator<Item = &str> {
        self.tokens.iter().map(|&s| s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = Source::new("hello world".to_string());
        let t = s.tokens_iter().collect::<Vec<_>>();
        drop(s);
        println!("{:?}", t);
    }
}
