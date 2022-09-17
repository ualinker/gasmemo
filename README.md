> ⚠️ This is **Early stage / work in progress** project, do not expect it to do something 
> meaningful just yet.

> 🇺🇦 It has been developed with i18n in mind, but for now є тільки українська. Other languages 
> can be added to `i18n` directory (see `i18n/uk/gasmemo.ftl` as an example).

# Run

1. Install Diesel for Cargo:
    ```bash
    cargo install diesel
    ```
   
2. Add `.env` file to the project root:
    ```dotenv
    DATABASE_URL=gasmemo.sq3
    ```
   
3. Create database and its structure:
    ```bash
    diesel migration run
    ```

4. Now you can run the program:
    ```bash
    cargo run
    ```