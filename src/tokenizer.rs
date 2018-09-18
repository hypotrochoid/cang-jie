use jieba_rs::Jieba;
use options::TokenizerOption;
use std::sync::Arc;
use stream::CangjieTokenStream;

#[derive(Clone, Debug)]
pub struct CangJieTokenizer {
    pub worker: Arc<Jieba>,
    pub option: TokenizerOption,
}

impl Default for CangJieTokenizer {
    fn default() -> Self {
        CangJieTokenizer {
            worker: Arc::new(Jieba::empty()),
            option: TokenizerOption::Default { hmm: false },
        }
    }
}

impl<'a> ::tantivy::tokenizer::Tokenizer<'a> for CangJieTokenizer {
    type TokenStreamImpl = CangjieTokenStream<'a>;

    fn token_stream(&self, text: &'a str) -> Self::TokenStreamImpl {
        let result = match self.option {
            TokenizerOption::All => self.worker.cut_all(text),
            TokenizerOption::Default { hmm: use_hmm } => self.worker.cut(text, use_hmm),
            TokenizerOption::ForSearch { hmm: use_hmm } => {
                self.worker.cut_for_search(text, use_hmm)
            }
            TokenizerOption::Unicode => {
                text.chars()
                    .fold((0usize, vec![]), |(offset, mut result), the_char| {
                        result.push(&text[offset..offset + the_char.len_utf8()]);
                        (offset + the_char.len_utf8(), result)
                    }).1
            }
        };
        trace!("{:?}->{:?}", text, result);
        CangjieTokenStream::new(result)
    }
}