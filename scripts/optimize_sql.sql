pragma journal_mode = delete;
pragma page_size = 1024;

DROP INDEX IF EXISTS word_index;
DROP INDEX IF EXISTS phonemes_index;
VACUUM;

DELETE FROM english WHERE ROWID NOT IN (SELECT MIN(ROWID) FROM english GROUP BY word, phonemes);

CREATE INDEX word_index ON english(word COLLATE NOCASE);
CREATE INDEX phonemes_index ON english(phonemes COLLATE NOCASE);
VACUUM;