CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE,
    email VARCHAR(100) UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    google_id VARCHAR(50),
    google_access_token VARCHAR(100),
    google_refresh_token VARCHAR(100),
    google_expires_in TIMESTAMP,
    github_id VARCHAR(50),
    github_access_token VARCHAR(100),
    github_refresh_token VARCHAR(100),
    github_expires_in TIMESTAMP,
    apple_id VARCHAR(50),
    apple_identity_token VARCHAR(100),
    apple_access_token VARCHAR(100),
    apple_refresh_token VARCHAR(100),
    apple_expires_in TIMESTAMP
);

CREATE INDEX idx_username ON users(username);
CREATE INDEX idx_email ON users(email);
