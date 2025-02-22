# 📘 **File Microservice API Guide**  
> **A lightweight file upload microservice built with Rust & Axum.**  

📌 **Base URL:** `http://localhost:3000`  

---

## 📌 **1. Health Check Endpoint**  

### **🔹 Check API Health**
**Request:**  
```bash
curl http://localhost:3000/health
```

**Expected Response:**  
```json
{
    "status": "ok",
    "timestamp": 1707350400,
    "version": "0.1.0"
}
```

✅ **Purpose:**  
- Ensures that the API is running and accessible.  
- Includes a version identifier for tracking deployments.  

---

## 📌 **2. Metrics Endpoint**  

### **🔹 Get API Metrics**
**Request:**  
```bash
curl http://localhost:3000/metrics
```

**Expected Response:**  
```json
{
    "uptime": 3600,
    "memory_usage": 1024,
    "cpu_usage": 0.5,
    "total_requests": 100
}
```

✅ **Purpose:**  
- Provides system insights like uptime, memory usage, and request count.  
- Useful for **monitoring API health and performance**.  

---

## 📌 **3. File Upload Endpoint**  

### **🔹 Upload a File**
**Request:**  
```bash
curl -X POST \
  -F "upfile=@/path/to/your/file.txt" \
  http://localhost:3000/upload
```

**Expected Response:**  
```json
{
    "name": "file.txt",
    "type": "text/plain",
    "size": 1234
}
```

### **🔹 Upload an Image**
```bash
curl -X POST \
  -F "upfile=@/path/to/your/image.jpg" \
  http://localhost:3000/upload
```

**Expected Response:**  
```json
{
    "name": "image.jpg",
    "type": "image/jpeg",
    "size": 5678
}
```

✅ **Purpose:**  
- Accepts files via **multipart form uploads**.  
- Returns file metadata, including **name, type, and size**.  

---

## 📌 **4. File Download Endpoint**  

### **🔹 Download a File**
**Request:**  
```bash
curl -X GET "http://localhost:3000/file/download?filename=file.txt" -o downloaded_file.txt
```

**Expected Response (File Stream)**  
- The file is downloaded to your system.  

✅ **Purpose:**  
- Allows users to **retrieve previously uploaded files**.  
- Ensures secure file handling and controlled access.  

---

## 📌 **5. Rate Limiting Endpoint**  

### **🔹 Check Rate Limits**
**Request:**  
```bash
curl -I http://localhost:3000/upload
```

**Response Headers:**  
```
X-RateLimit-Limit: 5
X-RateLimit-Remaining: 3
X-RateLimit-Reset: 60
```

✅ **Purpose:**  
- Prevents **API abuse** using a token bucket algorithm.  
- Returns **limit, remaining requests, and reset time**.  

---

## 📌 **6. Error Handling & Edge Cases**  

### **🔹 Upload Without a File**
```bash
curl -X POST http://localhost:3000/upload
```

**Expected Response:**  
```json
{
    "error": "No file was uploaded"
}
```

### **🔹 Upload File Exceeding Size Limit (>5MB)**
```bash
curl -X POST \
  -F "upfile=@/path/to/large/file.zip" \
  http://localhost:3000/upload
```

**Expected Response:**  
```json
{
    "error": "File too large"
}
```

### **🔹 Download Non-Existent File**
```bash
curl -X GET "http://localhost:3000/file/download?filename=notfound.txt"
```

**Expected Response:**  
```json
{
    "error": "File not found"
}
```

✅ **Purpose:**  
- Ensures **proper validation and error handling** for incorrect requests.  

---

## 📌 **7. PowerShell Equivalents (For Windows Users)**  

### **Health Check**
```powershell
Invoke-RestMethod -Uri "http://localhost:3000/health" -Method Get
```

### **File Upload**
```powershell
$filePath = "C:\path\to\your\file.txt"
$form = @{
    upfile = Get-Item -Path $filePath
}
Invoke-RestMethod -Uri "http://localhost:3000/upload" -Method Post -Form $form
```

---

## 🛠 **Testing & Debugging**  

| **Issue** | **Possible Fixes** |  
|-----------|------------------|  
| **"Connection refused"** | Ensure server is running (`cargo run`) & check port (3000) |  
| **"File not found"** | Verify file path & filename before upload/download |  
| **"413 Payload Too Large"** | File exceeds the **5MB upload limit** |  

---

# 🚀 **Final Notes**  
✅ **This API Guide provides all necessary endpoints, expected responses, and troubleshooting steps.**  
✅ **Includes CLI & PowerShell commands for testing.**  
✅ **Ensures secure and scalable file handling with Rust & Axum.**  

🔹 **Follow these steps to confidently test and use the File Microservice API!** 🚀  
