-- Your SQL goes here
CREATE TABLE products (
	id SERIAL PRIMARY KEY,
	external_id UUID DEFAULT uuid_generate_v4() UNIQUE NOT NULL,
	SKU VARCHAR(8) UNIQUE NOT NULL,
	name VARCHAR(64) NOT NULL,
	description VARCHAR(1024) NOT NULL,
	price BIGINT NOT NULL
)