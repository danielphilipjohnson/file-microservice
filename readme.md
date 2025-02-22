# 🚀 **File Microservice API**  
> A **secure** and **scalable** file upload microservice built with **Rust and Axum**.  

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)  
![Axum](https://img.shields.io/badge/Axum-000000?style=for-the-badge&logo=rust&logoColor=white)  
![Tokio](https://img.shields.io/badge/Tokio-0052CC?style=for-the-badge&logo=tokio&logoColor=white)  

---

## 🌟 **Features**
✔ **File Upload** – Supports multipart form uploads  
✔ **Rate Limiting** – Uses **Tower-based** rate limiting  
✔ **CORS Support** – Configurable cross-origin policies  
✔ **Health Checks & Metrics** – Tracks API uptime and status  
✔ **Secure JSON Responses** – Returns structured metadata  
✔ **Scalable Architecture** – Built with **Axum**, **Tokio**, and **Tower**  

---

## 🛠 **Tech Stack**
| Technology | Purpose |
|------------|---------|
| **Rust** 🦀 | High-performance systems programming |
| **Axum** ⚡ | Async web framework |
| **Tokio** 🔄 | Async runtime for handling multiple requests |
| **Tower** 📡 | Middleware for rate limiting & request tracing |
| **Serde** 📦 | JSON serialization/deserialization |
| **tracing** 🔍 | Structured logging & monitoring |

---

## 📂 **Project Structure**
```
/file_microservice
├── src/
│   ├── main.rs            # Application entry point
│   ├── handlers/          # Request handlers
│       ├── file.rs        # File upload logic
│       ├── health.rs      # Health check & metrics
│   ├── middleware/        # Middleware (rate limiting)
│   ├── models/            # Data structures
│   ├── config/            # Application configuration
├── tests/                 # Integration tests
├── Cargo.toml             # Dependencies
└── README.md              # Documentation
```

---

## 🚀 **Getting Started**

### **1️⃣ Install Prerequisites**
- **Rust (via Rustup)**  
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

### **2️⃣ Clone the Repository**
```sh
git clone https://github.com/yourusername/file_microservice.git
cd file_microservice
```

---

### **3️⃣ Setup Environment Variables**
Create a `.env` file:
```ini
PORT=3000
MAX_FILE_SIZE=5242880  # 5MB
```

---

### **4️⃣ Build and Run the Server**
```sh
cargo build
cargo run
```
Your API will start at **`http://localhost:3000`** 🎉

---

## 📡 **API Endpoints**

### **1️⃣ Upload a File (`POST /upload`)**
#### **Example Request**
```sh
curl -X POST http://localhost:3000/upload \
     -F "upfile=@example.txt"
```

#### **Example Response**
```json
{
  "name": "example.txt",
  "type": "text/plain",
  "size": 1234
}
```

✅ **Validates file type and size**  
✅ **Returns metadata about the uploaded file**  

---

### **2️⃣ Health Check (`GET /health`)**
#### **Example Request**
```sh
curl http://localhost:3000/health
```

#### **Example Response**
```json
{
  "status": "healthy",
  "timestamp": 1700000000
}
```

✅ **Useful for monitoring API availability**  

---

## 🔐 **Rate Limiting**
- **Limits API requests per client** using **Tower middleware**
- **Headers for Rate Limits:**  
  - `X-RateLimit-Limit`: Max requests allowed  
  - `X-RateLimit-Remaining`: Remaining requests in window  
  - `X-RateLimit-Reset`: Time (in seconds) until reset  

---

## 🧪 **Testing**
```sh
cargo test
```

Run a specific test:
```sh
cargo test test_file_upload
```

---

## 🛠 **Development Commands**

```sh
# Build the project
cargo build

# Run the application
cargo run

# Run tests
cargo test

# Check for compilation errors
cargo check

# Update dependencies
cargo update

# Clean build artifacts
cargo clean
```

### **Development Tips**
- Use **cargo watch** for auto-reloading:
  ```sh
  cargo install cargo-watch
  cargo watch -x run
  ```
- Format code:
  ```sh
  cargo fmt
  ```
- Check for warnings:
  ```sh
  cargo clippy
  ```

---

## 🤝 **Contributing**
1. Fork the repository  
2. Create a feature branch  
3. Commit your changes  
4. Push to the branch  
5. Create a Pull Request  

---

## 🔗 **Contact**
- **Blog**: [dev.to/danielphilipjohnson](https://dev.to/danielphilipjohnson)  
- **GitHub**: [github.com/yourusername](https://github.com/yourusername)  

---

## 🚀 **Let's Build Together!**
If you find this project useful, please **star** ⭐ the repo and contribute! 🎉  

