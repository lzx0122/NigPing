-- Create a table for admin users
create table public.admin_users (
  id uuid not null default gen_random_uuid (),
  username text not null,
  password_hash text not null,
  created_at timestamp with time zone not null default now(),
  constraint admin_users_pkey primary key (id),
  constraint admin_users_username_key unique (username)
);

-- Note: You should insert a user manually or via a script.
-- Example of inserting a user with a bcrypt hash.
-- The hash below is for 'password123' (generated with default salt rounds)
-- INSERT INTO public.admin_users (username, password_hash)
-- VALUES ('admin', '$2a$10$YourGeneratedHashHere');
