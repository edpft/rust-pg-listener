SELECT * FROM events;
INSERT INTO events (event_name) VALUES ('event 1');
UPDATE events SET event_name = 'new event name' WHERE event_id = 1;
DELETE FROM events WHERE event_id = 1;