CREATE TABLE IF NOT EXISTS "user" (
    id UUID PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL
);

INSERT INTO "user" (id, email, password)
VALUES ('aa7f82d0-ad10-4d65-8bfa-1fe49767c6cc', 'admin@portfolio-manager.com', '$2b$12$CDWE5LbAds9gIAMuX5hDZuQbntXx0BByY3Wh.AwbfuvZp5RUvqj4S');