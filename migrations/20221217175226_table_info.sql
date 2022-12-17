create table  IF NOT EXISTS dshbrd_info (
            key varchar(255),
            value varchar(255),
            updated_at timestamp default current_timestamp,
            created_at timestamp default current_timestamp,
            UNIQUE (key, value)
);