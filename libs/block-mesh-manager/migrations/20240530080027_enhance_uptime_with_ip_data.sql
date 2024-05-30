ALTER TABLE uptime_reports ADD COLUMN ip TEXT;
ALTER TABLE uptime_reports ADD COLUMN latitude DOUBLE PRECISION;
ALTER TABLE uptime_reports ADD COLUMN longitude DOUBLE PRECISION;
ALTER TABLE uptime_reports ADD COLUMN country TEXT;
ALTER TABLE uptime_reports ADD COLUMN city TEXT;
ALTER TABLE uptime_reports ADD COLUMN region TEXT;
ALTER TABLE uptime_reports ADD COLUMN timezone TEXT;
ALTER TABLE uptime_reports ADD COLUMN isp TEXT;

CREATE INDEX uptime_reports_ip ON uptime_reports (ip);
CREATE INDEX uptime_reports_latitude ON uptime_reports (latitude);
CREATE INDEX uptime_reports_longitude ON uptime_reports (longitude);
CREATE INDEX uptime_reports_country ON uptime_reports (country);
CREATE INDEX uptime_reports_city ON uptime_reports (city);
CREATE INDEX uptime_reports_region ON uptime_reports (region);
CREATE INDEX uptime_reports_timezone ON uptime_reports (timezone);
CREATE INDEX uptime_reports_isp ON uptime_reports (isp);