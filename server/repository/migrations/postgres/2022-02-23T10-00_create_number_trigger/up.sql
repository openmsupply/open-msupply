CREATE TRIGGER number_trigger
  AFTER INSERT OR UPDATE OR DELETE ON number
  FOR EACH ROW EXECUTE PROCEDURE update_changelog();
