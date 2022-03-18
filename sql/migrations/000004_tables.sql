CREATE TABLE IF NOT EXISTS throwaway.profile
(
  profile_id UUID        NOT NULL DEFAULT gen_random_uuid_v6(),
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
