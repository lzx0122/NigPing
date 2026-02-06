export interface User {
  id: string;
  username: string;
  created_at: string;
  updated_at: string;
  is_active: boolean;
}

export interface CreateUserRequest {
  username: string;
  password?: string;
  autoGenerate?: boolean;
}

export interface CreateUserResponse {
  user: User;
  password?: string; // Only returned if auto-generated
}

export interface ResetPasswordRequest {
  password?: string;
  autoGenerate?: boolean;
}

export interface ResetPasswordResponse {
  user: User;
  password?: string; // Only returned if auto-generated
}
