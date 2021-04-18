-- Your SQL goes here
CREATE TABLE items (
    item_id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    price INT NOT NULL,
    stock INT NOT NULL,
    owner VARCHAR(255) NOT NULL,
    posted_timestamp DATETIME NOT NULL
)
