CREATE TYPE user_realm AS ENUM ('csh', 'google');

CREATE TABLE users (
    id VARCHAR PRIMARY KEY,
    realm user_realm NOT NULL,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL
);


CREATE TABLE event (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    location VARCHAR NOT NULL,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE NOT NULL,
    creator VARCHAR NOT NULL REFERENCES users(id)
);

CREATE TABLE car (
    id SERIAL PRIMARY KEY,
    event_id INT REFERENCES event(id) ON DELETE CASCADE,
    driver VARCHAR NOT NULL REFERENCES users(id),
    max_capacity INT NOT NULL,
    departure_time TIMESTAMP WITH TIME ZONE NOT NULL,
    return_time TIMESTAMP WITH TIME ZONE NOT NULL,
    comment VARCHAR NOT NULL
);

CREATE TABLE rider (
    car_id INT REFERENCES car(id) ON DELETE CASCADE,
    rider VARCHAR REFERENCES users(id),
    PRIMARY KEY (car_id, rider)
);
