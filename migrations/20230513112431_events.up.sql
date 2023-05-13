CREATE TABLE IF NOT EXISTS events (
    event_id        int GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    event_name      text NOT NULL,
    event_datetime  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX events_id_index ON events (event_id);

CREATE OR REPLACE FUNCTION events_trigger ()
    RETURNS trigger
    AS $$
DECLARE
    payload TEXT;
BEGIN
    payload := json_build_object(
        'timestamp', CURRENT_TIMESTAMP,
        'operation', INITCAP(TG_OP),
        'schema', TG_TABLE_SCHEMA,
        'table', TG_TABLE_NAME,
        'old', COALESCE(row_to_json(OLD), NULL),
        'new', COALESCE(row_to_json(NEW), NULL)
    );
    
    PERFORM pg_notify('event', payload);
    RETURN NEW;
END;
$$
LANGUAGE 'plpgsql'
SECURITY DEFINER;

CREATE OR REPLACE TRIGGER events_trigger
    BEFORE INSERT OR UPDATE OR DELETE ON events
    FOR EACH ROW
    EXECUTE PROCEDURE events_trigger ();

