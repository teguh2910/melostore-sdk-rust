# Melostore API Service Documentation

Backend service tunggal (`melostore-server`) berbasis Rust (Axum) yang beroperasi sebagai **Direct / Stateless API Gateway** untuk query produk, pengecekan nickname, dan pemesanan layanan ke Melo H2H API (v1.4.0) secara real-time.

---

## 1. Konfigurasi Environment

File `.env` di direktori `apps/server/.env` atau environment sistem / docker:

```dotenv
BIND_ADDR=0.0.0.0:8080
MELOSTORE_API_KEY=your_api_key
MELOSTORE_API_SECRET=your_secret_key
MELOSTORE_WEBHOOK_SECRET=your_webhook_secret
MELOSTORE_BASE_URL=https://api.melostore.id
MELOSTORE_SANDBOX=true
CORS_ORIGIN=*
```

---

## 2. Menjalankan Service

### Menggunakan Cargo
```bash
cargo run -p melostore-server
```

### Menggunakan Docker Compose
```bash
docker compose up -d --build
```
Service akan berjalan di port `8080` (dimapping ke `127.0.0.1:8088` pada host).

---

## 3. Daftar Endpoint

Prefix utama: `/api/v1`

### 3.1 Health Check & Balance
Mengecek status koneksi dan saldo H2H.
- **URL**: `GET /api/v1/health` (atau `GET /health`)
- **Contoh Response**:
  ```json
  {
    "status": "ok",
    "provider": {
      "connected": true,
      "balance": 1500000,
      "is_sandbox": true
    }
  }
  ```

---

### 3.2 Katalog / Produk
- **URL**: `GET /api/v1/products`
- **Query Parameters**:
  - `limit`: Jumlah data per halaman (default: 500).
  - `cursor`: Kursor halaman berikutnya.
  - `brand_id`: ID Brand (opsional).
  - `server`: Server code (opsional).
- **URL Kategori / Brand**: `GET /api/v1/products/{brand_slug}`

---

### 3.3 Validasi Akun (Check Nickname)
- **URL**: `POST /api/v1/validate-account`
- **Request Body**:
  ```json
  {
    "game_code": "mobile-legends",
    "customer_target": "12345678",
    "customer_target_zone": "1234"
  }
  ```

---

### 3.4 Transaksi (Orders)
- **Buat Pesanan**: `POST /api/v1/orders`
  - **Request Body**:
    ```json
    {
      "sku_code": "ML86",
      "customer_target": "12345678",
      "customer_target_zone": "1234",
      "buyer_trx_id": "ODGJ-ORDER-1",
      "sandbox_mode": true
    }
    ```
- **Cek Status Pesanan**: `GET /api/v1/orders/{id}`
