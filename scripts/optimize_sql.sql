DELETE FROM english WHERE ROWID NOT IN (SELECT MIN(ROWID) FROM english GROUP BY word, phonemes);
DROP INDEX IF EXISTS word_index;
DROP INDEX IF EXISTS phonemes_index;
VACUUM;

/*  This is dangerous! We want a quick way to add COLLATE NOCASE to each column, but modifying columns is not
    permitted in SQLite. So we have this hack. Whee~
*/
PRAGMA writable_schema = 1;
UPDATE SQLITE_MASTER SET SQL = 'CREATE TABLE IF NOT EXISTS "english" (
    "word" TEXT COLLATE NOCASE,
    "phonemes" TEXT COLLATE NOCASE
);
';
PRAGMA writable_schema = 0;
/*  End of dangerous section.  */
VACUUM;

pragma journal_mode = delete;
pragma page_size = 1024;

CREATE INDEX word_index ON english(word COLLATE NOCASE);
CREATE INDEX phonemes_index ON english(phonemes COLLATE NOCASE);
VACUUM;