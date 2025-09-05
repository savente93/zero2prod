BEGIN;
UPDATE subscriptions
SET status = 'confirmed'
WHERE status IS NULL;
alter table subscriptions alter column status set NOT NULL;
commit;
