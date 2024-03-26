
-- Create a table named "Customers" with columns for ID, name, email, and phone number

CREATE TABLE Customers (
  id INT  ROWID,  -- Auto-incrementing integer for unique ID
  name VARCHAR(255) NOT NULL,         -- Customer name (string up to 255 characters)
  email VARCHAR(255) UNIQUE,         -- Customer email (unique)
  phone_number VARCHAR(20)           -- Customer phone number
);

-- Create a table named "Orders" with columns for order ID, customer ID (foreign key), and order date

CREATE TABLE Orders (
  order_id INT  ROWID,
  customer_id INT NOT NULL,
  order_date DATE NOT NULL,
  FOREIGN KEY (customer_id) REFERENCES Customers(id)  -- Foreign key referencing Customers.id
);

-- Create a table named "Products" with columns for product ID, name, price, and description

CREATE TABLE Products (
  product_id INT  ROWID,
  name VARCHAR(255) NOT NULL,
  price DECIMAL(10,2) NOT NULL,        -- Price with two decimal places
  description TEXT                     -- Product description (larger text)
);
