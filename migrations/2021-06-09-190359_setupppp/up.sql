-- Your SQL goes here
INSERT INTO yearending(id) VALUES (1);

CREATE OR REPLACE VIEW AssignedBooks AS
    SELECT
        c.id as class_id,
        c.name as class_name,
        b.id  as book_id,
        b.title as book_title
    FROM
         booksasinged as bkas
            join books b on bkas.book_id = b.id
            join classes c on bkas.class_id = c.id;


CREATE OR REPLACE VIEW ChildBookView AS
    SELECT
        b.id  as book_id,
        b.title as book_title,
        c.id as class_id,
        c.name as class_name,
        t.id IS NOT NULL as is_taken,
        c3.id as child_id
    FROM
         books as b
            join booksasinged bas on b.id=bas.book_id
            join classes c on bas.class_id = c.id
            join childandclass c2 on c.id = c2.class_id
            join child c3 on c2.chid_id = c3.id
            left join takenbooks t on c3.id = t.child_id and b.id=t.book_id;
