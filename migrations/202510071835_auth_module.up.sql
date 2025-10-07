CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE SCHEMA IF NOT EXISTS "auth";

CREATE TABLE IF NOT EXISTS "auth"."user_profile"
(
    id         UUID PRIMARY KEY   DEFAULT gen_random_uuid(),
    name       TEXT      NOT NULL,
    email      TEXT      NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION "auth"."user_profile_updated_at_trigger"()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW."updated_at" = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER "auth-user_profile_updated_at_trigger"
    BEFORE UPDATE
    ON "auth"."user_profile"
    FOR EACH ROW
EXECUTE FUNCTION "auth"."user_profile_updated_at_trigger"();

CREATE TYPE "auth"."otp_reason" AS ENUM (
    'change_password',
    'change_email_address',
    'delete_account'
    );

CREATE TABLE IF NOT EXISTS "auth"."email_otp"
(
    id            BIGSERIAL PRIMARY KEY,
    email         VARCHAR(255)        NOT NULL,
    otp           VARCHAR(32)         NOT NULL,
    has_been_used BOOLEAN             NOT NULL DEFAULT FALSE,
    created_at    TIMESTAMP           NOT NULL DEFAULT NOW(),
    reason        "auth"."otp_reason" NOT NULL
);

CREATE INDEX IF NOT EXISTS "auth-email_otp_email_idx" ON "auth"."email_otp" ("email") WHERE "has_been_used" = FALSE;
CREATE INDEX IF NOT EXISTS "auth-email_otp_time_idx" ON "auth"."email_otp" ("created_at");

CREATE TABLE IF NOT EXISTS "auth"."email_account"
(
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email         varchar(255) NOT NULL UNIQUE,
    password_hash TEXT         NOT NULL,
    user_id       UUID         NOT NULL REFERENCES "auth"."user_profile" (id) ON DELETE CASCADE ON UPDATE CASCADE,
    banned_at     TIMESTAMP
);

