# cang-jie([仓颉](https://en.wikipedia.org/wiki/Cangjie))

[![Build Status](https://travis-ci.org/DCjanus/cang-jie.svg?branch=master)](https://travis-ci.org/DCjanus/cang-jie)
[![dependency status](https://deps.rs/repo/github/dcjanus/cang-jie/status.svg)](https://deps.rs/repo/github/dcjanus/cang-jie)
![GitHub](https://img.shields.io/github/license/mashape/apistatus.svg)

A Chinese tokenizer for [tantivy](https://github.com/tantivy-search/tantivy), based on [jieba-rs](https://github.com/messense/jieba-rs).

As of now, only support UTF-8.

## Example

```rust
    let mut schema_builder = SchemaBuilder::default();
    let text_indexing = TextFieldIndexing::default()
        .set_tokenizer("cang_jie") // Set custom tokenizer
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let text_options = TextOptions::default()
        .set_indexing_options(text_indexing)
        .set_stored();
    // ... Some code   
     let index = Index::create(RAMDirectory::create(), schema.clone())?;
     let tokenizer = CangJieTokenizer {
                        worker: Arc::new(Jieba::empty()), // empty dictionary
                        option: TokenizerOption::Unicode,
                     };
     index.tokenizers().register("cang_jie", tokenizer); 
    // ... Some code
```

[Full example](./tests/unicode_split.rs)