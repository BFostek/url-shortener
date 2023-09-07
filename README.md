# URL Shortener - DevGym Project

Welcome to the URL Shortener project, a coding exercise from the DevGym platform. This project is a part of my journey to immerse myself into the Rust programming language and gain proficiency in it. As a newcomer to Rust, this project serves as a practical playground to hone my skills and understand the nuances of Rust development.

## Project Overview

This URL Shortener is a web application developed using Rust with the help of the Axum framework. It features a simple REST API that allows users to shorten URLs, making them easier to share and manage. Through this project, I aim to explore various aspects of Rust, including memory safety, data concurrency, and other features that make Rust a unique and powerful programming language.

## Getting Started

To get started with this project, follow the instructions below:

1. **Prerequisites**: Ensure you have the following installed:
   - Rust (latest version)
   - Docker & Docker-compose

2. **Setup**:
   - Clone the repository to your local machine.
   - Navigate to the project directory in your terminal.
   
3. **Environment Variables**: 
   - Create a `.env` file in the project root and add your database credentials:
     ```
     POSTGRES_USER=your_postgres_user
     POSTGRES_PASSWORD=your_postgres_password
     ```

4. **Database Setup**: 
   - Set up the PostgreSQL database using Docker:
     ```
     docker-compose up -d db
     ```

5. **Run the Application**:
   - Build and run the Rust application:
     ```
     cargo run
     ```

6. **API Endpoints**:
   - `GET /:code`: Retrieve the original URL from the shortened code.
   


## Conclusion

This project is a learning endeavor from DevGym to practice and improve my Rust programming skills. Feedback and contributions are welcome as I continue on this journey of learning Rust.

Thank you for checking out this project, and I hope it serves as a beneficial resource in your Rust learning journey as well.

Happy Coding!

