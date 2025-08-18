# File cấu hình Docker
FROM python:3.11-slim

# Thiết lập thư mục làm việc
WORKDIR /app

# Cài đặt system dependencies
RUN apt-get update && apt-get install -y \
    gcc \
    && rm -rf /var/lib/apt/lists/*

# Sao chép requirements file
COPY requirements.txt .

# Cài đặt Python dependencies
RUN pip install --no-cache-dir -r requirements.txt

# Sao chép toàn bộ source code
COPY . .

# Tạo thư mục cho database
RUN mkdir -p /app/data

# Expose port
EXPOSE 8000

# Command để chạy ứng dụng
CMD ["python", "run.py"]
