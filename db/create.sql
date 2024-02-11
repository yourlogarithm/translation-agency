DROP TABLE IF EXISTS translations;
DROP TABLE IF EXISTS translators;
DROP TABLE IF EXISTS documents;
DROP TABLE IF EXISTS clients;
DROP TYPE IF EXISTS lang;
DROP INDEX IF EXISTS client_usr_unique_lower_idx;
DROP INDEX IF EXISTS translator_usr_unique_lower_idx;


CREATE TABLE clients (
    id SERIAL PRIMARY KEY,
    usr VARCHAR(30) NOT NULL,
    pwd VARCHAR(255) NOT NULL,
    fname VARCHAR(255),
    lname VARCHAR(255),
    email VARCHAR(320) NOT NULL,
    v BOOLEAN DEFAULT FALSE,
    t TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT chk_username_length CHECK (char_length(usr) >= 6),
    CONSTRAINT chk_username_format CHECK (usr ~* '^[a-z][a-z0-9_.-]*$' AND usr !~ '[_.-]{2,}' AND usr !~* '^[_.-]|[_.-]$'),
    CONSTRAINT chk_fname_format CHECK (fname ~ '^[a-zA-ZàáâäãåąčćęèéêëėįìíîïłńòóôöõøùúûüųūÿýżźñçčšžÀÁÂÄÃÅĄĆČĖĘÈÉÊËÌÍÎÏĮŁŃÒÓÔÖÕØÙÚÛÜŲŪŸÝŻŹÑßÇŒÆČŠŽ∂ð ,.''-]+$'),
    CONSTRAINT chk_lname_format CHECK (lname ~ '^[a-zA-ZàáâäãåąčćęèéêëėįìíîïłńòóôöõøùúûüųūÿýżźñçčšžÀÁÂÄÃÅĄĆČĖĘÈÉÊËÌÍÎÏĮŁŃÒÓÔÖÕØÙÚÛÜŲŪŸÝŻŹÑßÇŒÆČŠŽ∂ð ,.''-]+$'),
    CONSTRAINT email_format_chk CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'),
    CONSTRAINT email_unique UNIQUE (email)
);
CREATE UNIQUE INDEX client_usr_unique_lower_idx ON clients (LOWER(usr));

CREATE TYPE lang AS ENUM (
    'aa', 'ab', 'ae', 'af', 'ak', 'am', 'an', 'ar', 'as', 'av', 'ay', 'az', 'ba', 'be', 'bg', 'bi', 'bm', 'bn', 'bo', 'br', 'bs', 'ca', 'ce', 'ch', 'co', 'cr', 'cs', 'cu', 'cv', 'cy', 'da', 'de', 'dv', 'dz', 'ee', 'el', 'en', 'eo', 'es', 'et', 'eu', 'fa', 'ff', 'fi', 'fj', 'fo', 'fr', 'fy', 'ga', 'gd', 'gl', 'gn', 'gu', 'gv', 'ha', 'he', 'hi', 'ho', 'hr', 'ht', 'hu', 'hy', 'hz', 'ia', 'id', 'ie', 'ig', 'ii', 'ik', 'io', 'is', 'it', 'iu', 'ja', 'jv', 'ka', 'kg', 'ki', 'kj', 'kk', 'kl', 'km', 'kn', 'ko', 'kr', 'ks', 'ku', 'kv', 'kw', 'ky', 'la', 'lb', 'lg', 'li', 'ln', 'lo', 'lt', 'lu', 'lv', 'mg', 'mh', 'mi', 'mk', 'ml', 'mn', 'mr', 'ms', 'mt', 'my', 'na', 'nb', 'nd', 'ne', 'ng', 'nl', 'nn', 'no', 'nr', 'nv', 'ny', 'oc', 'oj', 'om', 'or', 'os', 'pa', 'pi', 'pl', 'ps', 'pt', 'qu', 'rm', 'rn', 'ro', 'ru', 'rw', 'sa', 'sc', 'sd', 'se', 'sg', 'si', 'sk', 'sl', 'sm', 'sn', 'so', 'sq', 'sr', 'ss', 'st', 'su', 'sv', 'sw', 'ta', 'te', 'tg', 'th', 'ti', 'tk', 'tl', 'tn', 'to', 'tr', 'ts', 'tt', 'tw', 'ty', 'ug', 'uk', 'ur', 'uz', 've', 'vi', 'vo', 'wa', 'wo', 'xh', 'yi', 'yo', 'za', 'zh', 'zu'
);

CREATE TABLE documents (
    id SERIAL PRIMARY KEY,
    cid INT NOT NULL,
    hash BYTEA NOT NULL,
    l lang NOT NULL,
    t TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT hash_length_chk CHECK (octet_length(hash) = 32),
    CONSTRAINT fk_client FOREIGN KEY(cid) REFERENCES clients(id)
);

CREATE TABLE translators (
    id SERIAL PRIMARY KEY,
    usr VARCHAR(30) NOT NULL,
    email VARCHAR(64) NOT NULL,
    pwd VARCHAR(255) NOT NULL,
    langs lang[],
    t TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT chk_username_length CHECK (char_length(usr) >= 6 AND char_length(usr) <= 30),
    CONSTRAINT chk_username_format CHECK (usr ~* '^[a-z][a-z0-9_.-]*$' AND usr !~ '[_.-]{2,}' AND usr !~* '^[_.-]|[_.-]$')
);
CREATE UNIQUE INDEX translator_usr_unique_lower_idx ON translators (LOWER(usr));

CREATE TABLE translations (
    src INT NOT NULL,
    dst INT NOT NULL,
    by INT NOT NULL, 
    t TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_src FOREIGN KEY(src) REFERENCES documents(id),
    CONSTRAINT fk_dst FOREIGN KEY(dst) REFERENCES documents(id),
    CONSTRAINT fk_by FOREIGN KEY(by) REFERENCES translators(id)
);

