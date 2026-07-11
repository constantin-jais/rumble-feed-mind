-- Local development principals only. Production credentials are provisioned by
-- the platform and must never be committed. This file runs only on a fresh
-- docker-compose PostgreSQL data volume, after provision-roles.sql.

CREATE ROLE feed_radar_migrator_dev LOGIN NOINHERIT PASSWORD 'feed_radar_migrator_dev';
CREATE ROLE feed_radar_app_dev LOGIN NOINHERIT PASSWORD 'feed_radar_app_dev';
CREATE ROLE feed_radar_auth_dev LOGIN NOINHERIT PASSWORD 'feed_radar_auth_dev';
CREATE ROLE feed_radar_worker_dev LOGIN NOINHERIT PASSWORD 'feed_radar_worker_dev';
CREATE ROLE feed_radar_readonly_dev LOGIN NOINHERIT PASSWORD 'feed_radar_readonly_dev';

GRANT feed_radar_owner TO feed_radar_migrator_dev;
GRANT feed_radar_app TO feed_radar_app_dev;
GRANT feed_radar_auth TO feed_radar_auth_dev;
GRANT feed_radar_worker TO feed_radar_worker_dev;
GRANT feed_radar_readonly TO feed_radar_readonly_dev;

SELECT format(
    'GRANT CONNECT ON DATABASE %I TO feed_radar_migrator_dev, feed_radar_app_dev, feed_radar_auth_dev, feed_radar_worker_dev, feed_radar_readonly_dev',
    current_database()
)\gexec
