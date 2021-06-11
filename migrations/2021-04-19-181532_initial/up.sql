-- Your SQL goes here
CREATE TABLE Child (
    id              BIGSERIAL   PRIMARY KEY,
    name            VARCHAR     NOT NULL,
    last_name       VARCHAR     NOT NULL,
    very_last_name  VARCHAR     NOT NULL,
    login           VARCHAR     NOT NULL,
    pass_hash       VARCHAR     NOT NULL,
    register_data   TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE Teacher (
    id              BIGSERIAL   PRIMARY KEY,
    name            VARCHAR     NOT NULL,
    last_name       VARCHAR     NOT NULL,
    very_last_name  VARCHAR     NOT NULL,
    login           VARCHAR     NOT NULL,
    pass_hash       VARCHAR     NOT NULL,
    register_data   TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE Books (
    id              BIGSERIAL   PRIMARY KEY,
    title           VARCHAR     NOT NULL
);

CREATE TABLE Classes (
    id              BIGSERIAL PRIMARY KEY,
    name            VARCHAR     NOT NULL
);

CREATE TABLE ChildAndClass (
    id              BIGSERIAL   PRIMARY KEY,
    chid_id         BIGINT      NOT NULL REFERENCES Child(id),
    class_id        BIGINT      NOT NULL REFERENCES Classes(id)
);

CREATE TABLE YearEnding (
    id              BIGSERIAL PRIMARY KEY,
    is_ending       BOOLEAN     NOT NULL DEFAULT FALSE
);

CREATE TABLE BooksAsinged (
    id              BIGSERIAL   PRIMARY KEY,
    book_id         BIGINT      NOT NULL REFERENCES Books(id),
    class_id        BIGINT      NOT NULL REFERENCES Classes(id)
);

CREATE TABLE TakenBooks (
    id              BIGSERIAL PRIMARY KEY,
    book_id         BIGINT      NOT NULL REFERENCES Books(id),
    child_id        BIGINT      NOT NULL REFERENCES Child(id)
);
