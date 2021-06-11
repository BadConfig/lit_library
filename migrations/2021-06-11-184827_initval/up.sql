-- Your SQL goes here

CREATE OR REPLACE VIEW AssignedBooks AS
    SELECT
        c.id as class_id,
        c.name as class_name,
        b.id  as book_id,
        b.title as book_title,
        b2.id IS NOT NULL as is_asigned
    FROM
         classes as c
            full join books as b on true
            left join booksasinged b2 on b.id = b2.book_id and c.id = b2.class_id;


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


INSERT INTO classes (name) VALUES
('9.1'),
('9.2'),
('9.3'),
('9.4'),
('9.5'),
('9.6'),
('10.1'),
('10.2'),
('10.3'),
('10.4'),
('10.5'),
('10.6'),
('11.1'),
('11.2'),
('11.3'),
('11.4'),
('11.5'),
('11.6');

INSERT INTO auth(login, auth_type, roles) VALUES ('admin','plain','{admin,teacher}');

INSERT INTO teacher(id, name, last_name, very_last_name, login, pass_hash) VALUES
(currval(pg_get_serial_sequence('auth','id')),'admin',
    'admin','admin','admin','jGl25bVBBBW96Qi9Te4V37Fnqchz/Eu4qB9vKrRIqRg=');

