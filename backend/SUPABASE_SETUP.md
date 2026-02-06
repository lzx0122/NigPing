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

## 4. 建立 game_ip_ranges 資料表 (新功能)

```sql
CREATE TABLE game_ip_ranges (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  game_id VARCHAR(50) NOT NULL,
  ip_range VARCHAR(50) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  UNIQUE(game_id, ip_range)
);

-- 建立索引
CREATE INDEX idx_game_ip_ranges_game_id ON game_ip_ranges(game_id);

-- 啟用 RLS
ALTER TABLE game_ip_ranges ENABLE ROW LEVEL SECURITY;

-- 允許所有人讀寫 (開發階段)
CREATE POLICY "Allow public all access on game_ip_ranges" ON game_ip_ranges
  FOR ALL USING (true) WITH CHECK (true);

-- 預先寫入 PUBG 資料 (可選)
-- 注意：這裡請自行替換為實際需要的初始資料，或透過 API 寫入
-- INSERT INTO game_ip_ranges (game_id, ip_range) VALUES
-- ('pubg', '13.124.0.0/16'),
-- ('pubg', '13.125.0.0/16'),
-- ...
```

## 完成後

執行完 SQL 腳本後，請告訴我，我會啟動 backend 進行測試。

## 5. �إ� users ��ƪ� (VPN �ϥΪ̺޲z)

```sql
-- Create a table for VPN users
CREATE TABLE public.users (
  id UUID NOT NULL DEFAULT gen_random_uuid(),
  username TEXT NOT NULL,
  password_hash TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  CONSTRAINT users_pkey PRIMARY KEY (id),
  CONSTRAINT users_username_key UNIQUE (username)
);

-- Add index for faster lookups
CREATE INDEX users_username_idx ON public.users (username);
CREATE INDEX users_is_active_idx ON public.users (is_active);

-- �ҥ� RLS
ALTER TABLE users ENABLE ROW LEVEL SECURITY;

-- ���\�Ҧ��HŪ�g (�}�o���q)
-- �`�N�G�Ͳ��������ӭ���u���޲z���i�H�ק�
CREATE POLICY "Allow public all access on users" ON users
  FOR ALL USING (true) WITH CHECK (true);
```
