-- 1. Create VPN Profiles Table
CREATE TABLE IF NOT EXISTS public.vpn_profiles (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id UUID REFERENCES public.users(id) NOT NULL,
    server_ip TEXT NOT NULL, -- Changed from INET to TEXT to match servers table if needed, or keep INET if consistent
    device_name TEXT NOT NULL,
    public_key TEXT NOT NULL UNIQUE,
    allowed_ip INET NOT NULL UNIQUE,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now()) NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now()) NOT NULL
);

-- 2. Create Index for faster IP allocation checks
CREATE INDEX IF NOT EXISTS idx_vpn_allowed_ip ON public.vpn_profiles(allowed_ip);
CREATE INDEX IF NOT EXISTS idx_vpn_user_id ON public.vpn_profiles(user_id);

-- 3. Enable Realtime for vpn_profiles (Important for VPS Agent)
ALTER PUBLICATION supabase_realtime ADD TABLE vpn_profiles;

-- 4. Enable RLS
ALTER TABLE public.vpn_profiles ENABLE ROW LEVEL SECURITY;

-- 5. RLS Policies
-- Allow users to view their own profiles
CREATE POLICY "Users can view their own profiles" 
ON public.vpn_profiles FOR SELECT 
USING (auth.uid() = user_id);

-- Allow admins (service_role) full access
-- Note: service_role bypasses RLS by default, but good to be explicit if needed.

-- 6. RPC Function for Atomic IP Allocation
CREATE OR REPLACE FUNCTION allocate_vpn_ip(
    p_user_id UUID,
    p_server_ip TEXT,
    p_device_name TEXT,
    p_public_key TEXT
)
RETURNS JSONB
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
    new_ip INET;
    octet INT;
    existing_octets INT[];
BEGIN
    -- 1. Check if public key already exists
    IF EXISTS (SELECT 1 FROM vpn_profiles WHERE public_key = p_public_key) THEN
        RETURN jsonb_build_object('success', false, 'error', 'Public key already registered');
    END IF;

    -- 2. Get all used last octets for the 10.0.0.x range
    SELECT ARRAY_AGG((host(allowed_ip)::inet - '10.0.0.0'::inet))
    INTO existing_octets
    FROM vpn_profiles
    WHERE is_active = true 
      AND family(allowed_ip) = 4 
      AND allowed_ip << '10.0.0.0/24';

    -- 3. Find the first available octet starting from 2 (1 is gateway)
    -- We can use generate_series and EXCEPT to find the first missing one
    SELECT num INTO octet
    FROM generate_series(2, 254) AS num
    WHERE NOT (num = ANY(COALESCE(existing_octets, ARRAY[]::INT[])))
    ORDER BY num ASC
    LIMIT 1;

    IF octet IS NULL THEN
        RETURN jsonb_build_object('success', false, 'error', 'No IP addresses available in range 10.0.0.0/24');
    END IF;

    new_ip := ('10.0.0.' || octet)::inet;

    -- 4. Insert the new profile
    INSERT INTO vpn_profiles (user_id, server_ip, device_name, public_key, allowed_ip)
    VALUES (p_user_id, p_server_ip, p_device_name, p_public_key, new_ip);

    -- 5. Return the result
    RETURN jsonb_build_object(
        'success', true,
        'assigned_ip', new_ip,
        'server_endpoint', p_server_ip || ':51820',
        'server_public_key', 'SERVER_PUB_KEY_PLACEHOLDER', 
        'allowed_ips', '0.0.0.0/0' 
    );
EXCEPTION WHEN OTHERS THEN
    RETURN jsonb_build_object('success', false, 'error', SQLERRM);
END;
$$;
