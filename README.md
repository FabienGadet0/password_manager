# password_manager

- **Functionality:** Store passwords securely using encryption and manage them with features like adding, deleting, and viewing entries.
- **Concepts:**
  - Data Structures: Choose appropriate data structures (structs, enums) to represent password entries efficiently.
  - Error Handling: Handle errors like invalid input or encryption failures using the `Result` type.
  - Secure Password Storage: Employ hashing algorithms (e.g., Argon2) from the `argon2` crate to securely store passwords without saving them in plain text.
  - Persistence: Utilize a persistent storage mechanism (e.g., JSON files using the `serde` crate) to save and load password data across program runs.
  - User Interaction: Prompt the user for input through the `std::io` module, allowing them to add, delete, or view password entries.
  - Encryption: ring
