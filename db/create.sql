DROP TABLE IF EXISTS translations;
DROP TABLE IF EXISTS translators;
DROP TABLE IF EXISTS documents;
DROP TABLE IF EXISTS users;
DROP TYPE IF EXISTS lang;
DROP INDEX IF EXISTS user_username_unique_lower_idx;
DROP INDEX IF EXISTS user_email_unique_lower_idx;
DROP INDEX IF EXISTS user_version_code_idx;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    username VARCHAR(30) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    email VARCHAR(320) NOT NULL UNIQUE,
    verification_code VARCHAR(255) NOT NULL,
    verified BOOLEAN DEFAULT FALSE NOT NULL,
    creation_timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    update_timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW()
    CONSTRAINT chk_username_length CHECK (char_length(username) >= 6),
    CONSTRAINT chk_username_format CHECK (username ~* '^[a-z][a-z0-9_.-]*$' AND username !~ '[_.-]{2,}' AND username !~* '^[_.-]|[_.-]$'),
    CONSTRAINT chk_fname_format CHECK (first_name ~ '^[a-zA-ZàáâäãåąčćęèéêëėįìíîïłńòóôöõøùúûüųūÿýżźñçčšžÀÁÂÄÃÅĄĆČĖĘÈÉÊËÌÍÎÏĮŁŃÒÓÔÖÕØÙÚÛÜŲŪŸÝŻŹÑßÇŒÆČŠŽ∂ð ,.''-]+$'),
    CONSTRAINT chk_lname_format CHECK (last_name ~ '^[a-zA-ZàáâäãåąčćęèéêëėįìíîïłńòóôöõøùúûüųūÿýżźñçčšžÀÁÂÄÃÅĄĆČĖĘÈÉÊËÌÍÎÏĮŁŃÒÓÔÖÕØÙÚÛÜŲŪŸÝŻŹÑßÇŒÆČŠŽ∂ð ,.''-]+$'),
    CONSTRAINT email_format_chk CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$')
);
CREATE UNIQUE INDEX user_username_unique_lower_idx ON users (LOWER(username));
CREATE UNIQUE INDEX user_email_unique_lower_idx ON users (LOWER(email));
CREATE INDEX user_version_code_idx ON users(verification_code);

CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.update_timestamp = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_user_timestamp BEFORE UPDATE
ON users FOR EACH ROW EXECUTE FUNCTION update_timestamp();


CREATE TYPE lang AS ENUM (
    'aa', 'ab', 'ae', 'af', 'ak', 'am', 'an', 'ar', 'as', 'av', 'ay', 'az', 'ba', 'be', 'bg', 'bi', 'bm', 'bn', 'bo', 'br', 'bs', 'ca', 'ce', 'ch', 'co', 'cr', 'cs', 'cu', 'cv', 'cy', 'da', 'de', 'dv', 'dz', 'ee', 'el', 'en', 'eo', 'es', 'et', 'eu', 'fa', 'ff', 'fi', 'fj', 'fo', 'fr', 'fy', 'ga', 'gd', 'gl', 'gn', 'gu', 'gv', 'ha', 'he', 'hi', 'ho', 'hr', 'ht', 'hu', 'hy', 'hz', 'ia', 'id', 'ie', 'ig', 'ii', 'ik', 'io', 'is', 'it', 'iu', 'ja', 'jv', 'ka', 'kg', 'ki', 'kj', 'kk', 'kl', 'km', 'kn', 'ko', 'kr', 'ks', 'ku', 'kv', 'kw', 'ky', 'la', 'lb', 'lg', 'li', 'ln', 'lo', 'lt', 'lu', 'lv', 'mg', 'mh', 'mi', 'mk', 'ml', 'mn', 'mr', 'ms', 'mt', 'my', 'na', 'nb', 'nd', 'ne', 'ng', 'nl', 'nn', 'no', 'nr', 'nv', 'ny', 'oc', 'oj', 'om', 'or', 'os', 'pa', 'pi', 'pl', 'ps', 'pt', 'qu', 'rm', 'rn', 'ro', 'ru', 'rw', 'sa', 'sc', 'sd', 'se', 'sg', 'si', 'sk', 'sl', 'sm', 'sn', 'so', 'sq', 'sr', 'ss', 'st', 'su', 'sv', 'sw', 'ta', 'te', 'tg', 'th', 'ti', 'tk', 'tl', 'tn', 'to', 'tr', 'ts', 'tt', 'tw', 'ty', 'ug', 'uk', 'ur', 'uz', 've', 'vi', 'vo', 'wa', 'wo', 'xh', 'yi', 'yo', 'za', 'zh', 'zu'
);

CREATE TABLE documents (
    id SERIAL PRIMARY KEY,
    uid UUID NOT NULL,
    hash BYTEA NOT NULL,
    l lang NOT NULL,
    t TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT hash_length_chk CHECK (octet_length(hash) = 32),
    CONSTRAINT fk_user FOREIGN KEY(uid) REFERENCES users(id)
);

CREATE TABLE translators (
    id SERIAL PRIMARY KEY,
    uid UUID NOT NULL UNIQUE,
    langs lang[],
    CONSTRAINT fk_uid FOREIGN KEY(uid) REFERENCES users(id)
);

CREATE TABLE translations (
    src INT NOT NULL,
    dst INT NOT NULL,
    by INT NOT NULL, 
    t TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_src FOREIGN KEY(src) REFERENCES documents(id),
    CONSTRAINT fk_dst FOREIGN KEY(dst) REFERENCES documents(id),
    CONSTRAINT fk_by FOREIGN KEY(by) REFERENCES translators(id)
);

