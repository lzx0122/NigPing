# Supabase Database Setup

請在您的 Supabase Dashboard 的 SQL Editor 中執行以下 SQL 腳本來建立資料表：

## 1. 建立 servers 資料表

```sql
CREATE TABLE servers (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  ip VARCHAR(45) NOT NULL UNIQUE,
  region VARCHAR(100) NOT NULL,
  added_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 建立索引以提升查詢效能
CREATE INDEX idx_servers_region ON servers(region);
CREATE INDEX idx_servers_added_at ON servers(added_at DESC);
```

## 2. 設定 Row Level Security (RLS)

```sql
-- 啟用 RLS
ALTER TABLE servers ENABLE ROW LEVEL SECURITY;

-- 允許所有人讀取（因為是公開的伺服器列表）
CREATE POLICY "Allow public read access" ON servers
  FOR SELECT USING (true);

-- 允許所有人新增、更新、刪除（開發階段）
CREATE POLICY "Allow public insert access" ON servers
  FOR INSERT WITH CHECK (true);

CREATE POLICY "Allow public update access" ON servers
  FOR UPDATE USING (true);

CREATE POLICY "Allow public delete access" ON servers
  FOR DELETE USING (true);
```

## 3. 驗證設定

執行以下查詢確認資料表已建立：

```sql
SELECT * FROM servers;
```

## 注意事項

⚠️ **生產環境安全性**：目前的 RLS 政策允許任何人修改資料。建議在生產環境中：

- 實作 Supabase 認證
- 限制只有管理員可以修改伺服器列表
- 或使用 Service Role Key 在後端進行操作

## 完成後

執行完 SQL 腳本後，請告訴我，我會啟動 backend 進行測試。
