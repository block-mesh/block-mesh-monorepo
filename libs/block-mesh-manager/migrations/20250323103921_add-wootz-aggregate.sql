CREATE TABLE aggregates_wootz PARTITION OF aggregates FOR VALUES IN ('Wootz');
CREATE TABLE aggregates_interactive_ext PARTITION OF aggregates FOR VALUES IN ('InteractiveExt');