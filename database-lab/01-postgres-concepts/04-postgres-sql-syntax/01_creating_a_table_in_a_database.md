```sql
CREATE TABLE users (
   id SERIAL PRIMARY KEY,
   username TEXT NOT NULL,
   email TEXT UNIQUE NOT NULL,
   created_at TIMESTAMP DEFAULT NOW()
);
```

How to insert some data:
```sql
INSERT INTO users (username, email)
VALUES
('dezly', 'dezly@example.com'),
('alex', 'alex@example.com'),
('mira', 'mira@example.com');
```
