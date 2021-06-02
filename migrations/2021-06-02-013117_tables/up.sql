-- Your SQL goes here
CREATE TABLE category(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    deleted_at TIMESTAMP WITHOUT TIME ZONE
);

CREATE TABLE label(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    color_hex VARCHAR,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    deleted_at TIMESTAMP WITHOUT TIME ZONE
);

CREATE TABLE data_status(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    deleted_at TIMESTAMP WITHOUT TIME ZONE
);

CREATE TABLE list(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    deleted_at TIMESTAMP WITHOUT TIME ZONE
);

CREATE TABLE todo(
    id SERIAL PRIMARY KEY,
    title VARCHAR UNIQUE NOT NULL,
    description VARCHAR NOT NULL,
    start_at TIMESTAMP WITH TIME ZONE NOT NULL,
    end_at TIMESTAMP WITH TIME ZONE NOT NULL,
    id_category INT,
    id_label INT,
    id_list INT,
    id_data_status INT,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    FOREIGN KEY(id_category) REFERENCES category(id),
    FOREIGN KEY(id_label) REFERENCES label(id),
    FOREIGN KEY(id_list) REFERENCES list(id),
    FOREIGN KEY(id_data_status) REFERENCES data_status(id)
);

INSERT INTO public.data_status(name) VALUES ('done');