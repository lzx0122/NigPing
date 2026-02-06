-- Create a table for VPN users
create table public.users (
  id uuid not null default gen_random_uuid (),
  username text not null,
  password_hash text not null,
  created_at timestamp with time zone not null default now(),
  updated_at timestamp with time zone not null default now(),
  is_active boolean not null default true,
  constraint users_pkey primary key (id),
  constraint users_username_key unique (username)
);

-- Add index for faster lookups
create index users_username_idx on public.users (username);
create index users_is_active_idx on public.users (is_active);

-- Note: Run this SQL in your Supabase SQL Editor to create the users table
