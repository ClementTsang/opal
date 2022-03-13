pragma journal_mode = delete;
pragma page_size = 1024;
DELETE FROM english WHERE ROWID NOT IN (SELECT MIN(ROWID) FROM english GROUP BY word, phonemes);
DROP INDEX IF EXISTS word_index;
DROP INDEX IF EXISTS phonemes_index;
CREATE INDEX word_index ON english(word);
CREATE INDEX phonemes_index ON english(phonemes);
vacuum;