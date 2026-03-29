---
name: nigping-project
description: NigPing monorepo layout, tech stack, and pnpm commands. Use for any task that needs correct paths or how to run/build each package.
---

# NigPing 專案脈絡

此 repo **沒有根目錄 `package.json`**；各子專案獨立，於各自目錄使用 **pnpm**。

## 目錄與職責

| 路徑 | 角色 | 技術 |
|------|------|------|
| `Tauri/` | 桌面客戶端（主 UI） | Vue 3、Vite、TypeScript、Pinia、Vitest、Tailwind、Tauri 2 |
| `Tauri/src-tauri/` | Tauri / Rust 後端 | Rust（Tauri、Windows 網路等） |
| `backend/` | API 服務 | Hono、Node、TypeScript、Supabase |
| `backend/admin-ui/` | 後台小面板 | Vue 3、Vite、TypeScript |
| `vps-agent/` | VPS 上的 WireGuard Agent | Hono、Node、TypeScript、Supabase |

## 常用指令（於各子目錄執行）

### `Tauri/`（桌面）

- 開發：`pnpm dev`（Vite）
- Tauri 桌面開發：`pnpm tauri dev`
- 建置前端：`pnpm build`
- 測試：`pnpm test`（Vitest）

### `backend/`

- 僅 API：`pnpm dev`
- API + admin 同時：`pnpm dev:all`
- 建置（含 admin 建置 + `tsc`）：`pnpm build`
- 僅 admin 開發：`pnpm admin:dev`（等同 `cd admin-ui && pnpm dev`）

### `backend/admin-ui/`

- 開發：`pnpm dev`
- 建置：`pnpm build`  
  （通常由 `backend` 的 `admin:build` 一併觸發。）

### `vps-agent/`

- 開發：`pnpm dev`
- 建置：`pnpm build`

## Agent 注意事項

- 前端 UI 以 **Vue 3** 為主，不是 React。
- 後端為 **Hono**，不是 Express；模式可類比一般 Node HTTP 服務，但 API 風格以 Hono 為準。
- 專案內 **未使用 ClickHouse**；分析型資料庫相關 skill 僅供參考，勿假設 repo 已整合。
