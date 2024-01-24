CREATE TABLE
  users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    hash VARCHAR(255) NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW ()
  );

-- initial password to change: dev_only_pwd
INSERT INTO
  users (id, email, hash, is_admin)
VALUES
  (
    'a74f9b43-8a49-4d97-8270-9879d37c600d',
    'root@test.com',
    '$argon2id$v=19$m=19456,t=2,p=1$AreaBODoNb1PVkrVYG47YQ$RqDZNg9uwWgRDFoeJkIED5RarIBPky6a0mvjr8sqVfs',
    true
  );
