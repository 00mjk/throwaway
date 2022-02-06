CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- NOTE: This schema is created by Vault as well
CREATE SCHEMA IF NOT EXISTS throwaway;

CREATE TABLE IF NOT EXISTS throwaway.profile
(
  profile_id UUID        NOT NULL DEFAULT gen_random_uuid(),
  name       TEXT        NOT NULL,
  email      TEXT        NOT NULL,
  password   TEXT        NOT NULL,
  country    TEXT        NOT NULL,
  timezone   TEXT        NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  is_deleted BOOLEAN     NOT NULL DEFAULT FALSE,

  UNIQUE (email),

  PRIMARY KEY (profile_id)
);

-- DROP TABLE IF EXISTS throwaway.business CASCADE;
-- CREATE TABLE IF NOT EXISTS throwaway.business
-- (
--   business_id UUID        NOT NULL DEFAULT gen_random_uuid(),
--   profile_id  UUID        NOT NULL,
--   name        TEXT        NOT NULL,
--   country     TEXT        NOT NULL,
--   currency    TEXT        NOT NULL,
--   created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
--   updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
--   is_deleted  BOOLEAN     NOT NULL DEFAULT FALSE,
--
--   PRIMARY KEY (business_id),
--
--   FOREIGN KEY (profile_id) REFERENCES throwaway.profile (profile_id)
-- );

-- DROP TABLE IF EXISTS throwaway.account CASCADE;
-- CREATE TABLE IF NOT EXISTS throwaway.account
-- (
--   account_id     UUID NOT NULL DEFAULT gen_random_uuid(),
--   name           TEXT NOT NULL,
--   country        TEXT NOT NULL,
--   account_number TEXT NOT NULL,
--   sort_code      TEXT NOT NULL,
--
--   PRIMARY KEY (account_id)
-- );
--
-- DROP TABLE IF EXISTS throwaway.account_profile_junction;
-- CREATE TABLE IF NOT EXISTS throwaway.account_profile_junction
-- (
--   account_id UUID NOT NULL,
--   profile_id UUID NOT NULL,
--
--   PRIMARY KEY (account_id, profile_id),
--
--   FOREIGN KEY (account_id) REFERENCES throwaway.account (account_id),
--   FOREIGN KEY (profile_id) REFERENCES throwaway.profile (profile_id)
-- );
