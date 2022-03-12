pragma journal_mode = delete;
pragma page_size = 1024;
DROP INDEX IF EXISTS word_index;
DROP INDEX IF EXISTS phoneme_index;
CREATE INDEX word_index ON english(word);
CREATE INDEX phonemes_index ON english(phonemes);
vacuum;