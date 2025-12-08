use crate::trans::MessageFormatter;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {}

pub trait MessagePriv<'a>: Sized {
    fn parse(parser: &mut MessageParser<'a>) -> Result<Self, ParseError>;
    fn format(&self, formatter: &mut MessageFormatter<'_>);
}

pub struct MessageParser<'a> {
    buffer: &'a [u32],
}

impl MessageFormatter<'_> {
    #[inline(always)]
    pub fn words<const N: usize>(&mut self, words: [u32; N]) {
        self.buffer[self.words_written..][..words.len()].copy_from_slice(&words);
        self.words_written += words.len();
    }

    #[inline(always)]
    pub fn array(&mut self, data: &[u8]) {
        self.words([data.len() as u32]);
        uapi::as_bytes_mut(&mut self.buffer[self.words_written..])[..data.len()]
            .copy_from_slice(data);
        self.words_written += (data.len() + 3) / 4;
    }

    #[inline(always)]
    pub fn string(&mut self, data: &str) {
        let len = data.len() + 1;
        self.words([len as u32]);
        let tail = uapi::as_bytes_mut(&mut self.buffer[self.words_written..]);
        tail[..data.len()].copy_from_slice(data.as_bytes());
        tail[data.len()] = 0;
        self.words_written += (len + 3) / 4;
    }
}
