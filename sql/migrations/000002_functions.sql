/**
 * Returns a time-ordered UUID (v6).
 *
 * The multicast bit is set automatically in the node identifier.
 */
CREATE OR REPLACE FUNCTION gen_random_uuid_v6() RETURNS UUID AS $$
DECLARE
  v_time TIMESTAMP WITH TIME ZONE := null;
  v_secs BIGINT := null;
  v_usec BIGINT := null;
  v_timestamp BIGINT := null;
  v_timestamp_hex VARCHAR := null;
  v_bytes BYTEA;

  c_greg bigint :=  -12219292800; -- Gregorian epoch: '1582-10-15 00:00:00'
BEGIN
  -- Get time and random values
  v_time := clock_timestamp();

  -- Extract seconds and microseconds
  v_secs := EXTRACT(EPOCH FROM v_time);
  v_usec := mod(EXTRACT(MICROSECONDS FROM v_time)::NUMERIC, 10^6::NUMERIC);

  -- Calculate the timestamp
  v_timestamp := (((v_secs - c_greg) * 10^6) + v_usec) * 10;

  -- Generate timestamp hexadecimal (and set version number: 6)
  v_timestamp_hex := lpad(to_hex(v_timestamp), 16, '0');
  v_timestamp_hex := substr(v_timestamp_hex, 2, 12) || '6' || substr(v_timestamp_hex, 14, 3);

  -- Concat timestamp hex with random hex to generate a byte array
  v_bytes := decode(substr(v_timestamp_hex || md5(random()::TEXT), 1, 32), 'hex');

  -- Set variant bits (10xx)
  v_bytes := set_bit(v_bytes, 71, 1);
  v_bytes := set_bit(v_bytes, 70, 0);

  RETURN encode(v_bytes, 'hex')::UUID;
END $$ LANGUAGE plpgsql;
